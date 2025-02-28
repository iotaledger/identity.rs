// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota_interaction::error::Error as IotaRpcError;
use identity_iota_interaction::error::IotaRpcResult;
use identity_iota_interaction::generated_types::ExecuteTransactionBlockParams;
use identity_iota_interaction::generated_types::GetCoinsParams;
use identity_iota_interaction::generated_types::GetDynamicFieldObjectParams;
use identity_iota_interaction::generated_types::GetObjectParams;
use identity_iota_interaction::generated_types::GetOwnedObjectsParams;
use identity_iota_interaction::generated_types::GetTransactionBlockParams;
use identity_iota_interaction::generated_types::QueryEventsParams;
use identity_iota_interaction::generated_types::SortOrder;
use identity_iota_interaction::generated_types::WaitForTransactionParams;
use identity_iota_interaction::rpc_types::CoinPage;
use identity_iota_interaction::rpc_types::EventFilter;
use identity_iota_interaction::rpc_types::EventPage;
use identity_iota_interaction::rpc_types::IotaObjectDataOptions;
use identity_iota_interaction::rpc_types::IotaObjectResponse;
use identity_iota_interaction::rpc_types::IotaObjectResponseQuery;
use identity_iota_interaction::rpc_types::IotaPastObjectResponse;
use identity_iota_interaction::rpc_types::IotaTransactionBlockResponseOptions;
use identity_iota_interaction::rpc_types::ObjectsPage;
use identity_iota_interaction::types::base_types::IotaAddress;
use identity_iota_interaction::types::base_types::ObjectID;
use identity_iota_interaction::types::base_types::SequenceNumber;
use identity_iota_interaction::types::digests::TransactionDigest;
use identity_iota_interaction::types::dynamic_field::DynamicFieldName;
use identity_iota_interaction::types::event::EventID;
use identity_iota_interaction::types::quorum_driver_types::ExecuteTransactionRequestType;
use identity_iota_interaction::SignatureBcs;
use identity_iota_interaction::TransactionDataBcs;
use js_sys::Promise;
use serde::Serialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

use super::wasm_types::PromiseIotaTransactionBlockResponse;
use super::wasm_types::WasmExecuteTransactionBlockParams;
use super::wasm_types::WasmIotaTransactionBlockResponseWrapper;
use super::WasmWaitForTransactionParams;

use crate::bindings::PromiseIotaObjectResponse;
use crate::bindings::PromiseObjectRead;
use crate::bindings::PromisePaginatedCoins;
use crate::bindings::PromisePaginatedEvents;
use crate::bindings::PromisePaginatedObjectsResponse;
use crate::bindings::WasmGetCoinsParams;
use crate::bindings::WasmGetDynamicFieldObjectParams;
use crate::bindings::WasmGetObjectParams;
use crate::bindings::WasmGetOwnedObjectsParams;
use crate::bindings::WasmGetTransactionBlockParams;
use crate::bindings::WasmQueryEventsParams;
use crate::bindings::WasmTryGetPastObjectParams;
use crate::common::types::PromiseString;
use crate::common::PromiseBigint;
use crate::console_log;
use crate::error::into_ts_sdk_result;
use crate::error::TsSdkError;

// This file contains the wasm-bindgen 'glue code' providing
// the interface of the TS Iota client to rust code.

// The typescript declarations imported in the following typescript_custom_section
// can be used as arguments for rust functions via the typescript_type annotation.
// In other words: The typescript_type "IotaClient" is imported here to be bound
// to the WasmIotaClient functions below.
// TODO: check why this isn't done by `module` macro attribute for `WasmIotaClient`
#[wasm_bindgen(typescript_custom_section)]
const IOTA_CLIENT_TYPE: &'static str = r#"
  import { IotaClient } from "@iota/iota-sdk/client";
"#;

