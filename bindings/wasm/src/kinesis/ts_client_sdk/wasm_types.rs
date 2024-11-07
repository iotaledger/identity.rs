use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use identity_iota::iota::iota_sdk_abstraction::rpc_types::{IotaExecutionStatus, OwnedObjectRef};
use identity_iota::iota::iota_sdk_abstraction::types::execution_status::ExecutionStatus;
use crate::kinesis::{ManagedWasmIotaClient, WasmIotaClient};
use super::super::types::into_sdk_type;

#[wasm_bindgen(typescript_custom_section)]
const TS_SDK_TYPES: &'static str = r#"
  import {
    Balance,
    IotaObjectData,
    IotaTransactionBlockResponseOptions,
    IotaTransactionBlockResponse,
  }
  from "@iota/iota.js/types";
  
  import {IotaTransactionBlockResponseAdapter} from "./wasm_types"
"#;

#[wasm_bindgen(module = "@iota/iota.js/types")]
extern "C" {
  #[wasm_bindgen(typescript_type = "Promise<Balance>")]
  pub type PromiseBalance;

  #[wasm_bindgen(typescript_type = "IotaObjectData")]
  pub type WasmIotaObjectData;
  
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

#[wasm_bindgen(module = "wasm_types")]
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


impl IotaTransactionBlockResponseAdapter {
  pub fn effects_execution_status(&self) -> Option<ExecutionStatus> {
    self.effects_execution_status_inner()
      .map(into_sdk_type::<ExecutionStatus, WasmExecutionStatus>)
  }
  
  pub   fn effects_created(&self) -> Option<Vec<OwnedObjectRef>> {
    self.effects_created_inner()
      .map(|vex_obj_ref| {
        vex_obj_ref
          .into_iter()
          .map(into_sdk_type::<OwnedObjectRef, WasmOwnedObjectRef>)
          .collect()
      })
  }
}