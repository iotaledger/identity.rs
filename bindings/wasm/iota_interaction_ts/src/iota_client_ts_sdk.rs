// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::boxed::Box;
use std::option::Option;
use std::result::Result;

use fastcrypto::hash::Blake2b256;
use identity_iota_interaction::shared_crypto::intent::Intent;
use identity_iota_interaction::shared_crypto::intent::IntentMessage;
use identity_iota_interaction::types::crypto::SignatureScheme;
use identity_iota_interaction::types::digests::TransactionDigest;
use identity_iota_interaction::types::dynamic_field::DynamicFieldName;
use js_sys::Uint8Array;
use secret_storage::Signer;

use identity_iota_interaction::error::IotaRpcResult;
use identity_iota_interaction::rpc_types::CoinPage;
use identity_iota_interaction::rpc_types::EventFilter;
use identity_iota_interaction::rpc_types::EventPage;
use identity_iota_interaction::rpc_types::IotaExecutionStatus;
use identity_iota_interaction::rpc_types::IotaObjectData;
use identity_iota_interaction::rpc_types::IotaObjectDataOptions;
use identity_iota_interaction::rpc_types::IotaObjectResponse;
use identity_iota_interaction::rpc_types::IotaObjectResponseQuery;
use identity_iota_interaction::rpc_types::IotaPastObjectResponse;
use identity_iota_interaction::rpc_types::IotaTransactionBlockResponseOptions;
use identity_iota_interaction::rpc_types::ObjectsPage;
use identity_iota_interaction::rpc_types::OwnedObjectRef;
use identity_iota_interaction::types::base_types::IotaAddress;
use identity_iota_interaction::types::base_types::ObjectID;
use identity_iota_interaction::types::base_types::SequenceNumber;
use identity_iota_interaction::types::event::EventID;
use identity_iota_interaction::types::quorum_driver_types::ExecuteTransactionRequestType;
use identity_iota_interaction::CoinReadTrait;
use identity_iota_interaction::EventTrait;
use identity_iota_interaction::IotaClientTrait;
use identity_iota_interaction::IotaKeySignature;
use identity_iota_interaction::IotaTransactionBlockResponseT;
use identity_iota_interaction::ProgrammableTransactionBcs;
use identity_iota_interaction::QuorumDriverTrait;
use identity_iota_interaction::ReadTrait;
use identity_iota_interaction::SignatureBcs;
use identity_iota_interaction::TransactionDataBcs;

use crate::bindings::add_gas_data_to_transaction;
use crate::bindings::get_transaction_digest;
use crate::bindings::sleep;
use crate::bindings::WasmIotaTxBlockResponseAdapter;
use crate::bindings::ManagedWasmIotaClient;
use crate::bindings::WasmIotaClient;
use crate::error::TsSdkError;
use crate::error::WasmError;
use crate::ProgrammableTransaction;

#[allow(dead_code)]
pub trait IotaTransactionBlockResponseAdaptedT:
  IotaTransactionBlockResponseT<Error = TsSdkError, NativeResponse =WasmIotaTxBlockResponseAdapter>
{
}
impl<T> IotaTransactionBlockResponseAdaptedT for T where
  T: IotaTransactionBlockResponseT<Error = TsSdkError, NativeResponse =WasmIotaTxBlockResponseAdapter>
{
}
#[allow(dead_code)]
pub type IotaTransactionBlockResponseAdaptedTraitObj =
  Box<dyn IotaTransactionBlockResponseT<Error = TsSdkError, NativeResponse =WasmIotaTxBlockResponseAdapter>>;

#[allow(dead_code)]
pub trait QuorumDriverApiAdaptedT:
  QuorumDriverTrait<Error = TsSdkError, NativeResponse =WasmIotaTxBlockResponseAdapter>
{
}
impl<T> QuorumDriverApiAdaptedT for T where
  T: QuorumDriverTrait<Error = TsSdkError, NativeResponse =WasmIotaTxBlockResponseAdapter>
{
}
#[allow(dead_code)]
pub type QuorumDriverApiAdaptedTraitObj =
  Box<dyn QuorumDriverTrait<Error = TsSdkError, NativeResponse =WasmIotaTxBlockResponseAdapter>>;

#[allow(dead_code)]
pub trait ReadApiAdaptedT: ReadTrait<Error = TsSdkError, NativeResponse =WasmIotaTxBlockResponseAdapter> {}
impl<T> ReadApiAdaptedT for T where
  T: ReadTrait<Error = TsSdkError, NativeResponse =WasmIotaTxBlockResponseAdapter>
{
}
#[allow(dead_code)]
pub type ReadApiAdaptedTraitObj =
  Box<dyn ReadTrait<Error = TsSdkError, NativeResponse =WasmIotaTxBlockResponseAdapter>>;

