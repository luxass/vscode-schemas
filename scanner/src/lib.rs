#[macro_use]
extern crate log;

#[macro_use]
extern crate serde;

use std::{fs::File, io::Read};

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