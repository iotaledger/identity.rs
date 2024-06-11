use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("The requested item \"{0}\" was not found.")]
    NotFound(String),
    #[error("Failed to parse the provided input into a resolvable type: {0}")]
    ParsingFailure(#[source] anyhow::Error),
    #[error(transparent)]
    Generic(anyhow::Error),
}
