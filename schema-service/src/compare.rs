use std::fs::File;
use std::io::{Read, Write};
use crate::{SchemaList};


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
