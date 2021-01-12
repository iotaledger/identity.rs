pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IOTA Error: {0}")]
    IotaError(#[from] identity_iota::error::Error),
    #[error("IOTA Core Error: {0}")]
    CoreError(#[from] identity_core::error::Error),
    #[error("JOSE Error: {0}")]
    JoseError(#[from] libjose::error::Error),
    #[error("DID Document Error: {0}")]
    DocumentError(#[from] did_doc::Error),
}
