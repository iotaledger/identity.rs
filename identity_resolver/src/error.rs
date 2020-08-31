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
    /// Network node Error
    #[error("Network Error: Wrong network specified, DID requires {0:?}")]
    NetworkNodeError(&'static str),
    /// Node Error
    #[error("Node Error: No node available")]
    NodeError,
    /// identity_core Error
    #[error("identity_core Error: {0}")]
    IdentityCoreError(#[from] identity_core::Error),
    /// identity_integration Error
    #[error("identity_core Error: {0}")]
    IdentityIntegrationError(#[from] identity_integration::Error),
    /// serde_json Error
    #[error("serde_json Error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
}
pub type Result<T> = AnyhowResult<T, Error>;
