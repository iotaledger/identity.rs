// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::iota::iota_sdk_abstraction::error::Error;
use identity_iota::iota::iota_sdk_abstraction::rpc_types::OwnedObjectRef;
use identity_iota::iota::iota_sdk_abstraction::types::base_types::IotaAddress;
use identity_iota::iota::iota_sdk_abstraction::types::execution_status::ExecutionStatus;
use identity_iota::iota::iota_sdk_abstraction::ProgrammableTransactionBcs;
use js_sys::Promise;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

use crate::storage::WasmStorageSigner;

use super::super::types::into_sdk_type;
use super::WasmIotaClient;

#[wasm_bindgen(typescript_custom_section)]
const TS_SDK_TYPES: &'static str = r#"
  import {
    Balance,
    ExecuteTransactionBlockParams,
    GetCoinsParams,
    GetDynamicFieldObjectParams,
    GetObjectParams,
    GetOwnedObjectsParams,
    GetTransactionBlockParams,
    IotaObjectData,
    IotaObjectResponse,
    IotaTransactionBlockResponse,
    IotaTransactionBlockResponseOptions,
    ObjectRead,
    PaginatedCoins,
    PaginatedEvents,
    PaginatedObjectsResponse,
    QueryEventsParams,
    TryGetPastObjectParams,
  }
  from "@iota/iota.js/client";
  
  import {
    executeTransaction,
    IotaTransactionBlockResponseAdapter,
  } from "./kinesis_client_helpers"
"#;

#[wasm_bindgen(module = "@iota/iota.js/client")]
extern "C" {
  #[wasm_bindgen(typescript_type = "Promise<Balance>")]
  pub type PromiseBalance;

  #[wasm_bindgen(typescript_type = "IotaObjectData")]
  pub type WasmIotaObjectData;

  #[wasm_bindgen(typescript_type = "ExecuteTransactionBlockParams")]
  #[derive(Clone)]
  pub type WasmExecuteTransactionBlockParams;

  #[wasm_bindgen(typescript_type = "IotaTransactionBlockResponseOptions")]
  #[derive(Clone)]
  pub type WasmIotaTransactionBlockResponseOptions;

  #[wasm_bindgen(typescript_type = "IotaTransactionBlockResponse")]
  #[derive(Clone)]
  pub type WasmIotaTransactionBlockResponse;

  #[wasm_bindgen(typescript_type = "GetDynamicFieldObjectParams")]
  #[derive(Clone)]
  pub type WasmGetDynamicFieldObjectParams;

  #[wasm_bindgen(typescript_type = "GetObjectParams")]
  #[derive(Clone)]
  pub type WasmGetObjectParams;

  #[wasm_bindgen(typescript_type = "Promise<IotaTransactionBlockResponse>")]
  #[derive(Clone)]
  pub type PromiseIotaTransactionBlockResponse;

  #[wasm_bindgen(typescript_type = "Promise<IotaObjectResponse>")]
  #[derive(Clone)]
  pub type PromiseIotaObjectResponse;

  #[wasm_bindgen(typescript_type = "GetOwnedObjectsParams")]
  #[derive(Clone)]
  pub type WasmGetOwnedObjectsParams;

  #[wasm_bindgen(typescript_type = "GetTransactionBlockParams")]
  #[derive(Clone)]
  pub type WasmGetTransactionBlockParams;

  #[wasm_bindgen(typescript_type = "Promise<PaginatedObjectsResponse>")]
  #[derive(Clone)]
  pub type PromisePaginatedObjectsResponse;

  #[wasm_bindgen(typescript_type = "TryGetPastObjectParams")]
  #[derive(Clone)]
  pub type WasmTryGetPastObjectParams;

  #[wasm_bindgen(typescript_type = "Promise<ObjectRead>")]
  #[derive(Clone)]
  pub type PromiseObjectRead;

  #[wasm_bindgen(typescript_type = "ExecutionStatus")]
  #[derive(Clone)]
  pub type WasmExecutionStatus;

  #[wasm_bindgen(typescript_type = "OwnedObjectRef")]
  #[derive(Clone)]
  pub type WasmOwnedObjectRef;

  #[wasm_bindgen(typescript_type = "QueryEventsParams")]
  #[derive(Clone)]
  pub type WasmQueryEventsParams;

  #[wasm_bindgen(typescript_type = "Promise<PaginatedEvents>")]
  #[derive(Clone)]
  pub type PromisePaginatedEvents;

  #[wasm_bindgen(typescript_type = "GetCoinsParams")]
  #[derive(Clone)]
  pub type WasmGetCoinsParams;

  #[wasm_bindgen(typescript_type = "Promise<PaginatedCoins>")]
  #[derive(Clone)]
  pub type PromisePaginatedCoins;

  #[wasm_bindgen(typescript_type = "Promise<IotaTransactionBlockResponseAdapter>")]
  #[derive(Clone)]
  pub type PromiseIotaTransactionBlockResponseAdapter;
}

