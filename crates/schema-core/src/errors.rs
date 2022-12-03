use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Reqwest Error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Bollard Error: {0}")]
    Bollard(#[from] bollard::errors::Error),
    #[error("Semver Error: {0}")]
    Semver(#[from] semver::Error),
    #[error("release not found")]
    ReleaseNotFound,
    #[error("release not supported, please use a release >= 1.45.0")]
    ReleaseNotSupported,
    #[error("unknown error")]
    Unknown,
}