#[allow(dead_code)]
pub trait CoinReadApiAdaptedT: CoinReadTrait<Error = TsSdkError> {}
impl<T> CoinReadApiAdaptedT for T where T: CoinReadTrait<Error = TsSdkError> {}
#[allow(dead_code)]
pub type CoinReadApiAdaptedTraitObj = Box<dyn CoinReadTrait<Error = TsSdkError>>;

#[allow(dead_code)]
pub trait EventApiAdaptedT: EventTrait<Error = TsSdkError> {}
impl<T> EventApiAdaptedT for T where T: EventTrait<Error = TsSdkError> {}
#[allow(dead_code)]
pub type EventApiAdaptedTraitObj = Box<dyn EventTrait<Error = TsSdkError>>;

#[allow(dead_code)]
pub trait IotaClientAdaptedT:
  IotaClientTrait<Error = TsSdkError, NativeResponse =WasmIotaTxBlockResponseAdapter>
{
}
impl<T> IotaClientAdaptedT for T where
  T: IotaClientTrait<Error = TsSdkError, NativeResponse =WasmIotaTxBlockResponseAdapter>
{
}
#[allow(dead_code)]
pub type IotaClientAdaptedTraitObj =
  Box<dyn IotaClientTrait<Error = TsSdkError, NativeResponse =WasmIotaTxBlockResponseAdapter>>;

pub struct IotaTransactionBlockResponseProvider {
  response: WasmIotaTxBlockResponseAdapter,
}

impl IotaTransactionBlockResponseProvider {
  pub fn new(response: WasmIotaTxBlockResponseAdapter) -> Self {
    IotaTransactionBlockResponseProvider { response }
  }
}

#[async_trait::async_trait(?Send)]
impl IotaTransactionBlockResponseT for IotaTransactionBlockResponseProvider {
  type Error = TsSdkError;
  type NativeResponse = WasmIotaTxBlockResponseAdapter;

  fn effects_is_none(&self) -> bool {
    self.response.effects_is_none()
  }

  fn effects_is_some(&self) -> bool {
    self.response.effects_is_some()
  }

  fn to_string(&self) -> String {
    format!("{:?}", self.response.to_string())
  }

  fn effects_execution_status(&self) -> Option<IotaExecutionStatus> {
    self
      .response
      .effects_execution_status()
      .map(|wasm_status| wasm_status.into())
  }

  fn effects_created(&self) -> Option<Vec<OwnedObjectRef>> {
    self
      .response
      .effects_created()
      .map(|wasm_o_ref_vec| wasm_o_ref_vec.into())
  }

  fn as_native_response(&self) -> &Self::NativeResponse {
    &self.response
  }

  fn as_mut_native_response(&mut self) -> &mut Self::NativeResponse {
    &mut self.response
  }

  fn clone_native_response(&self) -> Self::NativeResponse {
    self.response.clone()
  }
}

pub struct ReadAdapter {
  client: ManagedWasmIotaClient,
}

#[async_trait::async_trait(?Send)]
impl ReadTrait for ReadAdapter {
  type Error = TsSdkError;
  type NativeResponse = WasmIotaTxBlockResponseAdapter;

  async fn get_chain_identifier(&self) -> Result<String, Self::Error> {
    Ok(self.client.get_chain_identifier().await.unwrap())
  }

  async fn get_dynamic_field_object(
    &self,
    parent_object_id: ObjectID,
    name: DynamicFieldName,
  ) -> IotaRpcResult<IotaObjectResponse> {
    self.client.get_dynamic_field_object(parent_object_id, name).await
  }

  async fn get_object_with_options(
    &self,
    object_id: ObjectID,
    options: IotaObjectDataOptions,
  ) -> IotaRpcResult<IotaObjectResponse> {
    self.client.get_object_with_options(object_id, options).await
  }

  async fn get_owned_objects(
    &self,
    address: IotaAddress,
    query: Option<IotaObjectResponseQuery>,
    cursor: Option<ObjectID>,
    limit: Option<usize>,
  ) -> IotaRpcResult<ObjectsPage> {
    self.client.get_owned_objects(address, query, cursor, limit).await
  }

  async fn get_reference_gas_price(&self) -> IotaRpcResult<u64> {
    self.client.get_reference_gas_price().await
  }