#[wasm_bindgen(module = "@iota/iota-sdk/client")]
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

  #[wasm_bindgen(method, js_name = getTransactionBlock)]
  pub fn get_transaction_block(
    this: &WasmIotaClient,
    input: &WasmGetTransactionBlockParams,
  ) -> PromiseIotaTransactionBlockResponse;

  #[wasm_bindgen(method, js_name = getReferenceGasPrice)]
  pub fn get_reference_gas_price(this: &WasmIotaClient) -> PromiseBigint;

  #[wasm_bindgen(method, js_name = tryGetPastObject)]
  pub fn try_get_past_object(this: &WasmIotaClient, input: &WasmTryGetPastObjectParams) -> PromiseObjectRead;

  #[wasm_bindgen(method, js_name = queryEvents)]
  pub fn query_events(this: &WasmIotaClient, input: &WasmQueryEventsParams) -> PromisePaginatedEvents;

  #[wasm_bindgen(method, js_name = getCoins)]
  pub fn get_coins(this: &WasmIotaClient, input: &WasmGetCoinsParams) -> PromisePaginatedCoins;

  #[wasm_bindgen(method, js_name = waitForTransaction)]
  pub fn wait_for_transaction(
    this: &WasmIotaClient,
    input: &WasmWaitForTransactionParams,
  ) -> PromiseIotaTransactionBlockResponse;
}

// Helper struct used to convert TYPESCRIPT types to RUST types
#[derive(Clone)]
pub struct ManagedWasmIotaClient(pub(crate) WasmIotaClient);

// convert TYPESCRIPT types to RUST types
impl ManagedWasmIotaClient {
  pub fn new(iota_client: WasmIotaClient) -> Self {
    ManagedWasmIotaClient(iota_client)
  }

  pub async fn get_chain_identifier(&self) -> Result<String, TsSdkError> {
    let promise: Promise = Promise::resolve(&WasmIotaClient::get_chain_identifier(&self.0));
    into_ts_sdk_result(JsFuture::from(promise).await)
  }

  pub async fn execute_transaction_block(
    &self,
    tx_data_bcs: &TransactionDataBcs,
    signatures: &[SignatureBcs],
    options: Option<IotaTransactionBlockResponseOptions>,
    request_type: Option<ExecuteTransactionRequestType>,
  ) -> IotaRpcResult<WasmIotaTransactionBlockResponseWrapper> {
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
    Ok(WasmIotaTransactionBlockResponseWrapper::new(result.into()))
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

    #[allow(deprecated)] // will be refactored
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

    #[allow(deprecated)] // will be refactored
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

    #[allow(deprecated)] // will be refactored
    Ok(result.into_serde()?)
  }

  pub async fn get_transaction_with_options(
    &self,
    digest: TransactionDigest,
    options: IotaTransactionBlockResponseOptions,
  ) -> IotaRpcResult<WasmIotaTransactionBlockResponseWrapper> {
    let params: WasmGetTransactionBlockParams =
      serde_wasm_bindgen::to_value(&GetTransactionBlockParams::new(digest.to_string(), Some(options)))
        .map_err(|e| {
          console_log!(
            "Error executing serde_wasm_bindgen::to_value(WasmIotaObjectDataOptions): {:?}",
            e
          );
          IotaRpcError::FfiError(format!("{:?}", e))
        })?
        .into();

    // Rust `ReadApi::get_transaction_with_options` calls `get_transaction_block` via http while
    // TypeScript uses the name `getTransactionBlock` directly, so we have to call this function
    let promise: Promise = Promise::resolve(&WasmIotaClient::get_transaction_block(&self.0, &params));
    let result: JsValue = JsFuture::from(promise).await.map_err(|e| {
      console_log!("Error executing JsFuture::from(promise): {:?}", e);
      IotaRpcError::FfiError(format!("{:?}", e))
    })?;

    Ok(WasmIotaTransactionBlockResponseWrapper::new(result.into()))
  }

