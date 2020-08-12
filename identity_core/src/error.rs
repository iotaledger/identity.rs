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
}

/// Crate result type.
pub type Result<T> = AnyhowResult<T, Error>;
