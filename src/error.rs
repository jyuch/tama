pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

#[derive(Debug)]
pub enum Response {
    Ok(String),
    Fail(String),
}
