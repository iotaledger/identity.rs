#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Poisoned RwLock (read)")]
  RwLockPoisonedRead,
  #[error("Poisoned RwLock (write)")]
  RwLockPoisonedWrite,
  #[error("Proof not found: `{0}`")]
  MissingProofType(String),
  #[error("Invalid key length for `{0}`")]
  InvalidKeyLength(&'static str),
  #[error("Invalid key: `{0}`")]
  KeyError(anyhow::Error),
  #[error("Failed to create signature: {0}")]
  SignError(anyhow::Error),
  #[error("Failed to verify signature: {0}")]
  VerifyError(anyhow::Error),
  #[error("Failed to decode base64: {0}")]
  DecodeBase64(#[from] base64::DecodeError),
  #[error("Failed to decode JSON: {0}")]
  DecodeJSON(serde_json::Error),
  #[error("Failed to encode JSON: {0}")]
  EncodeJSON(serde_json::Error),
  #[error("Invalid proof document: {0}")]
  InvalidProofDocument(String),
  #[error("Failed to canonicalize document")]
  InvalidCanonicalization,
}

pub type Result<T> = std::result::Result<T, Error>;
