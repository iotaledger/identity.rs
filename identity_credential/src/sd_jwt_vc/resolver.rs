#![allow(async_fn_in_trait)]

use thiserror::Error;

type Result<T> = std::result::Result<T, Error>;

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
pub trait Resolver<I> {
  /// The resulting type.
  type Target;
  /// Fetch the resource of type [`Resolver::Target`] using `input`.
  async fn resolve(&self, input: &I) -> Result<Self::Target>;
  /// Like [`Resolver::resolve`] but with multiple inputs.
  async fn resolve_multiple(&self, inputs: impl AsRef<[I]>) -> Result<Vec<Self::Target>> {
    let mut results = Vec::<Self::Target>::with_capacity(inputs.as_ref().len());
    for input in inputs.as_ref() {
      let result = self.resolve(input).await?;
      results.push(result);
    }

    Ok(results)
  }
}