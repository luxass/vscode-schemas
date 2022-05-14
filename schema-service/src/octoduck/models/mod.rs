pub mod repos;
pub mod release;
pub mod compare;
// use reqwest::Url;
//
//
// #[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
// pub struct Permissions {
//     #[serde(default)]
//     pub admin: bool,
//     pub push: bool,
//     pub pull: bool,
//     #[serde(default)]
//     pub triage: bool,
//     #[serde(default)]
//     pub maintain: bool,
// }
//
// #[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
// pub struct License {
//     pub key: String,
//     pub name: String,
//     pub node_id: String,
//     pub spdx_id: String,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub url: Option<Url>,
//     pub html_url: Option<Url>,
//     pub description: Option<String>,
//     pub implementation: Option<String>,
//     pub permissions: Option<Vec<String>>,
//     pub conditions: Option<Vec<String>>,
//     pub limitations: Option<Vec<String>>,
//     pub body: Option<String>,
//     pub featured: Option<bool>,
// }
