#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Docker error: {0}")]
    DockerError(#[from] bollard::errors::Error),
    #[error("unknown error")]
    Unknown,
}
