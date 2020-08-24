use anyhow::Result as AnyhowResult;
use thiserror::Error as thisErr;
#[derive(Debug, thisErr)]
pub enum Error {
    /// Didn't get any transactions
    #[error("Fetching Error: No transactions found for this DID")]
    TransactionsNotFound,
    /// Didn't get any tail transactions
    #[error("Fetching Error: Couldn't get any tail transactions")]
    MissingTailTransaction,
    /// iota.rs Error
    #[error("iota.rs Error: {0}")]
    IotarsError(#[from] iota::client::error::Error),
    /// bee_ternary Error
    #[error("bee_ternary Error: {0}")]
    BeeTernaryError(#[from] iota::ternary::Error),
    /// bee_ternary Error
    #[error("Sorting Error: Couldn't sort transactions to bundles")]
    TransactionSortingFailed,
}
pub type Result<T> = AnyhowResult<T, Error>;
