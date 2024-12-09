// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::iota::iota_sdk_abstraction::error::Error as IotaRpcError;
use identity_iota::iota::iota_sdk_abstraction::error::IotaRpcResult;
use identity_iota::iota::iota_sdk_abstraction::generated_types::ExecuteTransactionBlockParams;
use identity_iota::iota::iota_sdk_abstraction::generated_types::GetCoinsParams;
use identity_iota::iota::iota_sdk_abstraction::generated_types::GetDynamicFieldObjectParams;
use identity_iota::iota::iota_sdk_abstraction::generated_types::GetObjectParams;
use identity_iota::iota::iota_sdk_abstraction::generated_types::GetOwnedObjectsParams;
use identity_iota::iota::iota_sdk_abstraction::generated_types::QueryEventsParams;
use identity_iota::iota::iota_sdk_abstraction::generated_types::SortOrder;
use identity_iota::iota::iota_sdk_abstraction::generated_types::TryGetPastObjectParams;
use identity_iota::iota::iota_sdk_abstraction::rpc_types::CoinPage;
use identity_iota::iota::iota_sdk_abstraction::rpc_types::EventFilter;
use identity_iota::iota::iota_sdk_abstraction::rpc_types::EventPage;
use identity_iota::iota::iota_sdk_abstraction::rpc_types::IotaObjectDataOptions;
use identity_iota::iota::iota_sdk_abstraction::rpc_types::IotaObjectResponse;
use identity_iota::iota::iota_sdk_abstraction::rpc_types::IotaObjectResponseQuery;
use identity_iota::iota::iota_sdk_abstraction::rpc_types::IotaPastObjectResponse;
use identity_iota::iota::iota_sdk_abstraction::rpc_types::IotaTransactionBlockResponseOptions;
use identity_iota::iota::iota_sdk_abstraction::rpc_types::ObjectsPage;
use identity_iota::iota::iota_sdk_abstraction::types::base_types::IotaAddress;
use identity_iota::iota::iota_sdk_abstraction::types::base_types::ObjectID;
use identity_iota::iota::iota_sdk_abstraction::types::base_types::SequenceNumber;
use identity_iota::iota::iota_sdk_abstraction::types::dynamic_field::DynamicFieldName;
use identity_iota::iota::iota_sdk_abstraction::types::event::EventID;
use identity_iota::iota::iota_sdk_abstraction::types::quorum_driver_types::ExecuteTransactionRequestType;
use identity_iota::iota::iota_sdk_abstraction::SignatureBcs;
use identity_iota::iota::iota_sdk_abstraction::TransactionDataBcs;
use identity_iota::iota::sui_name_tbd_error::Error;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

use crate::common::PromiseBigint;
use crate::common::PromiseString;
use crate::console_log;
use crate::error::JsValueResult;
use crate::kinesis::PromiseIotaObjectResponse;
use crate::kinesis::PromiseObjectRead;
use crate::kinesis::PromisePaginatedCoins;
use crate::kinesis::PromisePaginatedEvents;
use crate::kinesis::PromisePaginatedObjectsResponse;
use crate::kinesis::WasmExecuteTransactionBlockParams;
use crate::kinesis::WasmGetCoinsParams;
use crate::kinesis::WasmGetDynamicFieldObjectParams;
use crate::kinesis::WasmGetObjectParams;
use crate::kinesis::WasmGetOwnedObjectsParams;
use crate::kinesis::WasmQueryEventsParams;
use crate::kinesis::WasmTryGetPastObjectParams;

use super::wasm_types::IotaTransactionBlockResponseAdapter;
use super::wasm_types::PromiseIotaTransactionBlockResponse;

// This file contains the wasm-bindgen 'glue code' providing
// the interface of the TS Iota client to rust code.

// The typescript declarations imported in the following typescript_custom_section
// can be use as arguments for rust functions via the typescript_type annotation.
// In other words: The typescript_type "IotaClient" is imported here to be binded
// to the WasmIotaClient functions below.
// TODO: check why this isn't done by `module` macro attribute for `WasmKinesisClient`
#[wasm_bindgen(typescript_custom_section)]
const IOTA_CLIENT_TYPE: &'static str = r#"
  import { IotaClient } from "@iota/iota.js/client";
"#;

