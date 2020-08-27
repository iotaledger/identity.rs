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
    /// Node parsing Error
    #[error("Parsing Error: Can't parse node URL")]
    NodeURLParsingError,
    /// Node Error
    #[error("Node Error: Can't find a node for the selected network")]
    NetworkNodeError,
    /// identity_core Error
    #[error("identity_core Error: {0}")]
    IdentityCoreError(#[from] identity_core::Error),
    /// identity_integration Error
    #[error("identity_core Error: {0}")]
    IdentityIntegrationError(#[from] identity_integration::Error),
}
pub type Result<T> = AnyhowResult<T, Error>;
