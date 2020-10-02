use anyhow::Result as AnyhowResult;
use identity_common::Error as CommonError;
use pest::error::Error as PestError;
use thiserror::Error as DeriveError;

use crate::did_parser::Rule;

/// The main crate Error type; uses `thiserror`.
#[derive(Debug, DeriveError)]
pub enum Error {
    /// A format error that takes a String.  Indicates that the Format of the did is not correct.
    #[error("Format Error: {0}")]
    FormatError(String),
    /// Error from when pest can not properly parse a line.
    #[error("Parse Error: {0}")]
    ParseError(#[from] PestError<Rule>),
    /// Error for when the key format is not supported.
    #[error("Key Format Error: This Key encoding type is not supported")]
    KeyFormatError,
    /// Error for when the key type is not supported.
    #[error("Key Type Error: This key type is not supported")]
    KeyTypeError,
    /// Json related error from `serde_json`
    #[error("Json Error: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error(transparent)]
    CommonError(#[from] CommonError),
}

/// The main crate result type derived from the `anyhow::Result` type.
pub type Result<T> = AnyhowResult<T, Error>;
