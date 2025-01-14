// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// This file has been moved here from identity_iota_core/src/client_dummy.
// The file will be removed after the TS-Client-SDK is integrated.
// The file provides a POC for the wasm-bindgen glue code needed to
// implement the TS-Client-SDK integration.

use std::str::FromStr;

use identity_iota::iota::rebased::{rebased_err, Error};
use identity_iota::iota::IotaDID;
use identity_iota::iota::IotaDocument;
use identity_iota::iota::NetworkName;
use identity_iota::iota_interaction::rpc_types::IotaTransactionBlockResponseOptions;
use identity_iota::iota_interaction::types::base_types::{IotaAddress, ObjectID};
use identity_iota::iota_interaction::IotaClientTrait;
use identity_iota::iota_interaction::{IotaTransactionBlockResponseT, SignatureBcs, TransactionDataBcs};

use super::DummySigner;
use super::Identity;
use super::IdentityBuilder;
use super::IdentityClientBuilder;
use iota_interaction_ts::bindings::IotaTransactionBlockResponseAdapter;
use iota_interaction_ts::error::TsSdkError;

// `IdentityClient` is a dummy placeholder to prepare wasm bindings for the actual one
// as long as it is not compilable to wasm

pub struct IdentityClient<T: IotaClientTrait> {
  client: T,
  network_name: NetworkName,
  identity_iota_package_id: IotaAddress,
  sender_address: IotaAddress,
  sender_public_key: Vec<u8>,
}

// builder related functions
impl<T> IdentityClient<T>
where
  T: IotaClientTrait<Error=TsSdkError>,
{
  pub fn builder() -> IdentityClientBuilder<T> {
    IdentityClientBuilder::<T>::default()
  }

  pub(crate) fn from_builder(builder: IdentityClientBuilder<T>) -> Result<Self, Error> {
    let network_name = NetworkName::try_from(builder.network_name.unwrap_or("dummy".to_string())).unwrap();
    let sender_address = builder.sender_address.unwrap_or(IotaAddress::default());
    Ok(Self {
      client: builder.iota_client.unwrap(),
      network_name,
      identity_iota_package_id: IotaAddress::from(
        builder.identity_iota_package_id.unwrap_or(
          ObjectID::from_str("0x0101010101010101010101010101010101010101010101010101010101010101")
            .map_err(|e| Error::InvalidArgument(e.to_string()))?
        )),
      sender_public_key: builder.sender_public_key.unwrap_or(vec![1u8, 2u8, 3u8, 4u8]),
      sender_address,
    })
  }
}

// mock functions for wasm integration
impl<T> IdentityClient<T>
where
  T: IotaClientTrait<Error=TsSdkError, NativeResponse=IotaTransactionBlockResponseAdapter>,
{
  pub fn sender_public_key(&self) -> Result<&[u8], Error> {
    Ok(self.sender_public_key.as_ref())
  }

  pub fn sender_address(&self) -> Result<IotaAddress, Error> { Ok(self.sender_address.clone()) }

  pub fn network_name(&self) -> &NetworkName {
    &self.network_name
  }

  pub fn create_identity(&self, _iota_document: &[u8]) -> IdentityBuilder {
    IdentityBuilder::new(&[], ObjectID::from_str("foobar").expect("foobar can not be parsed into ObjectId"))
  }

  pub async fn get_identity(&self, _object_id: ObjectID) -> Result<Identity, Error> {
    unimplemented!("get_identity");
  }

  pub async fn execute_dummy_transaction(
    &self,
    tx_data_bcs: TransactionDataBcs,
    signatures: Vec<SignatureBcs>,
  ) -> Result<Box<dyn IotaTransactionBlockResponseT<Error=TsSdkError, NativeResponse=IotaTransactionBlockResponseAdapter>>, Error> {
    let tx_response = self
      .client
      .quorum_driver_api()
      .execute_transaction_block(
        &tx_data_bcs,
        &signatures,
        Some(IotaTransactionBlockResponseOptions::new().with_effects()),
        None,
      ).await?;
    Ok(tx_response)
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
  T: IotaClientTrait<Error=TsSdkError, NativeResponse=IotaTransactionBlockResponseAdapter>,
{
  pub async fn get_chain_identifier(&self) -> Result<String, Error> {
    self.client.read_api().get_chain_identifier().await.map_err(rebased_err)
  }
}
