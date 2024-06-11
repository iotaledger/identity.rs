#![allow(async_fn_in_trait)]

use crate::Result;

pub trait Resolver<I, T> {
  async fn resolve(&self, input: &I) -> Result<T>;
}
