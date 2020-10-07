/// The main crate result type derived from the `anyhow::Result` type.
pub type Result<T, E = Error> = anyhow::Result<T, E>;

/// The main crate Error type; uses `thiserror`.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A format error that takes a String.  Indicates that the Format of the did is not correct.
    #[error("Format Error: {0}")]
    FormatError(String),
    /// Error from when pest can not properly parse a line.
    #[error("Parse Error: {0}")]
    ParseError(#[from] anyhow::Error),
    #[error("Diff Error: {0}")]
    DiffError(#[from] identity_diff::Error),
    #[error("Failed to encode JSON: {0}")]
    EncodeJSON(serde_json::Error),
    #[error("Failed to decode JSON: {0}")]
    DecodeJSON(serde_json::Error),
    #[error("Invalid Timestamp: {0}")]
    InvalidTimestamp(#[from] chrono::ParseError),
    #[error("Invalid object id")]
    InvalidObjectId,
    #[error("Invalid object type")]
    InvalidObjectType,
    #[error("Invalid Credential: {0}")]
    InvalidCredential(String),
    #[error("Invalid Presentation: {0}")]
    InvalidPresentation(String),
}
