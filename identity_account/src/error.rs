use anyhow::Result as AnyhowResult;
use thiserror::Error as DeriveError;

#[derive(Debug, DeriveError)]
pub enum Error {}

pub type Result<T> = AnyhowResult<T, Error>;
