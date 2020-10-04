use identity_common::Error as CommonError;
use std::result::Result as StdResult;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Cannot convert `Object` to `{0}`")]
    BadObjectConversion(&'static str),
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
    #[error(transparent)]
    CommonError(#[from] CommonError),
}

pub type Result<T, E = Error> = StdResult<T, E>;
