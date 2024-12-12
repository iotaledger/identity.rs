// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::option::Option;
use std::result::Result;
use std::boxed::Box;
use std::marker::Send;
use async_trait::async_trait;
use secret_storage::{Signer, SignatureScheme};
use crate::{OptionalSend, ProgrammableTransactionBcs, TransactionDataBcs, SignatureBcs,
            IotaTransactionBlockResponseBcs};
use crate::rpc_types::{
  IotaTransactionBlockResponseOptions,
  IotaObjectResponse,
  IotaPastObjectResponse,
  ObjectsPage,
  IotaObjectResponseQuery,
  IotaObjectDataOptions,
  IotaExecutionStatus,
  CoinPage,
  EventFilter,
  EventPage,
  IotaObjectData,
  OwnedObjectRef,
};
use crate::types::{
  quorum_driver_types::ExecuteTransactionRequestType,
  dynamic_field::DynamicFieldName,
  digests::TransactionDigest,
  event::EventID,
};
use crate::types::base_types::{SequenceNumber, IotaAddress, ObjectID};
use crate::error::IotaRpcResult;

pub struct IotaKeySignature {
  pub public_key: Vec<u8>,
  pub signature: Vec<u8>,
}

impl SignatureScheme for IotaKeySignature {
  type PublicKey = Vec<u8>;
  type Signature = Vec<u8>;
}

//********************************************************************
// TODO: rename the following traits to have a consistent relation
//  between platform specific trait specializations
//  and the platform agnostic traits specified in this file:
//  * QuorumDriverTrait -> QuorumDriverApiT
//  * ReadTrait -> ReadApiT
//  * CoinReadTrait -> CoinReadApiT
//  * EventTrait -> EventApiT
//
// Platform specific trait specializations are defined
// in modules identity_iota_core::iota_interaction_rust and
// iota_interaction_ts with the following names:
//  * QuorumDriverApiAdaptedT
//  * ReadApiAdaptedT
//  * CoinReadApiAdaptedT
//  * EventApiAdaptedT
//  * IotaClientAdaptedT
//********************************************************************


/// Adapter Allowing to query information from an IotaTransactionBlockResponse instance.
/// As IotaTransactionBlockResponse pulls too many dependencies we need to
/// hide it behind a trait.
#[cfg_attr(not(feature = "send-sync-transaction"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-transaction", async_trait)]
pub trait IotaTransactionBlockResponseT: OptionalSend {
  /// Error type used
  type Error;
  /// The response type used in the platform specific client sdk
  type NativeResponse;

  /// Indicates if IotaTransactionBlockResponse::effects is None
  fn effects_is_none(&self) -> bool;
  /// Indicates if there are Some(effects)
  fn effects_is_some(&self) -> bool;

  /// Returns Debug representation of the IotaTransactionBlockResponse
  fn to_string(&self) -> String;

  /// Returns Bcs serialization of the IotaTransactionBlockResponse
  fn to_bcs(&self) -> Result<IotaTransactionBlockResponseBcs, Self::Error>;

    /// If effects_is_some(), returns a clone of the IotaTransactionBlockEffectsAPI::status()
  /// Otherwise, returns None
  fn effects_execution_status(&self) -> Option<IotaExecutionStatus>;

  /// If effects_is_some(), returns IotaTransactionBlockEffectsAPI::created()
  /// as owned Vec.
  /// Otherwise, returns None
  fn effects_created(&self) -> Option<Vec<OwnedObjectRef>>;

  /// Returns a reference to the platform specific client sdk response instance wrapped by this adapter
  fn as_native_response(&mut self) -> &mut Self::NativeResponse;

  /// Consumes this adapter and returns the wrapped platform specific client sdk response
  fn into_native_response(self) -> Self::NativeResponse ;
}

#[cfg_attr(not(feature = "send-sync-transaction"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-transaction", async_trait)]
pub trait QuorumDriverTrait {
  /// Error type used
  type Error;
  /// The response type used in the platform specific client sdk
  type NativeResponse;

  async fn execute_transaction_block(
    &self,
    tx_data_bcs: &TransactionDataBcs,
    signatures: &Vec<SignatureBcs>,
    options: Option<IotaTransactionBlockResponseOptions>,
    request_type: Option<ExecuteTransactionRequestType>,
  ) -> IotaRpcResult<Box<dyn IotaTransactionBlockResponseT<Error = Self::Error, NativeResponse=Self::NativeResponse>>>;
}

#[cfg_attr(not(feature = "send-sync-transaction"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-transaction", async_trait)]
pub trait ReadTrait {
  /// Error type used
  type Error;
  /// The response type used in the platform specific client sdk
  type NativeResponse;

  async fn get_chain_identifier(&self) -> Result<String, Self::Error>;

  async fn get_dynamic_field_object(
    &self,
    parent_object_id: ObjectID,
    name: DynamicFieldName,
  ) -> IotaRpcResult<IotaObjectResponse>;

