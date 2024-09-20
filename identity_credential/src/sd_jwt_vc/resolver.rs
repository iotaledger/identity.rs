#![allow(async_fn_in_trait)]

use async_trait::async_trait;
use thiserror::Error;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
  #[error("The requested item \"{0}\" was not found.")]
  NotFound(String),
  #[error("Failed to parse the provided input into a resolvable type: {0}")]
  ParsingFailure(#[source] anyhow::Error),
  #[error(transparent)]
  Generic(#[from] anyhow::Error),
}

/// A type capable of asynchronously producing values of type [`Resolver::Target`] from inputs of type `I`.
#[async_trait]
pub trait Resolver<I: Sync> {
  /// The resulting type.
  type Target;
  /// Fetch the resource of type [`Resolver::Target`] using `input`.
  async fn resolve(&self, input: &I) -> Result<Self::Target>;
}
