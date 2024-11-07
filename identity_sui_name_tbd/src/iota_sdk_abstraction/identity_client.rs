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

use crate::error::Error;

use super::DummySigner;
use super::Identity;
use super::IdentityBuilder;
use super::IdentityClientBuilder;
use super::IotaClientTrait;

use super::types::base_types::{IotaAddress, ObjectID};

// `IdentityClient` is a dummy placeholder to prepare wasm bindings for the actual one
// as long as it is not compilable to wasm

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
    Ok(IotaAddress::from_str("0x0101010101010101010101010101010101010101010101010101010101010101")
        .map_err(|e| Error::InvalidArgument(e.to_string()))?
    )
  }

  pub fn network_name(&self) -> &NetworkName {
    &self.network_name
  }

  pub fn create_identity(&self, _iota_document: &[u8]) -> IdentityBuilder {
    IdentityBuilder::new(&[], ObjectID::from_str("foobar").expect("foobar can not be parsed into ObjectId"))
  }

  pub async fn get_identity(&self, _object_id: ObjectID) -> Result<Identity, Error> {
    unimplemented!("get_identity");
  }

  pub async fn resolve_did(&self, _did: &IotaDID) -> Result<IotaDocument, Error> {
    unimplemented!("resolve_did");
  }

  pub async fn publish_did_document(
    &self,
    _document: IotaDocument,
    _gas_budget: u64,
    _signer: &DummySigner,
  ) -> Result<IotaDocument, Error> {
    unimplemented!("publish_did_document");
  }

  pub async fn publish_did_document_update(
    &self,
    _document: IotaDocument,
    _gas_budget: u64,
    _signer: &DummySigner,
  ) -> Result<IotaDocument, Error> {
    unimplemented!("publish_did_document_update");
  }

  pub async fn deactivate_did_output(
    &self,
    _did: &IotaDID,
    _gas_budget: u64,
    _signer: &DummySigner,
  ) -> Result<(), Error> {
    unimplemented!("deactivate_did_output");
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
