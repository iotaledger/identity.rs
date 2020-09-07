use anyhow::Result as AnyhowResult;
use thiserror::Error as thisErr;
#[derive(Debug, thisErr)]
pub enum Error {
    /// Didn't found a document
    #[error("Resolve Error: No document found")]
    DocumentNotFound,
    /// Node parsing Error
    #[error("Parsing Error: Can't parse node URL")]
    NodeURLParsingError,
    /// Dereferencing Error
    #[error("Dereferencing Error: Couldn't dereference URL")]
    DereferencingError,
    /// Network node Error
    #[error("Network Error: Wrong network specified, DID requires {0:?}")]
    NetworkNodeError(&'static str),
    /// Node Error
    #[error("Node Error: No node available")]
    NodeError,
    /// DID method Error
    #[error("DID Error: DID method not supported")]
    DIDMethodError,
    /// identity_core Error
    #[error("identity_core Error: {0}")]
    IdentityCoreError(#[from] identity_core::Error),
    /// identity_integration Error
    #[error("identity_core Error: {0}")]
    IdentityIntegrationError(#[from] identity_integration::Error),
    /// serde_json Error
    #[error("serde_json Error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    /// Utf8Error
    #[error("Utf8Error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
    /// URL Error
    #[error("URL parse Error: {0}")]
    URLError(#[from] url::ParseError),
}
pub type Result<T> = AnyhowResult<T, Error>;
