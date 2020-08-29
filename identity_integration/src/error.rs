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
    /// bee_ternary Error
    #[error("Sorting Error: Couldn't sort transactions to bundles")]
    TransactionSortingFailed,
    /// Node Error
    #[error("Node Error: Wrong network specified")]
    NetworkNodeError,
}

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}

pub type Result<T> = AnyhowResult<T, Error>;
