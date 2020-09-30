use anyhow::Result as AnyhowResult;
use thiserror::Error as DeriveError;

#[derive(Debug, DeriveError)]
pub enum Error {
    #[error("IOError: `{0}`")]
    IOError(#[from] std::io::Error),
    #[error("Dir Error: `{0}`")]
    DirError(String),
    #[error("Compression Error: `{0}`")]
    CompressionError(String),
}

pub type Result<T> = AnyhowResult<T, Error>;