#[wasm_bindgen(module = "@iota/iota.js/client")]
extern "C" {
  #[wasm_bindgen(typescript_type = "IotaClient")]
  #[derive(Clone)]
  pub type WasmIotaClient;

  #[wasm_bindgen(method, js_name = getChainIdentifier)]
  pub fn get_chain_identifier(this: &WasmIotaClient) -> PromiseString;

  #[wasm_bindgen(method, js_name = executeTransactionBlock)]
  pub fn execute_transaction_block(
    this: &WasmIotaClient,
    params: &WasmExecuteTransactionBlockParams,
  ) -> PromiseIotaTransactionBlockResponse;

  #[wasm_bindgen(method, js_name = getDynamicFieldObject)]
  pub fn get_dynamic_field_object(
    this: &WasmIotaClient,
    input: &WasmGetDynamicFieldObjectParams,
  ) -> PromiseIotaObjectResponse;

  #[wasm_bindgen(method, js_name = getObject)]
  pub fn get_object(this: &WasmIotaClient, input: &WasmGetObjectParams) -> PromiseIotaObjectResponse;

  #[wasm_bindgen(method, js_name = getOwnedObjects)]
  pub fn get_owned_objects(this: &WasmIotaClient, input: &WasmGetOwnedObjectsParams)
    -> PromisePaginatedObjectsResponse;

  #[wasm_bindgen(method, js_name = getReferenceGasPrice)]
  pub fn get_reference_gas_price(this: &WasmIotaClient) -> PromiseBigint;

  #[wasm_bindgen(method, js_name = tryGetPastObject)]
  pub fn try_get_past_object(this: &WasmIotaClient, input: &WasmTryGetPastObjectParams) -> PromiseObjectRead;

  #[wasm_bindgen(method, js_name = queryEvents)]
  pub fn query_events(this: &WasmIotaClient, input: &WasmQueryEventsParams) -> PromisePaginatedEvents;

  #[wasm_bindgen(method, js_name = getCoins)]
  pub fn get_coins(this: &WasmIotaClient, input: &WasmGetCoinsParams) -> PromisePaginatedCoins;
}

// Helper struct used to convert TYPESCRIPT types to RUST types
#[derive(Clone)]
pub struct ManagedWasmIotaClient(WasmIotaClient);

// convert TYPESCRIPT types to RUST types
impl ManagedWasmIotaClient {
  pub fn new(iota_client: WasmIotaClient) -> Self {
    ManagedWasmIotaClient(iota_client)
  }

  pub async fn get_chain_identifier(&self) -> Result<String, Error> {
    let promise: Promise = Promise::resolve(&WasmIotaClient::get_chain_identifier(&self.0));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }

  pub async fn execute_transaction_block(
    &self,
    tx_data_bcs: &TransactionDataBcs,
    signatures: &Vec<SignatureBcs>,
    options: Option<IotaTransactionBlockResponseOptions>,
    request_type: Option<ExecuteTransactionRequestType>,
  ) -> IotaRpcResult<IotaTransactionBlockResponseAdapter> {
    let ex_tx_params: WasmExecuteTransactionBlockParams = serde_wasm_bindgen::to_value(
      &ExecuteTransactionBlockParams::new(tx_data_bcs, signatures, options, request_type),
    )
    .map_err(|e| {
      console_log!(
        "Error executing serde_wasm_bindgen::to_value(ExecuteTransactionBlockParams): {:?}",
        e
      );
      IotaRpcError::FfiError(format!("{:?}", e))
    })?
    .into();

    let promise: Promise = Promise::resolve(&WasmIotaClient::execute_transaction_block(&self.0, &ex_tx_params));
    let result: JsValue = JsFuture::from(promise).await.map_err(|e| {
      console_log!("Error executing JsFuture::from(promise): {:?}", e);
      IotaRpcError::FfiError(format!("{:?}", e))
    })?;
    Ok(IotaTransactionBlockResponseAdapter::new(result.into()))
  }

  /**
   * Return the dynamic field object information for a specified object
   */
  pub async fn get_dynamic_field_object(
    &self,
    parent_object_id: ObjectID,
    name: DynamicFieldName,
  ) -> IotaRpcResult<IotaObjectResponse> {
    let params: WasmGetDynamicFieldObjectParams =
      serde_wasm_bindgen::to_value(&GetDynamicFieldObjectParams::new(parent_object_id.to_string(), name))
        .map_err(|e| {
          console_log!(
            "Error executing serde_wasm_bindgen::to_value(WasmGetDynamicFieldObjectParams): {:?}",
            e
          );
          IotaRpcError::FfiError(format!("{:?}", e))
        })?
        .into();

    let promise: Promise = Promise::resolve(&WasmIotaClient::get_dynamic_field_object(&self.0, &params));
    let result: JsValue = JsFuture::from(promise).await.map_err(|e| {
      console_log!("Error executing JsFuture::from(promise): {:?}", e);
      IotaRpcError::FfiError(format!("{:?}", e))
    })?;

    Ok(result.into_serde()?)
  }

  pub async fn get_object_with_options(
    &self,
    object_id: ObjectID,
    options: IotaObjectDataOptions,
  ) -> IotaRpcResult<IotaObjectResponse> {
    let params: WasmGetObjectParams =
      serde_wasm_bindgen::to_value(&GetObjectParams::new(object_id.to_string(), Some(options)))
        .map_err(|e| {
          console_log!(
            "Error executing serde_wasm_bindgen::to_value(WasmIotaObjectDataOptions): {:?}",
            e
          );
          IotaRpcError::FfiError(format!("{:?}", e))
        })?
        .into();

    let promise: Promise = Promise::resolve(&WasmIotaClient::get_object(&self.0, &params));
    let result: JsValue = JsFuture::from(promise).await.map_err(|e| {
      console_log!("Error executing JsFuture::from(promise): {:?}", e);
      IotaRpcError::FfiError(format!("{:?}", e))
    })?;

    Ok(result.into_serde()?)
  }

