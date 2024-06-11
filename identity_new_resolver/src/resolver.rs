#![allow(async_fn_in_trait)]

use crate::Result;

pub trait Resolver<I> {
  type Target;
  async fn resolve(&self, input: &I) -> Result<Self::Target>;
}
