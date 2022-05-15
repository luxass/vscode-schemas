use crate::octoduck::models;
use super::*;


#[derive(Serialize)]
pub struct CompareBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    base: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    head: Option<String>,
}

impl<'octo, 'r> CompareBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self {
            handler,
            per_page: None,
            page: None,
            base: Some("main".to_string()),
            head: Some("main".to_string()),
        }
    }

    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }

    pub fn base(mut self, base: impl Into<String>) -> Self {
        self.base = Some(base.into());
        self
    }

    pub fn head(mut self, head: impl Into<String>) -> Self {
        self.head = Some(head.into());
        self
    }

    pub async fn send(self) -> Result<crate::octoduck::Pagination<compare::RootCommit>> {
        let url = format!(
            "repos/{owner}/{repo}/compare/{base}...{head}",
            owner = self.handler.owner,
            repo = self.handler.repo,
            base = self.base.as_ref().unwrap(),
            head = self.head.as_ref().unwrap(),
        );
        self.handler.duck.get(url, Some(&self)).await
    }
}
