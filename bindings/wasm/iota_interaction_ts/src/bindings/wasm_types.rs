// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota_interaction::rpc_types::OwnedObjectRef;
use identity_iota_interaction::types::base_types::IotaAddress;
use identity_iota_interaction::types::base_types::ObjectID;
use identity_iota_interaction::types::base_types::ObjectRef;
use identity_iota_interaction::types::base_types::SequenceNumber;
use identity_iota_interaction::types::execution_status::CommandArgumentError;
use identity_iota_interaction::types::execution_status::ExecutionStatus;
use identity_iota_interaction::types::object::Owner;
use identity_iota_interaction::ProgrammableTransactionBcs;
use js_sys::Promise;
use serde::Deserialize;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

use crate::bindings::WasmIotaClient;
use crate::common::into_sdk_type;
use crate::console_log;
use crate::error::TsSdkError;

// TODO: fix/add
// not available anymore
// use crate::storage::WasmStorageSigner;
type WasmStorageSigner = ();
// not available anymore
// use identity_iota::iota::iota_sdk_abstraction::error::Error;
enum Error {
  FfiError(String),
}

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
  } from "@iota/iota.js/client";
  import { bcs } from "@iota/iota.js/bcs";
  import {
    executeTransaction,
    IotaTransactionBlockResponseAdapter,
  } from "./iota_client_helpers"

  // TODO: decide if we use this or replace it with an adapter written in TypeScript type if needed
  type ProgrammableTransaction = ReturnType<typeof bcs.ProgrammableTransaction.parse>;
"#;

#[wasm_bindgen(module = "@iota/iota.js/client")]
extern "C" {
  #[wasm_bindgen(typescript_type = "Promise<Balance>")]
  pub type PromiseBalance;

  #[wasm_bindgen(typescript_type = "Transaction")]
  pub type WasmTransactionBuilder;

  #[wasm_bindgen(typescript_type = "TransactionArgument")]
  pub type WasmTransactionArgument;

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

  #[wasm_bindgen(typescript_type = "ObjectRef")]
  #[derive(Clone)]
  pub type WasmObjectRef;

  #[wasm_bindgen(typescript_type = "SharedObjectRef")]
  #[derive(Clone)]
  pub type WasmSharedObjectRef;

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

impl From<ObjectRef> for WasmObjectRef {
  fn from(value: ObjectRef) -> Self {
    let json_obj = serde_json::json!({
      "objectId": value.0,
      "version": value.1,
      "digest": value.2,
    });

    serde_wasm_bindgen::to_value(&json_obj)
      .expect("a JSON object is a JS value")
      // safety: `json_obj` was constructed following TS ObjectRef's interface.
      .unchecked_into()
  }
}

impl From<(ObjectID, SequenceNumber, bool)> for WasmSharedObjectRef {
  fn from(value: (ObjectID, SequenceNumber, bool)) -> Self {
    let json_obj = serde_json::json!({
      "objectId": value.0,
      "initialSharedVersion": value.1,
      "mutable": value.2,
    });

    serde_wasm_bindgen::to_value(&json_obj)
      .expect("a JSON object is a JS value")
      // safety: `json_obj` was constructed following TS SharedObjectRef's interface.
      .unchecked_into()
  }
}

impl TryFrom<OwnedObjectRef> for WasmSharedObjectRef {
  type Error = TsSdkError;
  fn try_from(value: OwnedObjectRef) -> Result<Self, Self::Error> {
    let Owner::Shared { initial_shared_version } = value.owner else {
      return Err(TsSdkError::CommandArgumentError(CommandArgumentError::TypeMismatch));
    };
    let obj_id = value.object_id();

    Ok((obj_id, initial_shared_version, true).into())
  }
}

impl WasmSharedObjectRef {
  #[allow(dead_code)]
  pub(crate) fn immutable(self) -> Self {
    const JS_FALSE: JsValue = JsValue::from_bool(false);

    let _ = js_sys::Reflect::set(&self, &JsValue::from_str("mutable"), &JS_FALSE);
    self
  }
}

#[wasm_bindgen(module = "/lib/iota_client_helpers.ts")]
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
    signer: WasmStorageSigner,   // --> TypeScript: Signer (iota_client_helpers module)
    gas_budget: Option<u64>,     // --> TypeScript: optional bigint
  ) -> PromiseIotaTransactionBlockResponseAdapter;
}

#[wasm_bindgen] // no module here, as imported via custom section above
extern "C" {
  #[wasm_bindgen(typescript_type = "ProgrammableTransaction")]
  pub type WasmProgrammableTransaction;
}

#[derive(Deserialize)]
struct WasmExecutionStatusAdapter {
  status: ExecutionStatus,
}

impl IotaTransactionBlockResponseAdapter {
  pub fn effects_execution_status(&self) -> Option<ExecutionStatus> {
    self.effects_execution_status_inner().map(|s| {
      let state: WasmExecutionStatusAdapter =
        into_sdk_type(s).expect("[IotaTransactionBlockResponseAdapter] Failed to convert WasmExecutionStatus");
      state.status
    })
  }

  pub fn effects_created(&self) -> Option<Vec<OwnedObjectRef>> {
    self.effects_created_inner().map(|vex_obj_ref| {
      vex_obj_ref
        .into_iter()
        .map(|obj| {
          into_sdk_type(obj).expect("[IotaTransactionBlockResponseAdapter] Failed to convert WasmOwnedObjectRef")
        })
        .collect()
    })
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

#[derive(Deserialize)]
// TODO: add manual deserialization later on (must be deserializable, but WasmPT is not)
// pub struct ProgrammableTransaction(WasmProgrammableTransaction);
pub struct ProgrammableTransaction(());

// TODO: fill in required functions and data handling here
impl ProgrammableTransaction {}
