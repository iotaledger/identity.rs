use anyhow::Result as AnyhowResult;
use thiserror::Error as DeriveError;

#[derive(Debug, DeriveError)]
pub enum Error {
    #[error("IOError: `{0}`")]
    IOError(#[from] std::io::Error),
}

pub type Result<T> = AnyhowResult<T, Error>;
