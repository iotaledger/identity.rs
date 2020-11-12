pub type Result<T, E = Error> = anyhow::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid Linked Data Signature: {0}")]
    InvalidLDSignature(String),
    #[error("Invalid Key Format")]
    InvalidKeyFormat,
    #[error("Invalid Signature")]
    InvalidSignature,
    #[error("Invalid Document")]
    InvalidDocument,
    #[error(transparent)]
    Common(#[from] identity_core::Error),
}
