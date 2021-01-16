/// The main crate result type derived from the `anyhow::Result` type.
pub type Result<T, E = Error> = anyhow::Result<T, E>;

/// The main crate Error type; uses `thiserror`.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to encode JSON: {0}")]
    EncodeJSON(serde_json::Error),
    #[error("Failed to decode JSON: {0}")]
    DecodeJSON(serde_json::Error),
    #[error("Failed to decode base16 data: {0}")]
    DecodeBase16(#[from] hex::FromHexError),
    #[error("Failed to decode base58 data: {0}")]
    DecodeBase58(#[from] bs58::decode::Error),
    #[error("Failed to decode base64 data: {0}")]
    DecodeBase64(#[from] base64::DecodeError),
    #[error("Invalid DID: {0}")]
    InvalidDID(#[from] did_url::Error),
    #[error("Invalid DID Document: {0}")]
    InvalidDocument(#[from] did_doc::Error),
    #[error("Invalid Document Diff: {0}")]
    InvalidDiff(#[from] identity_diff::Error),
    #[error("Invalid Url: {0}")]
    InvalidUrl(#[from] did_doc::url::ParseError),
    #[error("Invalid Timestamp: {0}")]
    InvalidTimestamp(#[from] chrono::ParseError),
    #[error("Invalid Credential: {0}")]
    InvalidCredential(String),
    #[error("Invalid Presentation: {0}")]
    InvalidPresentation(String),
    #[error("DID Resolution Error: {0}")]
    ResolutionError(anyhow::Error),
    #[error("DID Dereference Error: {0}")]
    DereferenceError(anyhow::Error),
}
