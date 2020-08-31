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
  #[error(transparent)]
  KeyError(anyhow::Error),
  #[error(transparent)]
  SignError(anyhow::Error),
  #[error(transparent)]
  VerifyError(anyhow::Error),
  #[error(transparent)]
  DecodeBase64(#[from] base64::DecodeError),
  #[error("Invalid proof document: {0}")]
  InvalidProofDocument(String),
  #[error("Failed to canonicalize document")]
  InvalidCanonicalization,
}

pub type Result<T> = std::result::Result<T, Error>;
