use anyhow::Result as AnyhowResult;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum Error {
    /// Didn't found a document
    #[error("Resolve Error: No document found")]
    DocumentNotFound,
}
pub type Result<T> = AnyhowResult<T, Error>;
