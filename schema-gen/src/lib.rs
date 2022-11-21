extern crate log;

#[macro_use]
extern crate serde;

pub mod docker;

use std::fs::{metadata, File};
use std::io::Read;
use std::process::{Child, Command};
use walkdir::WalkDir;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Metadata {
    pub version: String,
    pub schemas: Vec<String>,
}

pub fn read_metadata() -> Metadata {
    let mut file = File::open("../metadata.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    serde_json::from_str::<Metadata>(&contents).unwrap()
}

pub fn scan_for_files(dir: &str) -> Result<Vec<String>, std::io::Error> {
    let mut files: Vec<String> = Vec::new();
    for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();

        let metadata = metadata(&path)?;
        if metadata.is_file() {
            match path.extension() {
                Some(ext) => {
                    if ext == "json" || ext == "jsonc" || ext == "ts" || ext == "js" {
                        files.push(path.to_str().unwrap().to_string());
                    }
                }
                None => {}
            }
        }
    }

    Ok(files)
}

pub fn set_default_env(key: &str, value: &str) {
    if std::env::var(key).is_err() {
        std::env::set_var(key, value);
    }
}

pub fn run_driver() -> Child {
    Command::new(String::from("chromedriver"))
        .arg("--port=9515")
        .spawn()
        .expect("failed to run chromedriver")
}
