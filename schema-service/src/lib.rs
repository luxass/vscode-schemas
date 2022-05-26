#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

use std::fs::{metadata, File};
use std::io::{Read, Write};
use walkdir::WalkDir;

pub mod docker;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SchemaList {
    pub last_release: String,
    pub schemas: Vec<String>,
}

pub fn read_schema_list() -> SchemaList {
    let mut file = File::open("../schema-list.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let schema_list: SchemaList = serde_json::from_str(&contents).unwrap();
    schema_list
}

pub fn write_schema_list(schema_list: SchemaList) {
    let contents = serde_json::to_string_pretty(&schema_list).unwrap();
    let mut file = File::create("../schema-list.json").unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}

pub fn scan_for_ts_files(dir: &str) -> Result<Vec<String>, std::io::Error> {
    let mut files: Vec<String> = Vec::new();
    for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();

        let metadata = metadata(&path)?;
        if metadata.is_file() {
            if path.extension().unwrap() == "ts" {
                files.push(path.to_str().unwrap().to_owned())
            }
        }
    }

    Ok(files)
}