  async fn get_object_with_options(
    &self,
    object_id: ObjectID,
    options: IotaObjectDataOptions,
  ) -> IotaRpcResult<IotaObjectResponse>;

  async fn get_owned_objects(
    &self,
    address: IotaAddress,
    query: Option<IotaObjectResponseQuery>,
    cursor: Option<ObjectID>,
    limit: Option<usize>,
  ) -> IotaRpcResult<ObjectsPage>;

  async fn get_reference_gas_price(&self) -> IotaRpcResult<u64>;

  async fn get_transaction_with_options(
    &self,
    digest: TransactionDigest,
    options: IotaTransactionBlockResponseOptions,
  ) -> IotaRpcResult<Box<dyn IotaTransactionBlockResponseT<Error = Self::Error, NativeResponse=Self::NativeResponse>>>;

  async fn try_get_parsed_past_object(
    &self,
    object_id: ObjectID,
    version: SequenceNumber,
    options: IotaObjectDataOptions,
  ) -> IotaRpcResult<IotaPastObjectResponse>;
}

#[cfg_attr(not(feature = "send-sync-transaction"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-transaction", async_trait)]
pub trait CoinReadTrait {
  type Error;

  async fn get_coins(
    &self,
    owner: IotaAddress,
    coin_type: Option<String>,
    cursor: Option<ObjectID>,
    limit: Option<usize>,
  ) -> IotaRpcResult<CoinPage>;
}


#[cfg_attr(not(feature = "send-sync-transaction"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-transaction", async_trait)]
pub trait EventTrait {
  /// Error type used
  type Error;

  async fn query_events(
    &self,
    query: EventFilter,
    cursor: Option<EventID>,
    limit: Option<usize>,
    descending_order: bool,
  ) -> IotaRpcResult<EventPage>;
}

#[cfg_attr(not(feature = "send-sync-transaction"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-transaction", async_trait)]
pub trait IotaClientTrait {
  /// Error type used
  type Error;
  /// The response type used in the platform specific client sdk
  type NativeResponse;

  #[cfg(not(feature = "send-sync-transaction"))]
  fn quorum_driver_api(&self) -> Box<dyn QuorumDriverTrait<Error = Self::Error, NativeResponse=Self::NativeResponse>+ '_>;
  #[cfg(feature = "send-sync-transaction")]
  fn quorum_driver_api(&self) -> Box<dyn QuorumDriverTrait<Error = Self::Error, NativeResponse=Self::NativeResponse> + Send + '_>;

  #[cfg(not(feature = "send-sync-transaction"))]
  fn read_api(&self) -> Box<dyn ReadTrait<Error = Self::Error, NativeResponse=Self::NativeResponse> + '_>;
  #[cfg(feature = "send-sync-transaction")]
  fn read_api(&self) -> Box<dyn ReadTrait<Error = Self::Error, NativeResponse=Self::NativeResponse> + Send + '_>;

  #[cfg(not(feature = "send-sync-transaction"))]
  fn coin_read_api(&self) -> Box<dyn CoinReadTrait<Error = Self::Error> + '_>;
  #[cfg(feature = "send-sync-transaction")]
  fn coin_read_api(&self) -> Box<dyn CoinReadTrait<Error = Self::Error> + Send + '_>;

  #[cfg(not(feature = "send-sync-transaction"))]
  fn event_api(&self) -> Box<dyn EventTrait<Error = Self::Error> + '_>;
  #[cfg(feature = "send-sync-transaction")]
  fn event_api(&self) -> Box<dyn EventTrait<Error = Self::Error> + Send + '_>;

  #[cfg(not(feature = "send-sync-transaction"))]
  async fn execute_transaction<S: Signer<IotaKeySignature>>(
    &self,
    sender_address: IotaAddress,
    sender_public_key: &[u8],
    tx_bcs: ProgrammableTransactionBcs,
    gas_budget: Option<u64>,
    signer: &S
  ) -> Result<Box<dyn IotaTransactionBlockResponseT<Error=Self::Error, NativeResponse=Self::NativeResponse>>, Self::Error>;
  #[cfg(feature = "send-sync-transaction")]
  async fn execute_transaction<S: Signer<IotaKeySignature> + Sync>(
    &self,
    sender_address: IotaAddress,
    sender_public_key: &[u8],
    tx_bcs: ProgrammableTransactionBcs,
    gas_budget: Option<u64>,
    signer: &S
  ) -> Result<Box<dyn IotaTransactionBlockResponseT<Error=Self::Error, NativeResponse=Self::NativeResponse>>, Self::Error>;

  async fn default_gas_budget(
    &self,
    sender_address: IotaAddress,
    tx_bcs: &ProgrammableTransactionBcs
  ) -> Result<u64, Self::Error>;

  async fn get_previous_version(&self, iod: IotaObjectData) -> Result<Option<IotaObjectData>, Self::Error>;

  async fn get_past_object(
    &self,
    object_id: ObjectID,
    version: SequenceNumber,
  ) -> Result<IotaPastObjectResponse, Self::Error>;
}