use anyhow::Result as AnyhowResult;
use thiserror::Error as DeriveError;

#[derive(Debug, DeriveError)]
pub enum Error {
    #[error("Diff Error: {0}")]
    DiffError(String),
    #[error("Merge Error: {0}")]
    MergeError(String),
    #[error("Conversion Error: {0}")]
    ConversionError(String),
}

pub type Result<T> = AnyhowResult<T, Error>;
