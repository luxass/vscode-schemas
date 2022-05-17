pub mod releases;
pub mod compares;

use crate::{
    octoduck::{
        Octoduck, Result,
        models::{
            repos,
            release,
            compare
        }
    }
};

pub use releases::ReleasesHandler;
pub use compares::CompareHandler;
use crate::octoduck::models::repos::LatestCommit;

pub struct RepoHandler<'octo> {
    duck: &'octo Octoduck,
    owner: String,
    repo: String,
}

impl<'octo> RepoHandler<'octo> {
    pub(crate) fn new(duck: &'octo Octoduck, owner: String, repo: String) -> Self {
        Self { duck, owner, repo }
    }

    pub async fn get(&self) -> Result<repos::Repository> {
        let url = format!("repos/{owner}/{repo}", owner = self.owner, repo = self.repo,);
        self.duck.get(url, None::<&()>).await
    }

    pub fn releases(&self) -> ReleasesHandler<'_, '_> {
      ReleasesHandler::new(self)
    }

    pub fn compare(&self, base: String, head: String) -> CompareHandler<'_, '_> {
      CompareHandler::new(self, base, head)
    }

    pub async fn get_latest_commit_sha(&self) -> Result<String> {
        let repo = self.get().await?;

        let url = format!("repos/{owner}/{repo}/commits/{default_branch}", owner = self.owner, repo = self.repo, default_branch = repo.default_branch,);
        let commit = self.duck.get(url, None::<&()>).await?;
        commit.sha

    }
}
