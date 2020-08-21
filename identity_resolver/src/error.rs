use anyhow::Result as AnyhowResult;
use thiserror::Error as thisErr;
#[derive(Debug, thisErr)]
pub enum Error {
    /// Didn't found a document
    #[error("Resolve Error: No document found")]
    DocumentNotFound,
    /// Fetching data from the Tangle failed
    #[error("Fetching Error: No data from nodes")]
    FetchingError,
    /// DID parsing error
    #[error("Parsing Error: Can't parse DID")]
    DIDParsingError,
}
pub type Result<T> = AnyhowResult<T, Error>;
