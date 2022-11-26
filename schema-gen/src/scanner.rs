use lazy_static::lazy_static;
use log::{debug, trace};
use regex::Regex;
use serde_json::Value;
use std::{fs::{self, metadata}, path::{Path, PathBuf}};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
enum FileExt {
    Json,
    JsonC,
    TS,
    JS,
}

#[derive(Debug, Clone)]
struct ScannedFile {
    pub name: String,
    pub path: String,
    pub clean_path: String,
    pub ext: FileExt,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PackageJson {
    pub contributes: Option<Contributes>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Contributes {
    #[serde(rename = "jsonValidation")]
    pub json_validation: Option<Vec<JsonValidation>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct JsonValidation {
    #[serde(rename = "fileMatch")]
    pub file_match: Value,
    pub url: String,
}

lazy_static! {
    static ref SCHEMA_REGEX: Regex = Regex::new(r#"vscode://schemas/([^"]+)"#).unwrap();
}

#[derive(Debug)]
pub struct Scanner {
    files: Vec<ScannedFile>,
    sha: String,
    release: String,
    pub schemas: Vec<String>,
    pub schema_urls: Vec<String>,
}

impl Scanner {
    pub fn new(sha: String, release: String) -> Self {
        Self {
            files: Vec::new(),
            schemas: Vec::new(),
            schema_urls: Vec::new(),
            sha,
            release,
        }
    }

    pub fn scan_files(&mut self, dir: &str) -> Result<(), std::io::Error> {
        for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
            let path = entry.path();

            let metadata = metadata(&path)?;
            if metadata.is_file() {
                match path.extension() {
                    Some(ext) => {
                        if ext == "json" || ext == "jsonc" || ext == "ts" || ext == "js" {
                            let clean_path = path
                                .to_str()
                                .unwrap()
                                .replace(dir, "")
                                .replace(&format!("microsoft-vscode-{sha}", sha = self.sha), "");
                            self.files.push(ScannedFile {
                                name: path.file_name().unwrap().to_str().unwrap().to_string(),
                                path: path.to_str().unwrap().to_string(),
                                clean_path,
                                ext: match ext.to_str().unwrap() {
                                    "json" => FileExt::Json,
                                    "jsonc" => FileExt::JsonC,
                                    "ts" => FileExt::TS,
                                    "js" => FileExt::JS,
                                    _ => panic!("Unknown file extension"),
                                },
                            });
                        }
                    }
                    None => {}
                }
            }
        }
        Ok(())
    }

    pub fn parse_files(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for file in &self.files {
            trace!("Parsing file: {:?}", file);
            match file.ext {
                FileExt::JsonC | FileExt::Json => {
                    let contents = fs::read_to_string(&file.path)?;
                    if file.name == "package.json" {
                        let package_json: PackageJson =
                            serde_json::from_str::<PackageJson>(&contents)
                                .expect("failed to parse package.json");

                        if let Some(contributes) = package_json.contributes {
                            debug!("{:?}", contributes);
                            if let Some(json_validations) = contributes.json_validation {
                                for json_validation in json_validations {
                                    if let Some(captures) =
                                    SCHEMA_REGEX.captures(&json_validation.url)
                                    {
                                        let schema =
                                            captures.get(0).map_or("", |m| m.as_str()).to_string();
                                        if !self.schemas.contains(&schema) {
                                            debug!("{:?} - {}", schema, &file.name);
                                            self.schemas.push(schema);
                                        }
                                    } else {
                                        
                                        let url_re = Regex::new(r"https?")?;

                                        if !url_re.is_match(&json_validation.url) {
                                            let path = Path::new(&file.clean_path)
                                                .parent().unwrap()
                                                .join(&json_validation.url.strip_prefix("./").unwrap());

                                            let schema = format!(
                                                "https://raw.githubusercontent.com/microsoft/vscode/{release}{clean_path}", 
                                                release = self.release, 
                                                clean_path = path.to_str().unwrap()
                                            );
                                            if !self.schema_urls.contains(&schema) {
                                                debug!("{:?} - {}", schema, &file.name);
                                                self.schema_urls.push(schema);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        let lines = contents.lines();

                        lines.for_each(|line| {
                            if let Some(captures) = SCHEMA_REGEX.captures(line) {
                                let schema = captures.get(0).map_or("", |m| m.as_str()).to_string();
                                if !self.schemas.contains(&schema) {
                                    debug!("{:?} - {}", schema, &file.name);
                                    self.schemas.push(schema);
                                }
                            }
                        })
                    }
                }
                FileExt::TS | FileExt::JS => {
                    let contents = fs::read_to_string(&file.path).expect("failed to read file");
                    let lines = contents.lines();

                    lines.for_each(|line| {
                        if let Some(captures) = SCHEMA_REGEX.captures(line) {
                            let schema = captures.get(0).map_or("", |m| m.as_str()).to_string();
                            if !self.schemas.contains(&schema) {
                                debug!("{:?} - {}", schema, &file.name);
                                self.schemas.push(schema);
                            }
                        }
                    })
                }
            }
        }

        Ok(())
    }
}
