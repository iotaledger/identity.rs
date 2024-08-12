use super::IdentityClientBuilder;
use super::IotaClientTrait;

// dummy `IdentityClient` as placeholder to prepare wasm bindings for the actual one
// as long as it is not compilable to wasm

#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
#[non_exhaustive]
pub enum Error {
  #[error("identity dummy error was triggered; {0}")]
  Dummy(String),
}

pub struct IdentityClient<T: IotaClientTrait> {
  client: T,
}

// functions aligned with actual identity client
impl<T> IdentityClient<T>
where
  T: IotaClientTrait<Error = Error>,
{
  pub fn new(client: T) -> Self
  where
    T: IotaClientTrait,
  {
    Self { client }
  }

  pub fn from_builder(builder: IdentityClientBuilder<T>) -> Result<Self, Error> {
    Ok(Self {
      client: builder.iota_client.unwrap(),
    })
  }
}

// function(s) for wasm integration test
impl<T> IdentityClient<T>
where
  T: IotaClientTrait<Error = Error>,
{
  pub async fn get_chain_identifier(&self) -> Result<String, Error> {
    self.client.get_chain_identifier().await
  }
}