#[wasm_bindgen(module = "/lib/kinesis_client_helpers.ts")]
extern "C" {
  #[wasm_bindgen(typescript_type = "IotaTransactionBlockResponseAdapter")]
  #[derive(Clone)]
  pub type IotaTransactionBlockResponseAdapter;

  #[wasm_bindgen(constructor)]
  pub fn new(response: WasmIotaTransactionBlockResponse) -> IotaTransactionBlockResponseAdapter;

  #[wasm_bindgen(method)]
  pub fn effects_is_none(this: &IotaTransactionBlockResponseAdapter) -> bool;

  #[wasm_bindgen(method)]
  pub fn effects_is_some(this: &IotaTransactionBlockResponseAdapter) -> bool;

  #[wasm_bindgen(method)]
  pub fn to_string(this: &IotaTransactionBlockResponseAdapter) -> String;

  #[wasm_bindgen(method)]
  fn effects_execution_status_inner(this: &IotaTransactionBlockResponseAdapter) -> Option<WasmExecutionStatus>;

  #[wasm_bindgen(method)]
  fn effects_created_inner(this: &IotaTransactionBlockResponseAdapter) -> Option<Vec<WasmOwnedObjectRef>>;

  #[wasm_bindgen(js_name = executeTransaction)]
  fn execute_transaction_inner(
    iotaClient: &WasmIotaClient, // --> TypeScript: IotaClient
    sender_address: String,      // --> TypeScript: string
    sender_public_key: Vec<u8>,  // --> TypeScript: Uint8Array
    tx_bcs: Vec<u8>,             // --> TypeScript: Uint8Array,
    signer: WasmStorageSigner,   // --> TypeScript: Signer (kinesis_client_helpers module)
    gas_budget: Option<u64>,     // --> TypeScript: optional bigint
  ) -> PromiseIotaTransactionBlockResponseAdapter;
}

/// Note that Rust and TypeScript have a different structure for the status:
/// - TypeScript's version is an object with two properties (`status`: string union and error, string)
/// - Rust's version is an enum with success and error variant
#[derive(Deserialize)]
struct WasmExecutionStatusAdapter {
  status: ExecutionStatus,
}

impl IotaTransactionBlockResponseAdapter {
  pub fn effects_execution_status(&self) -> Option<ExecutionStatus> {
    self.effects_execution_status_inner().map(|s| {
      let state: WasmExecutionStatusAdapter = into_sdk_type(s).unwrap();
      state.status
    })
  }

  pub fn effects_created(&self) -> Option<Vec<OwnedObjectRef>> {
    self
      .effects_created_inner()
      .map(|vex_obj_ref| vex_obj_ref.into_iter().map(|obj| into_sdk_type(obj).unwrap()).collect())
  }
}

pub async fn execute_transaction(
  iota_client: &WasmIotaClient,       // --> Binding: WasmIotaClient
  sender_address: IotaAddress,        // --> Binding: String
  sender_public_key: &[u8],           // --> Binding: Vec<u8>
  tx_bcs: ProgrammableTransactionBcs, // --> Binding: Vec<u8>
  signer: WasmStorageSigner,          // --> Binding: WasmStorageSigner
  gas_budget: Option<u64>,            // --> Binding: Option<u64>,
) -> Result<IotaTransactionBlockResponseAdapter, Error> {
  let promise: Promise = Promise::resolve(&execute_transaction_inner(
    iota_client,
    sender_address.to_string(),
    sender_public_key.to_vec(),
    tx_bcs,
    signer,
    gas_budget,
  ));
  let result: JsValue = JsFuture::from(promise).await.map_err(|e| {
    console_log!("Error executing JsFuture::from(promise): {:?}", e);
    Error::FfiError(format!("{:?}", e))
  })?;

  Ok(IotaTransactionBlockResponseAdapter::new(result.into()))
}
