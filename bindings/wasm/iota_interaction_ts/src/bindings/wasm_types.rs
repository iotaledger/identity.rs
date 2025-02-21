// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use fastcrypto::traits::EncodeDecodeBase64;
use identity_iota_interaction::rpc_types::OwnedObjectRef;
use identity_iota_interaction::types::base_types::IotaAddress;
use identity_iota_interaction::types::base_types::ObjectID;
use identity_iota_interaction::types::base_types::ObjectRef;
use identity_iota_interaction::types::base_types::SequenceNumber;
use identity_iota_interaction::types::crypto::PublicKey;
use identity_iota_interaction::types::crypto::Signature;
use identity_iota_interaction::types::execution_status::CommandArgumentError;
use identity_iota_interaction::types::execution_status::ExecutionStatus;
use identity_iota_interaction::types::object::Owner;
use identity_iota_interaction::ProgrammableTransactionBcs;
use js_sys::Promise;
use js_sys::Uint8Array;
use serde::Deserialize;
use serde::Serialize;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsError;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

use crate::bindings::WasmIotaClient;
use crate::common::into_sdk_type;
use crate::common::PromiseUint8Array;
use crate::console_log;
use crate::error::TsSdkError;
use crate::error::WasmError;
use crate::error::WasmResult;

// TODO: fix/add signer or remove functions relying on it
type WasmStorageSigner = ();

#[wasm_bindgen(typescript_custom_section)]
const TS_SDK_TYPES: &str = r#"
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
  } from "@iota/iota-sdk/client";
  import { bcs } from "@iota/iota-sdk/bcs";
  import {
    executeTransaction,
    IotaTransactionBlockResponseAdapter,
  } from "./iota_client_helpers"
"#;

#[wasm_bindgen(module = "@iota/iota-sdk/client")]
extern "C" {
  #[wasm_bindgen(typescript_type = "Promise<Balance>")]
  pub type PromiseBalance;

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

  #[wasm_bindgen(typescript_type = "Signature")]
  pub type WasmIotaSignature;
}

impl TryFrom<Signature> for WasmIotaSignature {
  type Error = JsValue;
  fn try_from(sig: Signature) -> Result<Self, Self::Error> {
    let js_value = serde_wasm_bindgen::to_value(&sig)?;
    js_value.dyn_into()
  }
}

impl TryFrom<WasmIotaSignature> for Signature {
  type Error = JsValue;
  fn try_from(sig: WasmIotaSignature) -> Result<Self, Self::Error> {
    serde_wasm_bindgen::from_value(sig.into()).wasm_result()
  }
}

#[wasm_bindgen(module = "@iota/iota-sdk/transactions")]
extern "C" {
  #[wasm_bindgen(typescript_type = "Transaction")]
  pub type WasmTransactionBuilder;

  #[wasm_bindgen(js_name = "from", js_class = "Transaction", static_method_of = WasmTransactionBuilder, catch)]
  pub fn from_bcs_bytes(bytes: Uint8Array) -> Result<WasmTransactionBuilder, JsValue>;

  #[wasm_bindgen(method, structural, catch)]
  pub async fn build(this: &WasmTransactionBuilder) -> Result<Uint8Array, JsValue>;

  // TODO: decide if we need the following functions: "yagni" or not?

  // #[wasm_bindgen(js_name = "setSender", method, catch)]
  // pub fn set_sender(this: &WasmTransactionBuilder, address: String) -> Result<(), JsValue>;

  // #[wasm_bindgen(js_name = "setGasOwner", method, catch)]
  // pub fn set_gas_owner(this: &WasmTransactionBuilder, address: String) -> Result<(), JsValue>;

  // #[wasm_bindgen(js_name = "setGasPrice", method, catch)]
  // pub fn set_gas_price(this: &WasmTransactionBuilder, price: u64) -> Result<(), JsValue>;

  // #[wasm_bindgen(js_name = "setGasPayment", method, catch)]
  // pub fn set_gas_payment(this: &WasmTransactionBuilder, payments: Vec<WasmObjectRef>) -> Result<(), JsValue>;

  // #[wasm_bindgen(js_name = "setGasBudget", method, catch)]
  // pub fn set_gas_budget(this: &WasmTransactionBuilder, budget: u64) -> Result<(), JsValue>;

  // #[wasm_bindgen(js_name = "getData", method, catch)]
  // pub fn get_data(this: &WasmTransactionBuilder) -> Result<JsValue, JsValue>;
}

#[wasm_bindgen(module = "@iota/iota-sdk/cryptography")]
extern "C" {
  #[wasm_bindgen(typescript_type = PublicKey)]
  pub type WasmPublicKey;

  #[wasm_bindgen(js_name = toIotaPublicKey, method)]
  pub fn to_iota_public_key(this: &WasmPublicKey) -> String;
}

#[wasm_bindgen(module = "@iota/iota-sdk/keypairs/ed25519")]
extern "C" {
  pub type Ed25519PublicKey;

  #[wasm_bindgen(constructor, catch)]
  pub fn new_ed25519_pk(bytes: &[u8]) -> Result<Ed25519PublicKey, JsValue>;
}

#[wasm_bindgen(module = "@iota/iota-sdk/keypairs/secp256r1")]
extern "C" {
  pub type Secp256r1PublicKey;

  #[wasm_bindgen(constructor, catch)]
  pub fn new_secp256r1_pk(bytes: &[u8]) -> Result<Secp256r1PublicKey, JsValue>;
}