  pub async fn get_owned_objects(
    &self,
    address: IotaAddress,
    query: Option<IotaObjectResponseQuery>,
    cursor: Option<ObjectID>,
    limit: Option<usize>,
  ) -> IotaRpcResult<ObjectsPage> {
    let params: WasmGetOwnedObjectsParams = serde_wasm_bindgen::to_value(&GetOwnedObjectsParams::new(
      address.to_string(),
      cursor.map(|v| v.to_string()),
      limit,
      query.clone().map(|v| v.filter).flatten(),
      query.clone().map(|v| v.options).flatten(),
    ))
    .map_err(|e| {
      console_log!(
        "Error executing serde_wasm_bindgen::to_value(WasmIotaObjectDataOptions): {:?}",
        e
      );
      IotaRpcError::FfiError(format!("{:?}", e))
    })?
    .into();

    let promise: Promise = Promise::resolve(&WasmIotaClient::get_owned_objects(&self.0, &params));
    let result: JsValue = JsFuture::from(promise).await.map_err(|e| {
      console_log!("Error executing JsFuture::from(promise): {:?}", e);
      IotaRpcError::FfiError(format!("{:?}", e))
    })?;

    Ok(result.into_serde()?)
  }

  pub async fn get_reference_gas_price(&self) -> IotaRpcResult<u64> {
    let promise: Promise = Promise::resolve(&WasmIotaClient::get_reference_gas_price(&self.0));
    let result: JsValue = JsFuture::from(promise).await.map_err(|e| {
      console_log!("Error executing JsFuture::from(promise): {:?}", e);
      IotaRpcError::FfiError(format!("{:?}", e))
    })?;

    Ok(result.into_serde()?)
  }

  pub async fn try_get_parsed_past_object(
    &self,
    object_id: ObjectID,
    version: SequenceNumber,
    options: IotaObjectDataOptions,
  ) -> IotaRpcResult<IotaPastObjectResponse> {
    let params: WasmTryGetPastObjectParams = serde_wasm_bindgen::to_value(&TryGetPastObjectParams::new(
      object_id.to_string(),
      version,
      Some(options),
    ))
    .map_err(|e| {
      console_log!(
        "Error executing serde_wasm_bindgen::to_value(WasmIotaObjectDataOptions): {:?}",
        e
      );
      IotaRpcError::FfiError(format!("{:?}", e))
    })?
    .into();

    let promise: Promise = Promise::resolve(&WasmIotaClient::try_get_past_object(&self.0, &params));
    let result: JsValue = JsFuture::from(promise).await.map_err(|e| {
      console_log!("Error executing JsFuture::from(promise): {:?}", e);
      IotaRpcError::FfiError(format!("{:?}", e))
    })?;

    Ok(result.into_serde()?)
  }

  pub async fn query_events(
    &self,
    query: EventFilter,
    cursor: Option<EventID>,
    limit: Option<usize>,
    descending_order: bool,
  ) -> IotaRpcResult<EventPage> {
    let params: WasmQueryEventsParams = serde_wasm_bindgen::to_value(&QueryEventsParams::new(
      query,
      cursor,
      limit,
      Some(SortOrder::new(descending_order)),
    ))
    .map_err(|e| {
      console_log!(
        "Error executing serde_wasm_bindgen::to_value(WasmIotaObjectDataOptions): {:?}",
        e
      );
      IotaRpcError::FfiError(format!("{:?}", e))
    })?
    .into();

    let promise: Promise = Promise::resolve(&WasmIotaClient::query_events(&self.0, &params));
    let result: JsValue = JsFuture::from(promise).await.map_err(|e| {
      console_log!("Error executing JsFuture::from(promise): {:?}", e);
      IotaRpcError::FfiError(format!("{:?}", e))
    })?;

    Ok(result.into_serde()?)
  }

  pub async fn get_coins(
    &self,
    owner: IotaAddress,
    coin_type: Option<String>,
    cursor: Option<ObjectID>,
    limit: Option<usize>,
  ) -> IotaRpcResult<CoinPage> {
    let params: WasmGetCoinsParams = serde_wasm_bindgen::to_value(&GetCoinsParams::new(
      owner.to_string(),
      coin_type.map(|v| v.to_string()),
      cursor.map(|v| v.to_string()),
      limit,
    ))
    .map_err(|e| {
      console_log!(
        "Error executing serde_wasm_bindgen::to_value(WasmIotaObjectDataOptions): {:?}",
        e
      );
      IotaRpcError::FfiError(format!("{:?}", e))
    })?
    .into();

    let promise: Promise = Promise::resolve(&WasmIotaClient::get_coins(&self.0, &params));
    let result: JsValue = JsFuture::from(promise).await.map_err(|e| {
      console_log!("Error executing JsFuture::from(promise): {:?}", e);
      IotaRpcError::FfiError(format!("{:?}", e))
    })?;

    Ok(result.into_serde()?)
  }
}