  async fn get_transaction_with_options(
    &self,
    digest: TransactionDigest,
    options: IotaTransactionBlockResponseOptions,
  ) -> IotaRpcResult<IotaTransactionBlockResponseAdaptedTraitObj> {
    let wasm_response = self.client.get_transaction_with_options(digest, options).await?;

    Ok(Box::new(IotaTransactionBlockResponseProvider::new(wasm_response)))
  }

  async fn try_get_parsed_past_object(
    &self,
    _object_id: ObjectID,
    _version: SequenceNumber,
    _options: IotaObjectDataOptions,
  ) -> IotaRpcResult<IotaPastObjectResponse> {
    // TODO: does not work anymore, find out, why we need to pass a different `SequenceNumber` now
    unimplemented!("try_get_parsed_past_object");
    // self
    //   .client
    //   .try_get_parsed_past_object(object_id, version, options)
    //   .await
  }
}

pub struct QuorumDriverAdapter {
  client: ManagedWasmIotaClient,
}

#[async_trait::async_trait(?Send)]
impl QuorumDriverTrait for QuorumDriverAdapter {
  type Error = TsSdkError;
  type NativeResponse = WasmIotaTxBlockResponseAdapter;

  async fn execute_transaction_block(
    &self,
    tx_data_bcs: &TransactionDataBcs,
    signatures: &[SignatureBcs],
    options: Option<IotaTransactionBlockResponseOptions>,
    request_type: Option<ExecuteTransactionRequestType>,
  ) -> IotaRpcResult<IotaTransactionBlockResponseAdaptedTraitObj> {
    let wasm_response = self
      .client
      .execute_transaction_block(tx_data_bcs, signatures, options, request_type)
      .await?;
    Ok(Box::new(IotaTransactionBlockResponseProvider::new(wasm_response)))
  }
}

pub struct EventAdapter {
  client: ManagedWasmIotaClient,
}

#[async_trait::async_trait(?Send)]
impl EventTrait for EventAdapter {
  type Error = TsSdkError;

  async fn query_events(
    &self,
    query: EventFilter,
    cursor: Option<EventID>,
    limit: Option<usize>,
    descending_order: bool,
  ) -> IotaRpcResult<EventPage> {
    self.client.query_events(query, cursor, limit, descending_order).await
  }
}

pub struct CoinReadAdapter {
  client: ManagedWasmIotaClient,
}

#[async_trait::async_trait(?Send)]
impl CoinReadTrait for CoinReadAdapter {
  type Error = TsSdkError;

  async fn get_coins(
    &self,
    owner: IotaAddress,
    coin_type: Option<String>,
    cursor: Option<ObjectID>,
    limit: Option<usize>,
  ) -> IotaRpcResult<CoinPage> {
    self.client.get_coins(owner, coin_type, cursor, limit).await
  }
}

#[derive(Clone)]
pub struct IotaClientTsSdk {
  iota_client: ManagedWasmIotaClient,
}

#[async_trait::async_trait(?Send)]
impl IotaClientTrait for IotaClientTsSdk {
  type Error = TsSdkError;
  type NativeResponse = WasmIotaTxBlockResponseAdapter;

  fn quorum_driver_api(&self) -> QuorumDriverApiAdaptedTraitObj {
    Box::new(QuorumDriverAdapter {
      client: self.iota_client.clone(),
    })
  }

  fn read_api(&self) -> ReadApiAdaptedTraitObj {
    Box::new(ReadAdapter {
      client: self.iota_client.clone(),
    })
  }

  fn coin_read_api(&self) -> Box<dyn CoinReadTrait<Error = Self::Error> + '_> {
    Box::new(CoinReadAdapter {
      client: self.iota_client.clone(),
    })
  }

  fn event_api(&self) -> Box<dyn EventTrait<Error = Self::Error> + '_> {
    Box::new(EventAdapter {
      client: self.iota_client.clone(),
    })
  }

  async fn execute_transaction<S: Signer<IotaKeySignature>>(
    &self,
    sender_address: IotaAddress,
    sender_public_key: &[u8],
    tx_bcs: ProgrammableTransactionBcs,
    gas_budget: Option<u64>,
    signer: &S,
  ) -> Result<Box<dyn IotaTransactionBlockResponseT<Error = Self::Error, NativeResponse = Self::NativeResponse>>, Self::Error> {
    let tx: ProgrammableTransaction = tx_bcs.try_into()?;
    let response = self
      .sdk_execute_transaction(sender_address, sender_public_key, tx, gas_budget, signer)
      .await?;

    // wait a certain amount to time before continuing
    // a follow up step was fetching an object created with this tx, which - for some reason - wasn't available yet
    // TODO: check timing issues related to transactions finality here
    sleep(500)
        .await
        .map_err(WasmError::from)
        .map_err(TsSdkError::from)?;

    Ok(Box::new(response))
  }

  async fn default_gas_budget(
    &self,
    sender_address: IotaAddress,
    tx_bcs: &ProgrammableTransactionBcs,
  ) -> Result<u64, Self::Error> {
    unimplemented!();
  }

  async fn get_previous_version(&self, iod: IotaObjectData) -> Result<Option<IotaObjectData>, Self::Error> {
    unimplemented!();
  }

  async fn get_past_object(
    &self,
    object_id: ObjectID,
    version: SequenceNumber,
  ) -> Result<IotaPastObjectResponse, Self::Error> {
    self
      .iota_client
      .try_get_parsed_past_object(object_id, version, IotaObjectDataOptions::full_content())
      .await
      .map_err(|err| {
        // TODO: check error variant here, selection has been reduced / focused
        // Self::Error::InvalidIdentityHistory(format!("could not look up object {object_id} version {version}; {err}"))
        Self::Error::JsSysError(format!("could not look up object {object_id} version {version}; {err}"))
      })
  }
}

