use reqwest::StatusCode;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

#[derive(Debug)]
pub enum Response {
    Ok(Option<String>),
    Fail(Option<String>),
}

#[derive(Debug, Error)]
pub enum OperationError {
    #[error("Deployment type mismatch.")]
    DeploymentTypeMismatch,

    #[error("{0}")]
    HttpStatusNotSuccess(StatusCode),
}
