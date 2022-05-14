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
use crate::octoduck::repos::compares::CompareBuilder;

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

    pub fn compare(&self) -> CompareBuilder<'_, '_> {
      CompareBuilder::new(self)
    }
}
