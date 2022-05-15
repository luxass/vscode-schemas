#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

use std::fs::File;
use std::io::{Read, Write};
// use octocrab::models::repos::Release;
// use octocrab::{Octocrab, Page};
// use octocrab::repos::RepoHandler;
use url::Url;

pub mod docker;
pub mod releases;
pub mod repo;
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
    pub schemas: Vec<String>,
    pub versions_compared: VersionsCompared
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


pub fn read_schema_list() -> SchemaList {

    let mut file = File::open("../schema-list.toml").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let schema_list: SchemaList = toml::from_str(&contents).unwrap();
    schema_list
}

pub fn write_schema_list(schema_list: SchemaList) {
    let contents = toml::to_string_pretty(&schema_list).unwrap();
    let mut file = File::create("../schema-list.toml").unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}

