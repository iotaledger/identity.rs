// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::option::Option;
use std::result::Result;
use std::boxed::Box;
use std::marker::Send;
use async_trait::async_trait;
use secret_storage::{Signer, SignatureScheme};

use crate::iota_sdk_abstraction::{
  ProgrammableTransactionBcs,
  TransactionBcs,
  rpc_types::{
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
  },
  types::{
    quorum_driver_types::ExecuteTransactionRequestType,
    dynamic_field::DynamicFieldName,
    digests::TransactionDigest,
    base_types::{SequenceNumber, IotaAddress, ObjectID},
    event::EventID,
  },
  error::IotaRpcResult,
};

pub struct IotaKeySignature {
  pub public_key: Vec<u8>,
  pub signature: Vec<u8>,
}

impl SignatureScheme for IotaKeySignature {
  type PublicKey = Vec<u8>;
  type Signature = Vec<u8>;
}

/// Allows to query information from an IotaTransactionBlockResponse instance.
/// As IotaTransactionBlockResponse pulls too many dependencies we need to
/// hide it behind a trait.
#[async_trait::async_trait()]
pub trait IotaTransactionBlockResponseT: Send {
  type Error;

  /// Indicates if IotaTransactionBlockResponse::effects is None
  fn effects_is_none(&self) -> bool;
  /// Indicates if there are Some(effects)
  fn effects_is_some(&self) -> bool;

  /// Returns Debug representation of the IotaTransactionBlockResponse
  fn to_string(&self) -> String;

    /// If effects_is_some(), returns a clone of the IotaTransactionBlockEffectsAPI::status()
  /// Otherwise, returns None
  fn effects_execution_status(&self) -> Option<IotaExecutionStatus>;

  /// If effects_is_some(), returns IotaTransactionBlockEffectsAPI::created()
  /// as owned Vec.
  /// Otherwise, returns None
  fn effects_created(&self) -> Option<Vec<OwnedObjectRef>>;
}

#[async_trait::async_trait()]
pub trait QuorumDriverTrait {
  type Error;

  async fn execute_transaction_block(
    &self,
    tx_bcs: TransactionBcs,
    options: IotaTransactionBlockResponseOptions,
    request_type: Option<ExecuteTransactionRequestType>,
  ) -> IotaRpcResult<Box<dyn IotaTransactionBlockResponseT<Error = Self::Error>>>;
}

#[async_trait::async_trait()]
pub trait ReadTrait {
  type Error;

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
  ) -> IotaRpcResult<Box<dyn IotaTransactionBlockResponseT<Error = Self::Error>>>;

  async fn try_get_parsed_past_object(
    &self,
    object_id: ObjectID,
    version: SequenceNumber,
    options: IotaObjectDataOptions,
  ) -> IotaRpcResult<IotaPastObjectResponse>;
}

#[async_trait::async_trait()]
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


#[async_trait::async_trait()]
pub trait EventTrait {
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
  type Error;

  fn quorum_driver_api(&self) -> Box<dyn QuorumDriverTrait<Error = Self::Error> + Send + '_>;

  fn read_api(&self) -> Box<dyn ReadTrait<Error = Self::Error> + Send + '_>;

  fn coin_read_api(&self) -> Box<dyn CoinReadTrait<Error = Self::Error> + Send + '_>;

  fn event_api(&self) -> Box<dyn EventTrait<Error = Self::Error> + Send + '_>;

  async fn execute_transaction<S: Signer<IotaKeySignature> + Sync>(
    &self,
    sender_address: IotaAddress,
    sender_public_key: &[u8],
    tx_bcs: ProgrammableTransactionBcs,
    gas_budget: Option<u64>,
    signer: &S
  ) -> Result<Box<dyn IotaTransactionBlockResponseT<Error=Self::Error>>, Self::Error>;

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