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
    ParseError(anyhow::Error),
    #[error("Diff Error: {0}")]
    DiffError(#[from] identity_diff::Error),
    #[error("Crypto Error: {0}")]
    CryptoError(#[from] identity_crypto::Error),
    #[error("Failed to encode JSON: {0}")]
    EncodeJSON(serde_json::Error),
    #[error("Failed to decode JSON: {0}")]
    DecodeJSON(serde_json::Error),
    #[error("Invalid Timestamp: {0}")]
    InvalidTimestamp(#[from] chrono::ParseError),
    #[error("Failed to decode base16 data: {0}")]
    DecodeBase16(#[from] hex::FromHexError),
    #[error("Failed to decode base58 data: {0}")]
    DecodeBase58(#[from] bs58::decode::Error),
    #[error("Failed to decode base64 data: {0}")]
    DecodeBase64(#[from] base64::DecodeError),
    #[error("Invalid object id")]
    InvalidObjectId,
    #[error("Invalid object type")]
    InvalidObjectType,
    #[error("Invalid key type")]
    InvalidKeyType,
    #[error("Invalid Credential: {0}")]
    InvalidCredential(String),
    #[error("Invalid Presentation: {0}")]
    InvalidPresentation(String),
    #[error("Invalid Url: {0}")]
    InvalidUrl(#[from] url::ParseError),
    #[error("Invalid UTF-8: {0}")]
    InvalidUtf8(#[from] core::str::Utf8Error),
    #[error("DID Resolution Error: {0}")]
    ResolutionError(anyhow::Error),
    #[error("DID Dereference Error: {0}")]
    DereferenceError(anyhow::Error),
    #[error("Identity Reader Error: {0}")]
    IdentityReaderError(anyhow::Error),
    #[error("Identity Writer Error: {0}")]
    IdentityWriterError(anyhow::Error),
}
