// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::boxed::Box;
use std::option::Option;
use std::result::Result;

use identity_iota_interaction::rpc_types::IotaTransactionBlockEffects;
use identity_iota_interaction::types::crypto::Signature;
use identity_iota_interaction::types::digests::TransactionDigest;
use identity_iota_interaction::types::dynamic_field::DynamicFieldName;
use identity_iota_interaction::types::transaction::TransactionData;
use secret_storage::Signer;

use identity_iota_interaction::error::Error as IotaRpcError;
use identity_iota_interaction::error::IotaRpcResult;
use identity_iota_interaction::rpc_types::CoinPage;
use identity_iota_interaction::rpc_types::EventFilter;
use identity_iota_interaction::rpc_types::EventPage;
use identity_iota_interaction::rpc_types::IotaObjectData;
use identity_iota_interaction::rpc_types::IotaObjectDataOptions;
use identity_iota_interaction::rpc_types::IotaObjectResponse;
use identity_iota_interaction::rpc_types::IotaObjectResponseQuery;
use identity_iota_interaction::rpc_types::IotaPastObjectResponse;
use identity_iota_interaction::rpc_types::IotaTransactionBlockResponseOptions;
use identity_iota_interaction::rpc_types::ObjectsPage;
use identity_iota_interaction::types::base_types::IotaAddress;
use identity_iota_interaction::types::base_types::ObjectID;
use identity_iota_interaction::types::base_types::SequenceNumber;
use identity_iota_interaction::types::event::EventID;
use identity_iota_interaction::types::quorum_driver_types::ExecuteTransactionRequestType;
use identity_iota_interaction::types::transaction::ProgrammableTransaction as ProgrammableTransactionSdk;
use identity_iota_interaction::types::transaction::TransactionDataAPI as _;
use identity_iota_interaction::CoinReadTrait;
use identity_iota_interaction::EventTrait;
use identity_iota_interaction::IotaClientTrait;
use identity_iota_interaction::IotaKeySignature;
use identity_iota_interaction::IotaTransactionBlockResponseT;
use identity_iota_interaction::QuorumDriverTrait;
use identity_iota_interaction::ReadTrait;

use crate::bindings::ManagedWasmIotaClient;
use crate::bindings::WasmIotaClient;
use crate::bindings::WasmIotaTransactionBlockResponseWrapper;
use crate::error::TsSdkError;

#[allow(dead_code)]
pub trait IotaTransactionBlockResponseAdaptedT:
  IotaTransactionBlockResponseT<Error = TsSdkError, NativeResponse = WasmIotaTransactionBlockResponseWrapper>
{
}
impl<T> IotaTransactionBlockResponseAdaptedT for T where
  T: IotaTransactionBlockResponseT<Error = TsSdkError, NativeResponse = WasmIotaTransactionBlockResponseWrapper>
{
}
#[allow(dead_code)]
pub type IotaTransactionBlockResponseAdaptedTraitObj =
  Box<dyn IotaTransactionBlockResponseT<Error = TsSdkError, NativeResponse = WasmIotaTransactionBlockResponseWrapper>>;

#[allow(dead_code)]
pub trait QuorumDriverApiAdaptedT:
  QuorumDriverTrait<Error = TsSdkError, NativeResponse = WasmIotaTransactionBlockResponseWrapper>
{
}
impl<T> QuorumDriverApiAdaptedT for T where
  T: QuorumDriverTrait<Error = TsSdkError, NativeResponse = WasmIotaTransactionBlockResponseWrapper>
{
}
#[allow(dead_code)]
pub type QuorumDriverApiAdaptedTraitObj =
  Box<dyn QuorumDriverTrait<Error = TsSdkError, NativeResponse = WasmIotaTransactionBlockResponseWrapper>>;

#[allow(dead_code)]
pub trait ReadApiAdaptedT:
  ReadTrait<Error = TsSdkError, NativeResponse = WasmIotaTransactionBlockResponseWrapper>
{
}
impl<T> ReadApiAdaptedT for T where
  T: ReadTrait<Error = TsSdkError, NativeResponse = WasmIotaTransactionBlockResponseWrapper>
{
}
#[allow(dead_code)]
pub type ReadApiAdaptedTraitObj =
  Box<dyn ReadTrait<Error = TsSdkError, NativeResponse = WasmIotaTransactionBlockResponseWrapper>>;

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
  IotaClientTrait<Error = TsSdkError, NativeResponse = WasmIotaTransactionBlockResponseWrapper>
{
}
impl<T> IotaClientAdaptedT for T where
  T: IotaClientTrait<Error = TsSdkError, NativeResponse = WasmIotaTransactionBlockResponseWrapper>
{
}
#[allow(dead_code)]
pub type IotaClientAdaptedTraitObj =
  Box<dyn IotaClientTrait<Error = TsSdkError, NativeResponse = WasmIotaTransactionBlockResponseWrapper>>;

