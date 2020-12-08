pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Core Error: {0}")]
    CoreError(#[from] identity_core::Error),
    #[error("Diff Error: {0}")]
    DiffError(#[from] identity_core::identity_diff::Error),
    #[error("Invalid DID: {0}")]
    InvalidDID(#[from] identity_core::did_url::Error),
    #[error("Invalid Document: {0}")]
    InvalidDoc(#[from] identity_core::did_doc::Error),
    #[error("Client Error: {0}")]
    ClientError(#[from] iota::client::error::Error),
    #[error("Ternary Error: {0}")]
    TernaryError(#[from] iota::ternary::Error),
    #[error("Invalid Document: {error}")]
    InvalidDocument { error: &'static str },
    #[error("Invalid DID Network")]
    InvalidDIDNetwork,
    #[error("Invalid Tryte Conversion")]
    InvalidTryteConversion,
    #[error("Invalid Transaction Bundle")]
    InvalidTransactionBundle,
    #[error("Invalid Transaction Hashes")]
    InvalidTransactionHashes,
    #[error("Invalid Transaction Trytes")]
    InvalidTransactionTrytes,
    #[error("Invalid Bundle Tail")]
    InvalidBundleTail,
    #[error("Chain Error: {error}")]
    ChainError { error: &'static str },
}
