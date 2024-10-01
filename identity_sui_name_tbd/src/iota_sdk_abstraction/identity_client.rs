// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// This file has been moved here from identity_iota_core/src/client_dummy.
// The file will be removed after the TS-Client-SDK is integrated.
// The file provides a POC for the wasm-bindgen glue code needed to
// implement the TS-Client-SDK integration.

use std::str::FromStr;

use identity_iota_core::IotaDID;
use identity_iota_core::IotaDocument;
use identity_iota_core::NetworkName;

use super::DummySigner;
use super::Identity;
use super::IdentityBuilder;
use super::IdentityClientBuilder;
use super::IotaClientTrait;

use super::types::base_types::{IotaAddress, ObjectID};

// `IdentityClient` is a dummy placeholder to prepare wasm bindings for the actual one
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
    Ok(IotaAddress::from_str("dummy sender address")
        .map_err(|e| Error::Dummy(e.to_string()))?
    )
  }

  pub fn network_name(&self) -> &NetworkName {
    &self.network_name
  }

  pub fn create_identity(&self, _iota_document: &[u8]) -> IdentityBuilder {
    IdentityBuilder::new(&[], ObjectID::from_str("foobar").expect("foobar can not be parsed into ObjectId"))
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
    self.client.read_api().get_chain_identifier().await
  }
}