  pub async fn get_reference_gas_price(&self) -> IotaRpcResult<u64> {
    let promise: Promise = Promise::resolve(&WasmIotaClient::get_reference_gas_price(&self.0));
    let result: JsValue = JsFuture::from(promise).await.map_err(|e| {
      console_log!("Error executing JsFuture::from(promise): {:?}", e);
      IotaRpcError::FfiError(format!("{:?}", e))
    })?;

    #[allow(deprecated)] // will be refactored
    Ok(result.into_serde()?)
  }

  pub async fn try_get_parsed_past_object(
    &self,
    _object_id: ObjectID,
    _version: SequenceNumber,
    _options: IotaObjectDataOptions,
  ) -> IotaRpcResult<IotaPastObjectResponse> {
    // TODO: does not work anymore, find out, why we need to pass a different `SequenceNumber` now
    unimplemented!("try_get_parsed_past_object");
    // let params: WasmTryGetPastObjectParams = serde_wasm_bindgen::to_value(&TryGetPastObjectParams::new(
    //   object_id.to_string(),
    //   version,
    //   Some(options),
    // ))
    // .map_err(|e| {
    //   console_log!(
    //     "Error executing serde_wasm_bindgen::to_value(WasmIotaObjectDataOptions): {:?}",
    //     e
    //   );
    //   IotaRpcError::FfiError(format!("{:?}", e))
    // })?
    // .into();

    // let promise: Promise = Promise::resolve(&WasmIotaClient::try_get_past_object(&self.0, &params));
    // let result: JsValue = JsFuture::from(promise).await.map_err(|e| {
    //   console_log!("Error executing JsFuture::from(promise): {:?}", e);
    //   IotaRpcError::FfiError(format!("{:?}", e))
    // })?;

    // Ok(result.into_serde()?)
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

    #[allow(deprecated)] // will be refactored
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

    #[allow(deprecated)] // will be refactored
    Ok(result.into_serde()?)
  }

  /// Wait for a transaction block result to be available over the API.
  /// This can be used in conjunction with `execute_transaction_block` to wait for the transaction to
  /// be available via the API.
  /// This currently polls the `getTransactionBlock` API to check for the transaction.
  ///
  /// # Arguments
  ///
  /// * `digest` - The digest of the queried transaction.
  /// * `options` - Options for specifying the content to be returned.
  /// * `timeout` - The amount of time to wait for a transaction block. Defaults to one minute.
  /// * `poll_interval` - The amount of time to wait between checks for the transaction block. Defaults to 2 seconds.
  pub async fn wait_for_transaction(
    &self,
    digest: TransactionDigest,
    options: Option<IotaTransactionBlockResponseOptions>,
    timeout: Option<u64>,
    poll_interval: Option<u64>,
  ) -> IotaRpcResult<WasmIotaTransactionBlockResponseWrapper> {
    let params_object = WaitForTransactionParams::new(digest.to_string(), options, timeout, poll_interval);
    let params: WasmWaitForTransactionParams = serde_json::to_value(&params_object)
      .map_err(|e| {
        console_log!("Error serializing WaitForTransactionParams to Value: {:?}", e);
        IotaRpcError::FfiError(format!("{:?}", e))
      })
      .and_then(|v| {
        v.serialize(&serde_wasm_bindgen::Serializer::json_compatible())
          .map_err(|e| {
            console_log!("Error serializing Value to WasmWaitForTransactionParams: {:?}", e);
            IotaRpcError::FfiError(format!("{:?}", e))
          })
      })?
      .into();

    let promise: Promise = Promise::resolve(&WasmIotaClient::wait_for_transaction(&self.0, &params));
    let result: JsValue = JsFuture::from(promise).await.map_err(|e| {
      console_log!("Error executing JsFuture::from(promise): {:?}", e);
      IotaRpcError::FfiError(format!("{:?}", e))
    })?;

    Ok(WasmIotaTransactionBlockResponseWrapper::new(result.into()))
  }
}
