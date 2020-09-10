use chrono::ParseError as ChronoError;
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
    #[error("Invalid timestamp format ({0})")]
    InvalidTimestamp(ChronoError),
    #[error("Missing `Credential` subject")]
    MissingCredentialSubject,
    #[error("Invalid `Credential` subject")]
    InvalidCredentialSubject,
    #[error("Missing `Credential` issuer")]
    MissingCredentialIssuer,
    #[error("Failed to decode JSON: {0}")]
    DecodeJSON(serde_json::Error),
    #[error("Failed to encode JSON: {0}")]
    EncodeJSON(serde_json::Error),
}

pub type Result<T> = StdResult<T, Error>;
