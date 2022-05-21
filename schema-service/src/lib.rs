#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

use std::fs::{File, metadata};
use std::io::{Read, Write};
use walkdir::WalkDir;
// use octocrab::models::repos::Release;
// use octocrab::{Octocrab, Page};
// use octocrab::repos::RepoHandler;
use url::Url;

pub mod docker;
pub mod error;
pub mod octoduck;

pub type Result<T, E = error::Error> = std::result::Result<T, E>;



//
//
// #[derive(Debug)]
// pub struct LastTwoReleases(pub Release, pub Release);
//
// impl LastTwoReleases {
//     pub fn names(&self) -> (&str, &str) {
//         (&self.0.tag_name, &self.1.tag_name)
//     }
// }

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SchemaList {
    pub last_release: String,
    pub schemas: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VersionsCompared {
    pub base: String,
    pub head: String,
}


#[derive(Debug, Deserialize)]
pub struct CompareRoot {
    pub url: Url,
    pub html_url: Url,
    pub permalink_url: Url,
    pub diff_url: Url,
    pub patch_url: Url,
    //pub base_commit: String, // todo
    //pub merge_base_commit: String, // todo
    pub status: String,
    pub ahead_by: i64,
    pub behind_by: i64,
    pub total_commits: i64,
    //pub commits: Vec<String>, // todo
    pub files: Vec<CompareFile>,
}

#[derive(Debug, Deserialize)]
pub struct CompareFile {
    pub filename: String,
}


// pub fn read_schema_list() -> SchemaList {
//
//     let mut file = File::open("../schema-list.toml").unwrap();
//     let mut contents = String::new();
//     file.read_to_string(&mut contents).unwrap();
//
//     let schema_list: SchemaList = toml::from_str(&contents).unwrap();
//     schema_list
// }

pub fn write_schema_list(schema_list: SchemaList) {
    let contents = serde_json::to_string_pretty(&schema_list).unwrap();
    let mut file = File::create("../schema-list.json").unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}

pub fn parse_folder_name(sha: &str) -> String {
    return "microsoft-vscode-".to_owned() + &sha
}

pub fn clean_up_src_folder(folder_name: &str) {
    let path = std::path::Path::new(folder_name);
    if path.exists() {
        std::fs::remove_dir_all(folder_name).unwrap();
    }
}


pub fn scan_for_ts_files(dir: &str) -> Result<Vec<String>, std::io::Error> {
    let mut files: Vec<String> = Vec::new();
    for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();

        let metadata = metadata(&path)?;
        if metadata.is_file() {
            if path.extension().unwrap() == "ts" {
                // if path.to_str().unwrap().to_owned() != "../extraction\\microsoft-vscode-3649387\\src\\vs\\workbench\\services\\configuration\\common\\configuration.ts" {
                //      continue;
                // }
                files.push(path.to_str().unwrap().to_owned())
                // debug!("{}", path.display());
            }
        }
        // if last_modified > 0 && metadata.is_file() {
        //     println!(
        //         "Last modified: {:?} seconds, is read only: {:?}, size: {:?} bytes, filename: {:?}, full-path: {:?}",
        //         last_modified,
        //         metadata.permissions().readonly(),
        //         metadata.len(),
        //         path.file_name().ok_or("No filename").unwrap(),
        //         path.to_str().ok_or("No path").unwrap()
        //     );
        // }
    }

    Ok(files)
}