impl IotaClientTsSdk {
  pub fn new(iota_client: WasmIotaClient) -> Result<Self, TsSdkError> {
    Ok(Self {
      iota_client: ManagedWasmIotaClient::new(iota_client),
    })
  }

  /// Builds message with `TransactionData` intent, hashes it, and constructs full signature.
  async fn get_transaction_signature_bytes<S: Signer<IotaKeySignature>>(
    signer: &S,
    tx_data: &Vec<u8>,
    sender_public_key: &[u8],
  ) -> Result<Vec<u8>, TsSdkError> {
    let digest = get_transaction_digest(tx_data);
  
    let raw_signature = signer
      .sign(&digest.to_vec())
      .await
      .map_err(|err| 
        TsSdkError::TransactionSerializationError(format!("could not sign transaction message; {err}"))
      )?;
  
    let signature_bytes = [
      [SignatureScheme::ED25519.flag()].as_slice(),
      &raw_signature,
      sender_public_key,
    ]
    .concat();
  
    Ok(signature_bytes)
  }

  /// Inserts these values into the transaction and replaces placeholder values.
  /// 
  ///   - sender (overwritten as we assume a placeholder to be used in prepared transaction)
  ///   - gas budget (value determined automatically if not provided)
  ///   - gas price (value determined automatically)
  ///   - gas coin / payment object (fetched automatically)
  ///   - gas owner (equals sender)
  /// 
  /// # Arguments
  /// 
  ///   * `iota_client` -  client instance
  ///   * `sender_address` -  transaction sender (and the one paying for it)
  ///   * `tx_bcs` -  transaction data serialized to bcs, most probably having placeholder values 
  ///   * `gas_budget` -  optional fixed gas budget, determined automatically with a dry run if not provided
  async fn replace_transaction_placeholder_values(
    &self,
    tx: ProgrammableTransaction,
    sender_address: IotaAddress,
    gas_budget: Option<u64>,
  ) -> Result<Vec<u8>, TsSdkError> {
    let tx_bcs = tx.0.build()
      .await
      .map_err(WasmError::from)?
      .to_vec();
    let updated = add_gas_data_to_transaction(
      &self.iota_client.0,
      sender_address,
      tx_bcs,
      gas_budget,
    ).await?;

    Ok(updated)
  }

  // Submit tx to IOTA client, also:
  //   - ensures, `gas_budget` is set
  //   - signs tx
  //   - calls execute_transaction_block to submit tx (with signatures created here)
  async fn sdk_execute_transaction<S: Signer<IotaKeySignature>>(
    &self,
    sender_address: IotaAddress,
    sender_public_key: &[u8],
    tx: ProgrammableTransaction,
    gas_budget: Option<u64>,
    signer: &S,
  ) -> Result<IotaTransactionBlockResponseProvider, TsSdkError> {
    let final_tx = self.replace_transaction_placeholder_values(tx, sender_address, gas_budget).await?;
    let signature = Self::get_transaction_signature_bytes(signer, &final_tx, sender_public_key).await?;

    let wasm_response = self
      .quorum_driver_api()
      .execute_transaction_block(
        &final_tx,
        &vec![signature],
        Some(IotaTransactionBlockResponseOptions::full_content()),
        Some(ExecuteTransactionRequestType::WaitForLocalExecution),
      )
      .await.unwrap();
    let native = wasm_response.clone_native_response();

    Ok(IotaTransactionBlockResponseProvider::new(native))
  }
}
