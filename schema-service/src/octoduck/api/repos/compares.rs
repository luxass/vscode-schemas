use super::*;
use crate::octoduck::models;

pub struct CompareHandler<'octo, 'r> {
    parent: &'r RepoHandler<'octo>,
    base: String,
    head: String,
}

impl<'octo, 'r> CompareHandler<'octo, 'r> {
    pub(crate) fn new(parent: &'r RepoHandler<'octo>, base: String, head: String) -> Self {
        Self { parent, base, head }
    }

    pub async fn get(&self) -> Result<compare::CompareRoot> {
        let url = format!(
            "repos/{owner}/{repo}/compare/{base}...{head}",
            owner = self.parent.owner,
            repo = self.parent.repo,
            base = self.base,
            head = self.head
        );
        self.parent.duck.get(url, None::<&()>).await
    }

    pub fn list_commits(&self) -> ListCompareCommitsBuilder<'_, '_, '_> {
        ListCompareCommitsBuilder::new(self, self.base.clone(), self.head.clone())
    }
}

#[derive(Serialize)]
pub struct ListCompareCommitsBuilder<'octo, 'r1, 'r2> {
    #[serde(skip)]
    handler: &'r2 CompareHandler<'octo, 'r1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(skip)]
    base: String,
    #[serde(skip)]
    head: String,
}

impl<'octo, 'r1, 'r2> ListCompareCommitsBuilder<'octo, 'r1, 'r2> {
    pub(crate) fn new(handler: &'r2 CompareHandler<'octo, 'r1>, base: String, head: String) -> Self {
        Self {
            handler,
            per_page: None,
            page: None,
            base,
            head,
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

    pub async fn send(self) -> Result<crate::octoduck::Pagination<compare::CompareCommit>> {
        let url = format!(
            "repos/{owner}/{repo}/compare/{base}...{head}",
            owner = self.handler.parent.owner,
            repo = self.handler.parent.repo,
            base = self.base,
            head = self.head
        );
        self.handler.parent.duck.get(url, Some(&self)).await
    }
}
