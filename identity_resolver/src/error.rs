use thiserror::Error as DeriveError;

/// Crate Error type.
#[derive(Debug, DeriveError)]
pub enum Error {
    #[error("Format Error: {0}")]
    FormatError(String),
}

/// Crate result type.
pub type Result<T> = std::result::Result<T, Error>;
