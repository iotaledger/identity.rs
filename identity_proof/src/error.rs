#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to canonicalize object: {0}")]
    Canonicalize(anyhow::Error),
    #[error("Failed to pre-process document: {0}")]
    PreProcess(anyhow::Error),
    #[error("Invalid LD Signature: {0}")]
    InvalidLDSignature(String),
    #[error(transparent)]
    Common(#[from] identity_core::Error),
    #[error(transparent)]
    Crypto(#[from] identity_crypto::Error),
    #[error(transparent)]
    Custom(#[from] anyhow::Error),
}

pub type Result<T, E = Error> = anyhow::Result<T, E>;
