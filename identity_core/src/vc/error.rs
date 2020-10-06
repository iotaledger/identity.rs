#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid Credential: {0}")]
    InvalidCredential(String),
    #[error("Invalid Presentation: {0}")]
    InvalidPresentation(String),
    #[error("Missing base type for {0}")]
    MissingBaseType(&'static str),
    #[error("Missing base context for {0}")]
    MissingBaseContext(&'static str),
    #[error("Invalid base context for {0}")]
    InvalidBaseContext(&'static str),
    #[error("Invalid URI for {0}")]
    InvalidURI(&'static str),
    #[error("Missing `Credential` subject")]
    MissingCredentialSubject,
    #[error("Invalid `Credential` subject")]
    InvalidCredentialSubject,
    #[error("Missing `Credential` issuer")]
    MissingCredentialIssuer,
}

pub type Result<T, E = Error> = ::core::result::Result<T, E>;
