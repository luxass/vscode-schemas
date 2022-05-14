mod pagination;
mod api;
pub mod models;
mod from_response;

use secrecy::{ExposeSecret, SecretString};
use serde::Serialize;
use snafu::*;
use reqwest::{StatusCode, Url};

use crate::{
    error,
    Result,
};

pub use self::{
    api::{
        repos,
    },
    from_response::FromResponse,
    pagination::Pagination,
};


const GITHUB_BASE_URL: &str = "https://api.github.com";


pub async fn map_github_error(response: reqwest::Response) -> Result<reqwest::Response> {
    if response.status().is_success() {
        Ok(response)
    } else {
        Err(error::Error::GitHub {
            source: response
                .json::<error::GitHubError>()
                .await
                .context(error::HttpSnafu)?,
            backtrace: Backtrace::generate(),
        })
    }
}

enum Auth {
    None,
    PersonalToken(SecretString),
}

impl Default for Auth {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone)]
enum AuthState {
    None,
}

#[derive(Default)]
pub struct OctoduckBuilder {
    auth: Auth,
}

impl OctoduckBuilder {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn personal_token(mut self, token: String) -> Self {
        self.auth = Auth::PersonalToken(SecretString::new(token));
        self
    }

    pub fn build(self) -> Result<Octoduck> {
        let mut hmap = reqwest::header::HeaderMap::new();


         let auth_state = match self.auth {
            Auth::None => AuthState::None,
            Auth::PersonalToken(token) => {
                hmap.append(
                    reqwest::header::AUTHORIZATION,
                    (String::from("Bearer ") + token.expose_secret())
                        .parse()
                        .unwrap(),
                );
                AuthState::None
            }
        };



        let client = reqwest::Client::builder()
            .user_agent("deprecatedluxas/vscode-schemas")
            .default_headers(hmap)
            .build()
            .context(crate::error::HttpSnafu)?;

        Ok(Octoduck {
            client,
            base_url: Url::parse(GITHUB_BASE_URL).unwrap(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Octoduck {
    client: reqwest::Client,
    pub base_url: Url,
}


impl Octoduck {
    pub fn builder() -> OctoduckBuilder {
        OctoduckBuilder::default()
    }

    pub fn repos(&self, owner: impl Into<String>, repo: impl Into<String>) -> repos::RepoHandler {
        repos::RepoHandler::new(self, owner.into(), repo.into())
    }

    pub async fn get<R, A, P>(&self, route: A, parameters: Option<&P>) -> Result<R>
        where
            A: AsRef<str>,
            P: Serialize + ?Sized,
            R: FromResponse,
    {
        let response = self._get(self.absolute_url(route)?, parameters).await?;
        R::from_response(map_github_error(response).await?).await
    }

    pub async fn _get<P: Serialize + ?Sized>(
        &self,
        url: impl reqwest::IntoUrl,
        parameters: Option<&P>,
    ) -> Result<reqwest::Response> {
        let mut request = self.client.get(url);

        if let Some(parameters) = parameters {
            request = request.query(parameters);
        }
        self.execute(request).await
    }

    pub async fn execute(&self, mut request: reqwest::RequestBuilder) -> Result<reqwest::Response> {
        let mut retries = 0;
        loop {
            // Saved request that we can retry later if necessary
            let retry_request = None;

            let result = request.send().await;
            if let Err(ref e) = result {
                if let Some(StatusCode::UNAUTHORIZED) = e.status() {
                    if let Some(retry) = retry_request {
                        if retries < 3 {
                            retries += 1;
                            request = retry;
                            continue;
                        }
                    }
                }
            }
            return result.context(error::HttpSnafu);
        }
    }

    pub fn absolute_url(&self, url: impl AsRef<str>) -> Result<Url> {
        self.base_url
            .join(url.as_ref())
            .context(error::UrlSnafu)
    }

    pub async fn get_page<R: serde::de::DeserializeOwned>(
        &self,
        url: &Option<Url>,
    ) -> Result<Option<Pagination<R>>> {
        error!("GET_PAGE URL: {:?}", url);
        match url {
            Some(url) => self.get(url, None::<&()>).await.map(Some),
            None => Ok(None),
        }
    }
}
