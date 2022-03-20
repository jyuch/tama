use thiserror::Error;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

#[derive(Debug)]
pub enum Response {
    Ok(Option<String>),
    Fail(Option<String>),
}

#[derive(Debug, Error)]
pub enum ParallelError {
    #[error("Deployment type mismatch.")]
    Mismatch,
}
