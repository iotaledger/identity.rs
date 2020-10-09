pub type Result<T, E = Error> = anyhow::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    CoreError(#[from] identity_core::Error),
    #[error(transparent)]
    DiffError(#[from] identity_core::diff::Error),
    #[error(transparent)]
    ClientError(#[from] iota::client::error::Error),
    #[error(transparent)]
    TernaryError(#[from] iota::ternary::Error),
    #[error("Invalid DID Method ID")]
    InvalidMethodId,
    #[error("Invalid Tryte Conversion")]
    InvalidTryteConversion,
    #[error("Invalid Transaction: {0}")]
    InvalidTransaction(TransactionError),
    #[error("Invalid Document: {0}")]
    InvalidDocument(DocumentError),
}

#[derive(Debug, thiserror::Error)]
pub enum TransactionError {
    #[error("Missing Trytes")]
    MissingTrytes,
    #[error("Missing Bundle")]
    MissingBundle,
    #[error("Missing Content")]
    MissingContent,
    #[error("Unconfirmable Transaction")]
    Unconfirmable,
}

#[derive(Debug, thiserror::Error)]
pub enum DocumentError {
    #[error("Missing Payload")]
    MissingPayload,
    #[error("Missing Timestamp (Updated)")]
    MissingUpdated,
    #[error("Invalid DID Network")]
    NetworkMismatch,
}
