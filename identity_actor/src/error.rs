use serde::{ser::Serializer, Deserialize, Serialize};

/// The identity error type.
#[derive(Debug, thiserror::Error)]
pub enum IdentityMessageError {
    /// Unknown error.
    #[error("`{0}`")]
    UnknownError(String),
    /// Generic error.
    #[error("{0}")]
    GenericError(#[from] anyhow::Error),
    /// IO error.
    #[error("`{0}`")]
    IoError(#[from] std::io::Error),
    /// serde_json error.
    #[error("`{0}`")]
    JsonError(#[from] serde_json::error::Error),
    /// Message not found.
    #[error("message not found")]
    MessageNotFound,
}
