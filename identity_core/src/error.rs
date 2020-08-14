use anyhow::Result as AnyhowResult;
use thiserror::Error as DeriveError;

use pest::error::Error as PestError;

use crate::did_parser::Rule;

/// Crate Error type.
#[derive(Debug, DeriveError)]
pub enum Error {
    #[error("Format Error: {0}")]
    FormatError(String),
    #[error("Parse Error: {0}")]
    ParseError(#[from] PestError<Rule>),
    #[error("Key Format Error: This Key encoding type is not supported")]
    KeyFormatError,
    #[error("Key Type Error: This key type is not supported")]
    KeyTypeError,
    #[error("Json Error: {0}")]
    SerdeError(#[from] serde_json::Error),
}

/// Crate result type.
pub type Result<T> = AnyhowResult<T, Error>;
