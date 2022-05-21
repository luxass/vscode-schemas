use crate::octoduck::models;
use super::*;

pub struct ReleasesHandler<'octo, 'r> {
    parent: &'r RepoHandler<'octo>,
}

impl<'octo, 'r> ReleasesHandler<'octo, 'r> {
    pub(crate) fn new(parent: &'r RepoHandler<'octo>) -> Self {
        Self { parent }
    }

    pub fn list(&self) -> ListReleasesBuilder<'_, '_, '_> {
        ListReleasesBuilder::new(self)
    }
    
    
    pub async fn get_last_release(&self) -> Result<release::Release> {
        let url = format!(
            "repos/{owner}/{repo}/releases/latest",
            owner = self.parent.owner,
            repo = self.parent.repo,
        );
        self.parent.duck.get(url, None::<&()>).await
    }
}

#[derive(Serialize)]
pub struct ListReleasesBuilder<'octo, 'r1, 'r2> {
    #[serde(skip)]
    handler: &'r2 ReleasesHandler<'octo, 'r1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r1, 'r2> ListReleasesBuilder<'octo, 'r1, 'r2> {
    pub(crate) fn new(handler: &'r2 ReleasesHandler<'octo, 'r1>) -> Self {
        Self {
            handler,
            per_page: None,
            page: None,
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

    pub async fn send(self) -> Result<crate::octoduck::Pagination<release::Release>> {
        let url = format!(
            "repos/{owner}/{repo}/releases",
            owner = self.handler.parent.owner,
            repo = self.handler.parent.repo
        );
        self.handler.parent.duck.get(url, Some(&self)).await
    }
}
