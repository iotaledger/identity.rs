// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::boxed::Box;
use std::option::Option;
use std::result::Result;

use identity_iota::iota::iota_sdk_abstraction::error::IotaRpcResult;
use identity_iota::iota::iota_sdk_abstraction::rpc_types::CoinPage;
use identity_iota::iota::iota_sdk_abstraction::rpc_types::EventFilter;
use identity_iota::iota::iota_sdk_abstraction::rpc_types::EventPage;
use identity_iota::iota::iota_sdk_abstraction::rpc_types::IotaExecutionStatus;
use identity_iota::iota::iota_sdk_abstraction::rpc_types::IotaObjectData;
use identity_iota::iota::iota_sdk_abstraction::rpc_types::IotaObjectDataOptions;
use identity_iota::iota::iota_sdk_abstraction::rpc_types::IotaObjectResponse;
use identity_iota::iota::iota_sdk_abstraction::rpc_types::IotaObjectResponseQuery;
use identity_iota::iota::iota_sdk_abstraction::rpc_types::IotaPastObjectResponse;
use identity_iota::iota::iota_sdk_abstraction::rpc_types::IotaTransactionBlockResponseOptions;
use identity_iota::iota::iota_sdk_abstraction::rpc_types::ObjectsPage;
use identity_iota::iota::iota_sdk_abstraction::rpc_types::OwnedObjectRef;
use identity_iota::iota::iota_sdk_abstraction::types::base_types::IotaAddress;
use identity_iota::iota::iota_sdk_abstraction::types::base_types::ObjectID;
use identity_iota::iota::iota_sdk_abstraction::types::base_types::SequenceNumber;
use identity_iota::iota::iota_sdk_abstraction::types::digests::TransactionDigest;
use identity_iota::iota::iota_sdk_abstraction::types::dynamic_field::DynamicFieldName;
use identity_iota::iota::iota_sdk_abstraction::types::event::EventID;
use identity_iota::iota::iota_sdk_abstraction::types::quorum_driver_types::ExecuteTransactionRequestType;
use identity_iota::iota::iota_sdk_abstraction::CoinReadTrait;
use identity_iota::iota::iota_sdk_abstraction::EventTrait;
use identity_iota::iota::iota_sdk_abstraction::IotaClientTrait;
use identity_iota::iota::iota_sdk_abstraction::IotaKeySignature;
use identity_iota::iota::iota_sdk_abstraction::IotaTransactionBlockResponseT;
use identity_iota::iota::iota_sdk_abstraction::ProgrammableTransactionBcs;
use identity_iota::iota::iota_sdk_abstraction::QuorumDriverTrait;
use identity_iota::iota::iota_sdk_abstraction::ReadTrait;
use identity_iota::iota::iota_sdk_abstraction::SignatureBcs;
use identity_iota::iota::iota_sdk_abstraction::TransactionDataBcs;
use identity_iota::iota::sui_name_tbd_error::Error;
use secret_storage::Signer;

use super::super::ts_client_sdk::ManagedWasmIotaClient;
use super::super::ts_client_sdk::WasmIotaClient;
use super::super::types::IotaTransactionBlockResponseAdapter;

pub struct IotaTransactionBlockResponseProvider {
  response: IotaTransactionBlockResponseAdapter,
}

impl IotaTransactionBlockResponseProvider {
  pub fn new(response: IotaTransactionBlockResponseAdapter) -> Self {
    IotaTransactionBlockResponseProvider { response }
  }
}

#[async_trait::async_trait(?Send)]
impl IotaTransactionBlockResponseT for IotaTransactionBlockResponseProvider {
  type Error = Error;

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
}

pub struct ReadAdapter {
  client: ManagedWasmIotaClient,
}

#[async_trait::async_trait(?Send)]
impl ReadTrait for ReadAdapter {
  type Error = Error;

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
  ) -> IotaRpcResult<Box<dyn IotaTransactionBlockResponseT<Error = Self::Error>>> {
    unimplemented!("get_transaction_with_options");
  }

  async fn try_get_parsed_past_object(
    &self,
    object_id: ObjectID,
    version: SequenceNumber,
    options: IotaObjectDataOptions,
  ) -> IotaRpcResult<IotaPastObjectResponse> {
    self
      .client
      .try_get_parsed_past_object(object_id, version, options)
      .await
  }
}

pub struct QuorumDriverAdapter {
  client: ManagedWasmIotaClient,
}

#[async_trait::async_trait(?Send)]
impl QuorumDriverTrait for QuorumDriverAdapter {
  type Error = Error;

  async fn execute_transaction_block(
    &self,
    tx_data_bcs: &TransactionDataBcs,
    signatures: &Vec<SignatureBcs>,
    options: Option<IotaTransactionBlockResponseOptions>,
    request_type: Option<ExecuteTransactionRequestType>,
  ) -> IotaRpcResult<Box<dyn IotaTransactionBlockResponseT<Error = Self::Error>>> {
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
  type Error = Error;

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
  type Error = Error;

  async fn get_coins(
    &self,
    owner: IotaAddress,
    coin_type: Option<String>,
    cursor: Option<ObjectID>,
    limit: Option<usize>,
  ) -> IotaRpcResult<CoinPage> {
    todo!()
  }
}

#[derive(Clone)]
pub struct IotaClientTsSdk {
  iota_client: ManagedWasmIotaClient,
}

#[async_trait::async_trait(?Send)]
impl IotaClientTrait for IotaClientTsSdk {
  type Error = Error;

  fn quorum_driver_api(&self) -> Box<dyn QuorumDriverTrait<Error = Self::Error> + '_> {
    Box::new(QuorumDriverAdapter {
      client: self.iota_client.clone(),
    })
  }

  fn read_api(&self) -> Box<dyn ReadTrait<Error = Self::Error> + '_> {
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

  async fn execute_transaction<S: Signer<IotaKeySignature> + Sync>(
    &self,
    sender_address: IotaAddress,
    sender_public_key: &[u8],
    tx_bcs: ProgrammableTransactionBcs,
    gas_budget: Option<u64>,
    signer: &S,
  ) -> Result<Box<dyn IotaTransactionBlockResponseT<Error = Self::Error>>, Self::Error> {
    unimplemented!();
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
    unimplemented!();
  }
}

impl IotaClientTsSdk {
  pub fn new(iota_client: WasmIotaClient) -> Result<Self, Error> {
    Ok(Self {
      iota_client: ManagedWasmIotaClient::new(iota_client),
    })
  }
}
