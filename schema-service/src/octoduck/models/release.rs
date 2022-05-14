use chrono::{DateTime, Utc};
use reqwest::Url;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Release {
    pub url: Url,
    pub html_url: Url,
    pub assets_url: Url,
    pub upload_url: String,
    pub tarball_url: Option<Url>,
    pub zipball_url: Option<Url>,
    pub id: u64,
    pub node_id: String,
    pub tag_name: String,
    pub target_commitish: String,
    pub name: Option<String>,
    pub body: Option<String>,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub published_at: Option<DateTime<Utc>>,
    // pub author: crate::models::User,
    pub assets: Vec<Asset>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Asset {
    pub url: Url,
    pub browser_download_url: Url,
    pub id: u64,
    pub node_id: String,
    pub name: String,
    pub label: Option<String>,
    pub state: String,
    pub content_type: String,
    pub size: i64,
    pub download_count: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // pub uploader: User,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LastTwoReleases(pub Release, pub Release);

impl LastTwoReleases {
    pub fn names(&self) -> (&str, &str) {
        (&self.0.tag_name, &self.1.tag_name)
    }
}
