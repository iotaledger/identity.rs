#![allow(async_fn_in_trait)]

use crate::Result;

pub trait Resolver<I> {
  type Target;
  async fn resolve(&self, input: &I) -> Result<Self::Target>;
  async fn resolve_multiple(&self, inputs: impl AsRef<[I]>) -> Result<Vec<Self::Target>> {
    let mut results = Vec::<Self::Target>::with_capacity(inputs.as_ref().len());
    for input in inputs.as_ref() {
      let result = self.resolve(input).await?;
      results.push(result);
    }

    Ok(results)
  }
}
