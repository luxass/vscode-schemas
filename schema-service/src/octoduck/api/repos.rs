pub mod compares;
pub mod releases;

use crate::octoduck::{
    models::{compare, release, repos},
    Octoduck, Result,
};

use crate::octoduck::models::repos::LatestCommit;
pub use compares::CompareHandler;
pub use releases::ReleasesHandler;

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

        let default_branch = repo.default_branch.unwrap_or("main".to_string());

        let url = format!(
            "repos/{owner}/{repo}/commits/{default_branch}",
            owner = self.owner,
            repo = self.repo,
            default_branch = default_branch
        );
        let commit: LatestCommit = self.duck.get(url, None::<&()>).await?;
        Ok(commit.sha)
    }
}
