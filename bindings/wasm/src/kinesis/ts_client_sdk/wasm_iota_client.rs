// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::common::PromiseString;
use crate::error::JsValueResult;
use identity_iota::iota::iota_sdk_abstraction::types::base_types::ObjectID;
use identity_iota::iota::iota_sdk_abstraction::types::dynamic_field::DynamicFieldName;
use identity_iota::iota::sui_name_tbd_error::Error;
use identity_iota::iota::iota_sdk_abstraction::{TransactionDataBcs, SignatureBcs};
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use identity_iota::iota::iota_sdk_abstraction::error::{IotaRpcResult, Error as IotaRpcError};
use identity_iota::iota::iota_sdk_abstraction::rpc_types::{IotaObjectDataOptions, IotaObjectResponse, IotaTransactionBlockResponseOptions};
use identity_iota::iota::iota_sdk_abstraction::types::quorum_driver_types::ExecuteTransactionRequestType;
use identity_iota::iota::iota_sdk_abstraction::generated_types::{ExecuteTransactionBlockParams, GetDynamicFieldObjectParams, GetObjectParams};
use crate::kinesis::{PromiseIotaObjectResponse, WasmExecuteTransactionBlockParams, WasmGetDynamicFieldObjectParams, WasmGetObjectParams};
use crate::console_log;
use super::wasm_types::{
  PromiseIotaTransactionBlockResponse,
  IotaTransactionBlockResponseAdapter,
};

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
    params: &WasmExecuteTransactionBlockParams
  ) -> PromiseIotaTransactionBlockResponse;

  #[wasm_bindgen(method, js_name = getDynamicFieldObject)]
  pub fn get_dynamic_field_object(
    this: &WasmIotaClient,
    input: &WasmGetDynamicFieldObjectParams,
  ) -> PromiseIotaObjectResponse;

  #[wasm_bindgen(method, js_name = getObject)]
  pub fn get_object(
    this: &WasmIotaClient,
    input: &WasmGetObjectParams,
  ) -> PromiseIotaObjectResponse;
}

// Helper struct used to convert TYPESCRIPT types to RUST types
#[derive(Clone)]
pub struct ManagedWasmIotaClient(WasmIotaClient);

// convert TYPESCRIPT types to RUST types
impl ManagedWasmIotaClient {
  pub fn new (iota_client: WasmIotaClient) -> Self {
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
      &ExecuteTransactionBlockParams::new(tx_data_bcs, signatures, options, request_type)
    )
    .map_err(|e| {
      console_log!("Error executing serde_wasm_bindgen::to_value(ExecuteTransactionBlockParams): {:?}", e);
      IotaRpcError::FfiError(format!("{:?}", e))
    })?
    .into();

    let promise: Promise = Promise::resolve(
      &WasmIotaClient::execute_transaction_block(&self.0, &ex_tx_params)
    );
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
}

