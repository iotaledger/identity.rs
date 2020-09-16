pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Invalid Base64: {0}")]
  InvalidBase64(#[from] base64::DecodeError),
  #[error("Invalid JSON: {0}")]
  InvalidJson(#[from] serde_json::Error),
  #[error("Invalid JWK Format: {0}")]
  InvalidJwkFormat(anyhow::Error),
}
