extern crate log;

#[macro_use]
extern crate serde;

pub mod commands;
pub mod docker;
pub mod scanner;

use log::debug;
use regex::Regex;
use std::env;
use std::fs::{metadata, File};
use std::io::{Read, Write};
use std::path::Path;
use std::process::{Child, Command};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Metadata {
    pub version: String,
    // Schemas accessible via vscode://schemas/
    pub schemas: Vec<String>,
    // Schemas that are located inside folders, but not accessible via vscode://schemas/
    pub schema_urls: Vec<String>,
}

pub async fn read_metadata(metadata_url: String) -> Result<Metadata, Box<dyn std::error::Error>> {
    let url_re = Regex::new(r"https?")?;

    let metadata = if url_re.is_match(&metadata_url) {
        debug!("Fetching file");
        let res = reqwest::get(metadata_url).await?;
        res.text().await?
    } else {
        debug!("Reading file");

        let mut file = File::open(metadata_url)?;
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)?;
        file_contents
    };
    Ok(serde_json::from_str::<Metadata>(&metadata)?)
}

pub fn write_metadata(
    metadata: Metadata,
    metadata_path: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let contents = serde_json::to_string_pretty(&metadata)?;
    let mut file = File::create(Path::new(&metadata_path))?;
    file.write_all(contents.as_bytes())?;
    Ok(())
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

pub fn is_ci() -> bool {
    env::var("GITHUB_ACTIONS").is_ok() || env::var("CI").is_ok()
}

pub fn write_readme() {
    let github_actions = env::var("GITHUB_ACTIONS").expect("GITHUB_ACTIONS not set");

    let schemas_path = if github_actions == "true" {
        Path::new("schemas")
    } else {
        Path::new("../schemas")
    };

    // let file = File::create(schemas_dir.join("README.md")).unwrap();
    // let mut md = Markdown::new(file);

    // md.write("Visual Studio Code Schemas".heading(1)).unwrap();
    // md.write("This is a collection of schemas for Visual Studio Code.".quote())
    //     .unwrap();

    // md.write("Versions".heading(2)).unwrap();
}
