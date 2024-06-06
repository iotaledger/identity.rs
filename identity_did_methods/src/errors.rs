/// Alias for a `Result` with the error type [`Error`].
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// This type represents errors that can occur when constructing credentials and presentations or their serializations.
#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
#[non_exhaustive]
pub enum Error {
    /// Caused by a failure during TLS backend initialization or configuration loading.
    #[error("Error while adding a root certificate")]
    AddRootCertificateError,
    /// Caused by a failure during TLS backend initialization or configuration loading.
    #[error("Error while building the WebClient")]
    WebClientBuildError(#[source] reqwest::Error),
    /// Caused by an error while writing a DID document to a file.
    #[error("Error while writing a DID Document on a file")]
    DocumentFileWriteError(&'static str),
    /// Caused by an invalid DID.
    #[error("invalid did")]
    DIDSyntaxError(#[source] identity_did::Error),
    /// Caused by an invalid DID document.
    #[error("invalid document")]
    InvalidDoc(#[source] identity_document::Error),
    /// Caused by an error during JSON Web Signature verification.
    #[error("jws signature verification failed")]
    JwsVerificationError(#[source] identity_document::Error),
}