pub struct IotaTransactionBlockResponseProvider {
  response: WasmIotaTransactionBlockResponseWrapper,
  effects: Option<IotaTransactionBlockEffects>,
}

impl IotaTransactionBlockResponseProvider {
  pub fn new(response: WasmIotaTransactionBlockResponseWrapper) -> Self {
    let effects = response.effects().map(Into::into);
    IotaTransactionBlockResponseProvider { response, effects }
  }
}

#[async_trait::async_trait(?Send)]
impl IotaTransactionBlockResponseT for IotaTransactionBlockResponseProvider {
  type Error = TsSdkError;
  type NativeResponse = WasmIotaTransactionBlockResponseWrapper;

  fn to_string(&self) -> String {
    format!("{:?}", self.response.to_string())
  }

  fn effects(&self) -> Option<&IotaTransactionBlockEffects> {
    self.effects.as_ref()
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

  fn digest(&self) -> Result<TransactionDigest, Self::Error> {
    self.response.digest()
  }
}

pub struct ReadAdapter {
  client: ManagedWasmIotaClient,
}

#[async_trait::async_trait(?Send)]
impl ReadTrait for ReadAdapter {
  type Error = TsSdkError;
  type NativeResponse = WasmIotaTransactionBlockResponseWrapper;

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
  type NativeResponse = WasmIotaTransactionBlockResponseWrapper;

  async fn execute_transaction_block(
    &self,
    tx_data: TransactionData,
    signatures: Vec<Signature>,
    options: Option<IotaTransactionBlockResponseOptions>,
    request_type: Option<ExecuteTransactionRequestType>,
  ) -> IotaRpcResult<IotaTransactionBlockResponseAdaptedTraitObj> {
    let wasm_response = self
      .client
      .execute_transaction_block(tx_data, signatures, options, request_type)
      .await?;

    let digest = wasm_response
      .digest()
      .map_err(|e| IotaRpcError::FfiError(e.to_string()))?;

    self
      .client
      .wait_for_transaction(digest, Some(IotaTransactionBlockResponseOptions::new()), None, None)
      .await
      .unwrap();

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
  type NativeResponse = WasmIotaTransactionBlockResponseWrapper;

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
    tx_data: TransactionData,
    signer: &S,
  ) -> Result<
    Box<dyn IotaTransactionBlockResponseT<Error = Self::Error, NativeResponse = Self::NativeResponse>>,
    Self::Error,
  > {
    let response = self.sdk_execute_transaction(tx_data, signer).await?;

    // wait until new transaction block is available
    self
      .iota_client
      .wait_for_transaction(
        response.digest()?,
        Some(IotaTransactionBlockResponseOptions::new()),
        None,
        None,
      )
      .await
      .unwrap();

    Ok(Box::new(response))
  }

  async fn default_gas_budget(
    &self,
    _sender_address: IotaAddress,
    _tx: &ProgrammableTransactionSdk,
  ) -> Result<u64, Self::Error> {
    Ok(50_000_000)
  }

  async fn get_previous_version(&self, _iod: IotaObjectData) -> Result<Option<IotaObjectData>, Self::Error> {
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

  // Submit tx to IOTA client, also:
  //   - signs tx
  //   - calls execute_transaction_block to submit tx (with signatures created here)
  async fn sdk_execute_transaction<S: Signer<IotaKeySignature>>(
    &self,
    tx: TransactionData,
    signer: &S,
  ) -> Result<IotaTransactionBlockResponseProvider, TsSdkError> {
    let sender_public_key = signer
      .public_key()
      .await
      .map_err(|e| TsSdkError::WasmError(String::from("SecretStorage"), e.to_string()))?;
    let sender_address = IotaAddress::from(&sender_public_key);
    if sender_address != tx.sender() {
      return Err(TsSdkError::WasmError("SDK".to_owned(), format!("transaction data needs to be signed by address {}, but client can only provide signature for address {sender_address}", tx.sender())));
    }
    let signature = signer
      .sign(&tx)
      .await
      .map_err(|e| TsSdkError::WasmError("SecretStorage".to_owned(), e.to_string()))?;

    let wasm_response = self
      .quorum_driver_api()
      .execute_transaction_block(
        tx,
        vec![signature],
        Some(IotaTransactionBlockResponseOptions::full_content()),
        Some(ExecuteTransactionRequestType::WaitForLocalExecution),
      )
      .await
      .unwrap();
    let native = wasm_response.clone_native_response();

    Ok(IotaTransactionBlockResponseProvider::new(native))
  }
}