#[wasm_bindgen(module = "@iota/iota-sdk/keypairs/secp256k1")]
extern "C" {
  pub type Secp256k1PublicKey;

  #[wasm_bindgen(constructor, catch)]
  pub fn new_secp256k1_pk(bytes: &[u8]) -> Result<Secp256k1PublicKey, JsValue>;
}

impl TryFrom<&'_ PublicKey> for WasmPublicKey {
  type Error = JsValue;
  fn try_from(pk: &PublicKey) -> Result<Self, Self::Error> {
    let pk_bytes = pk.as_ref();
    let wasm_pk = match pk {
      PublicKey::Ed25519(_) => Ed25519PublicKey::new_ed25519_pk(pk_bytes)?.unchecked_into(),
      PublicKey::Secp256r1(_) => Secp256r1PublicKey::new_secp256r1_pk(pk_bytes)?.unchecked_into(),
      PublicKey::Secp256k1(_) => Secp256k1PublicKey::new_secp256k1_pk(pk_bytes)?.unchecked_into(),
      _ => return Err(JsError::new("unsupported PublicKey type").into()),
    };

    Ok(wasm_pk)
  }
}

impl TryFrom<WasmPublicKey> for PublicKey {
  type Error = JsValue;
  fn try_from(pk: WasmPublicKey) -> Result<Self, Self::Error> {
    let base64_pk = pk.to_iota_public_key();
    PublicKey::decode_base64(&base64_pk).map_err(|e| JsError::new(&e.to_string()).into())
  }
}

impl From<ObjectRef> for WasmObjectRef {
  fn from(value: ObjectRef) -> Self {
    let json_obj = serde_json::json!({
      "objectId": value.0,
      "version": value.1,
      "digest": value.2,
    });

    json_obj
      .serialize(&serde_wasm_bindgen::Serializer::json_compatible())
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

    json_obj
      .serialize(&serde_wasm_bindgen::Serializer::json_compatible())
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

  #[wasm_bindgen(method, js_name = "get_response")]
  fn response(this: &IotaTransactionBlockResponseAdapter) -> WasmIotaTransactionBlockResponse;

  #[wasm_bindgen(js_name = executeTransaction)]
  fn execute_transaction_inner(
    iota_client: &WasmIotaClient, // --> TypeScript: IotaClient
    sender_address: String,       // --> TypeScript: string
    tx_bcs: Vec<u8>,              // --> TypeScript: Uint8Array,
    signer: WasmStorageSigner,    // --> TypeScript: Signer (iota_client_helpers module)
    gas_budget: Option<u64>,      // --> TypeScript: optional bigint
  ) -> PromiseIotaTransactionBlockResponseAdapter;

  #[wasm_bindgen(js_name = "addGasDataToTransaction")]
  fn add_gas_data_to_transaction_inner(
    iota_client: &WasmIotaClient, // --> TypeScript: IotaClient
    sender_address: String,       // --> TypeScript: string
    tx_bcs: Vec<u8>,              // --> TypeScript: Uint8Array,
    gas_budget: Option<u64>,      // --> TypeScript: optional bigint
  ) -> PromiseUint8Array;

  #[wasm_bindgen(js_name = "sleep")]
  fn sleep_inner(ms: i32) -> Promise;
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
pub(crate) async fn add_gas_data_to_transaction(
  iota_client: &WasmIotaClient,
  sender_address: IotaAddress,
  tx_bcs: Vec<u8>,
  gas_budget: Option<u64>,
) -> Result<Vec<u8>, TsSdkError> {
  let promise: Promise = Promise::resolve(&add_gas_data_to_transaction_inner(
    iota_client,
    sender_address.to_string(),
    tx_bcs,
    gas_budget,
  ));
  let value: JsValue = JsFuture::from(promise).await.map_err(|e| {
    let message = "Error executing JsFuture::from(promise) for `add_gas_data_to_transaction`";
    let details = format!("{e:?}");
    console_log!("{message}; {details}");
    TsSdkError::WasmError(message.to_string(), details.to_string())
  })?;

  Ok(Uint8Array::new(&value).to_vec())
}

///  Helper function to pause execution.
pub async fn sleep(duration_ms: i32) -> Result<(), JsValue> {
  let promise = sleep_inner(duration_ms);
  let js_fut = JsFuture::from(promise);
  js_fut.await?;
  Ok(())
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
  tx_bcs: ProgrammableTransactionBcs, // --> Binding: Vec<u8>
  signer: WasmStorageSigner,          // --> Binding: WasmStorageSigner
  gas_budget: Option<u64>,            // --> Binding: Option<u64>,
) -> Result<IotaTransactionBlockResponseAdapter, TsSdkError> {
  let promise: Promise = Promise::resolve(&execute_transaction_inner(
    iota_client,
    sender_address.to_string(),
    tx_bcs,
    signer,
    gas_budget,
  ));
  let result: JsValue = JsFuture::from(promise).await.map_err(|e| {
    let message = "Error executing JsFuture::from(promise) for `execute_transaction`";
    let details = format!("{e:?}");
    console_log!("{message}; {details}");
    TsSdkError::WasmError(message.to_string(), details.to_string())
  })?;

  Ok(IotaTransactionBlockResponseAdapter::new(result.into()))
}

#[derive(Deserialize)]
#[serde(try_from = "Vec<u8>")]
pub struct ProgrammableTransaction(pub(crate) WasmTransactionBuilder);
impl TryFrom<Vec<u8>> for ProgrammableTransaction {
  type Error = TsSdkError;
  fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
    let uint8array: Uint8Array = value.as_slice().into();
    WasmTransactionBuilder::from_bcs_bytes(uint8array)
      .map(Self)
      .map_err(WasmError::from)
      .map_err(TsSdkError::from)
  }
}
