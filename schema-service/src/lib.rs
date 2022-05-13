#[macro_use]
extern crate async_trait;

#[macro_use]
extern crate log;

pub mod markdown;


pub struct SchemaList {
    pub versions_compared: VersionsCompared,
    pub schemas: Vec<String>
}

struct VersionsCompared {
    pub base: String,
    pub head: String,
}
