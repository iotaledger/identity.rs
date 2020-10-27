pub type Result<T, E = Error> = anyhow::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    CoreError(#[from] identity_core::Error),
    #[error(transparent)]
    CryptoError(#[from] identity_crypto::Error),
    #[error(transparent)]
    DiffError(#[from] identity_core::diff::Error),
    #[error(transparent)]
    ProofError(#[from] identity_proof::error::Error),
    #[error(transparent)]
    ClientError(#[from] iota::client::error::Error),
    #[error(transparent)]
    TernaryError(#[from] iota::ternary::Error),
    #[error("Invalid DID Method")]
    InvalidMethod,
    #[error("Invalid DID Method ID")]
    InvalidMethodId,
    #[error("Invalid DID Signature")]
    InvalidSignature,
    #[error("Invalid DID Network")]
    InvalidDIDNetwork,
    #[error("Invalid DID Authentication Key")]
    InvalidAuthenticationKey,
    #[error("Invalid DID Proof")]
    InvalidProof,
    #[error("Invalid Tryte Conversion")]
    InvalidTryteConversion,
    #[error("Invalid Transaction Bundle")]
    InvalidTransactionBundle,
    #[error("Invalid Transaction Hashes")]
    InvalidTransactionHashes,
    #[error("Invalid Transaction Trytes")]
    InvalidTransactionTrytes,
    #[error("Invalid Transfer Tail")]
    InvalidTransferTail,
    #[error("Transfer Unconfirmable")]
    TransferUnconfirmable,
    #[error("Invalid Transaction: {0}")]
    InvalidTransaction(TransactionError),
    #[error("Invalid Document: {0}")]
    InvalidDocument(DocumentError),
}

#[derive(Debug, thiserror::Error)]
pub enum TransactionError {
    #[error("Invalid Bundle")]
    InvalidBundle,
    #[error("Missing Bundle")]
    MissingBundle,
    #[error("Missing Content")]
    MissingContent,
    #[error("Missing Trytes")]
    MissingTrytes,
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
