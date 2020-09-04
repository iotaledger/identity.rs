#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to encode JSON: {0}")]
    EncodeJSON(serde_json::Error),
    #[error("Failed to decode JSON: {0}")]
    DecodeJSON(serde_json::Error),
    #[error("Invalid Timestamp: {0}")]
    InvalidTimestamp(#[from] chrono::ParseError),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
