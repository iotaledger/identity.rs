use serde::Deserialize;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use identity_iota_interaction::rpc_types::{OwnedObjectRef};
use identity_iota_interaction::types::execution_status::ExecutionStatus;
use crate::common::into_sdk_type;

#[wasm_bindgen(typescript_custom_section)]
const TS_SDK_TYPES: &'static str = r#"
  import {
    Balance,
    IotaObjectData,
    ExecuteTransactionBlockParams,
    IotaTransactionBlockResponseOptions,
    IotaTransactionBlockResponse,
  } from "@iota/iota.js/client";
  import {bcs} from "@iota/iota.js/bcs";
  
  import {IotaTransactionBlockResponseAdapter} from "./kinesis_client_helpers";

  // TODO: decide if we use this or replace it with an adapter written in TypeScript type if needed
  type ProgrammableTransaction = ReturnType<typeof bcs.ProgrammableTransaction.parse>;
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

  #[wasm_bindgen(typescript_type = "Promise<IotaTransactionBlockResponse>")]
  #[derive(Clone)]
  pub type PromiseIotaTransactionBlockResponse;

  #[wasm_bindgen(typescript_type = "ExecutionStatus")]
  #[derive(Clone)]
  pub type WasmExecutionStatus;

  #[wasm_bindgen(typescript_type = "OwnedObjectRef")]
  #[derive(Clone)]
  pub type WasmOwnedObjectRef;
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
}

#[wasm_bindgen] // no module here, as imported via custom section above
extern "C" {
  #[wasm_bindgen(typescript_type = "ProgrammableTransaction")]
  pub type WasmProgrammableTransaction;
}

#[derive(Deserialize)]
struct WasmExecutionStatusAdapter {
  status: ExecutionStatus
}

impl IotaTransactionBlockResponseAdapter {
  pub fn effects_execution_status(&self) -> Option<ExecutionStatus> {
    self.effects_execution_status_inner()
      .map(|s| {
        let state: WasmExecutionStatusAdapter = into_sdk_type(s)
            .expect("[IotaTransactionBlockResponseAdapter] Failed to convert WasmExecutionStatus");
        state.status
      })
  }
  
  pub fn effects_created(&self) -> Option<Vec<OwnedObjectRef>> {
    self.effects_created_inner()
      .map(|vex_obj_ref| {
        vex_obj_ref
          .into_iter()
          .map(|obj| into_sdk_type(obj)
              .expect("[IotaTransactionBlockResponseAdapter] Failed to convert WasmOwnedObjectRef"))
          .collect()
      })
  }
}

#[derive(Deserialize)]
// TODO: add manual deserialization later on (must be deserializable, but WasmPT is not)
// pub struct ProgrammableTransaction(WasmProgrammableTransaction);
pub struct ProgrammableTransaction(());

// TODO: fill in required functions and data handling here
impl ProgrammableTransaction {}
