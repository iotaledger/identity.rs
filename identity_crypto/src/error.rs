#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Poisoned RwLock (read)")]
  RwLockPoisonedRead,
  #[error("Poisoned RwLock (write)")]
  RwLockPoisonedWrite,
  #[error("Proof not found: `{0}`")]
  MissingProofType(String),
}

pub type Result<T> = std::result::Result<T, Error>;
