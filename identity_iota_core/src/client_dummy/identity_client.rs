use crate::IotaDID;
use crate::IotaDocument;
use crate::NetworkName;

use super::DummySigner;
use super::Identity;
use super::IdentityClientBuilder;
use super::IotaAddress;
use super::IotaClientTrait;
use super::ObjectID;

// dummy `IdentityClient` as placeholder to prepare wasm bindings for the actual one
// as long as it is not compilable to wasm

#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
#[non_exhaustive]
pub enum Error {
  #[error("identity dummy error was triggered; {0}")]
  Dummy(String),
  #[error("function or feature not implemented in dummy: {0}")]
  NotImplemented(String),
}

pub struct IdentityClient<T: IotaClientTrait> {
  client: T,
  network_name: NetworkName,
}

// builder related functions
impl<T> IdentityClient<T>
where
  T: IotaClientTrait<Error = Error>,
{
  pub fn builder() -> IdentityClientBuilder<T> {
    IdentityClientBuilder::<T>::default()
  }

  pub(crate) fn from_builder(builder: IdentityClientBuilder<T>) -> Result<Self, Error> {
    let network_name = NetworkName::try_from("dummy").unwrap();
    Ok(Self {
      client: builder.iota_client.unwrap(),
      network_name,
    })
  }
}

// mock functions for wasm integration
impl<T> IdentityClient<T>
where
  T: IotaClientTrait<Error = Error>,
{
  pub fn sender_public_key(&self) -> Result<&[u8], Error> {
    Ok(&([1, 2, 3, 4]))
  }

  pub fn sender_address(&self) -> Result<IotaAddress, Error> {
    Ok("dummy sender address".to_string())
  }

  pub fn network_name(&self) -> &NetworkName {
    &self.network_name
  }

  pub async fn get_identity(&self, _object_id: ObjectID) -> Result<Identity, Error> {
    Err(Error::NotImplemented("get_identity".to_string()))
  }

  pub async fn resolve_did(&self, _did: &IotaDID) -> Result<IotaDocument, Error> {
    Err(Error::NotImplemented("resolve_did".to_string()))
  }

  pub async fn publish_did_document(
    &self,
    _document: IotaDocument,
    _gas_budget: u64,
    _signer: &DummySigner,
  ) -> Result<IotaDocument, Error> {
    Err(Error::NotImplemented("publish_did_document".to_string()))
  }

  pub async fn publish_did_document_update(
    &self,
    _document: IotaDocument,
    _gas_budget: u64,
    _signer: &DummySigner,
  ) -> Result<IotaDocument, Error> {
    Err(Error::NotImplemented("publish_did_document_update".to_string()))
  }

  pub async fn deactivate_did_output(
    &self,
    _did: &IotaDID,
    _gas_budget: u64,
    _signer: &DummySigner,
  ) -> Result<(), Error> {
    Err(Error::NotImplemented("deactivate_did_output".to_string()))
  }
}

// test function(s) for wasm calling test
impl<T> IdentityClient<T>
where
  T: IotaClientTrait<Error = Error>,
{
  pub async fn get_chain_identifier(&self) -> Result<String, Error> {
    self.client.get_chain_identifier().await
  }
}
