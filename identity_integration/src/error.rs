use anyhow::Result as AnyhowResult;
use std::convert::Infallible;
use thiserror::Error as thisErr;
#[derive(Debug, thisErr)]
pub enum Error {
    /// Didn't get any transactions
    #[error("Fetching Error: No transactions found for this DID")]
    TransactionsNotFound,
    /// Didn't get any tail transactions
    #[error("Fetching Error: Couldn't find transactions")]
    MissingTransactions,
    /// iota.rs Error
    #[error("iota.rs Error: {0}")]
    IotarsError(#[from] iota::client::error::Error),
    /// bee_ternary Error
    #[error("bee_ternary Error: {0}")]
    BeeTernaryError(#[from] iota::ternary::Error),
    /// identity_core Error
    #[error("identity_core Error: {0}")]
    IdentityCoreError(#[from] identity_core::Error),
    /// serde_json Error
    #[error("serde_json Error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    /// Node Error
    #[error("Node Error: Wrong network specified, DID requires {0:?}")]
    NetworkNodeError(&'static str),
    /// Tryteconversion Error
    #[error("Tryteconversion Error: Couldn't convert public key to trytes")]
    TryteConversionError,
    /// identity_crypto Error
    #[error("identity_crypto Error: {0}")]
    IdentityCryptoError(#[from] identity_crypto::Error),
    /// bs58 Error
    #[error("bs58 decode Error: {0}")]
    Bs58Error(#[from] bs58::decode::Error),
}

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}

pub type Result<T> = AnyhowResult<T, Error>;
