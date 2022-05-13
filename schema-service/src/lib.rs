#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

pub mod markdown;

#[derive(Debug, Deserialize)]
pub struct SchemaList {
    pub versions_compared: VersionsCompared,
    pub schemas: Vec<String>
}

#[derive(Debug, Deserialize)]
pub struct VersionsCompared {
    pub base: String,
    pub head: String,
}
