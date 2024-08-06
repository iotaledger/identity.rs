use super::KinesisClientTrait;

// dummy `IdentityClient` as placeholder to prepare wasm bindings for the actual one
// as long as it is not compilable to wasm

/// will be deleted after impl
pub struct KinesisIdentityClientDummy<T: KinesisClientTrait> {
  client: T,
}

impl<T> KinesisIdentityClientDummy<T>
where
  T: KinesisClientTrait<Error = anyhow::Error>,
{
  /// will be deleted after impl
  pub fn new(client: T) -> Self
  where
    T: KinesisClientTrait,
  {
    Self { client }
  }

  /// will be deleted after impl
  pub async fn get_chain_identifier(&self) -> Result<String, anyhow::Error> {
    self.client.get_chain_identifier().await
  }
}
