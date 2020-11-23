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
    #[error("Bincode Error: `{0}`")]
    BincodeError(#[from] bincode::Error),
    #[error("Failed to encode JSON: `{0}`")]
    EncodeError(serde_json::Error),
    #[error("Failed to decode JSON: {0}")]
    DecodeJSON(serde_json::Error),
    #[error("Identity_core Error: `{0}`")]
    IdentityCoreError(#[from] identity_core::Error),
    #[error("FromUtf8 Error: `{0}`")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error("Identity_diff Error: `{0}`")]
    DiffError(#[from] identity_diff::Error),
    #[error("Identity_iota Error: `{0}`")]
    IdentityIotaError(#[from] identity_iota::error::Error),
    #[error("Identity state Error: `{0}`")]
    StateError(String),
}

pub type Result<T> = AnyhowResult<T, Error>;
