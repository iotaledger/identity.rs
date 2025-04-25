// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Object;
use iota_interaction::types::transaction::TransactionKind;
use iota_interaction::ProgrammableTransactionBcs;
use js_sys::Promise;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

use crate::error::TsSdkError;
use crate::error::TsSdkResult;
use crate::error::WasmError;
use crate::error::WasmResult;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "Promise<void>")]
  pub type PromiseVoid;

  #[wasm_bindgen(typescript_type = "Promise<bigint>")]
  pub type PromiseBigint;

  #[wasm_bindgen(typescript_type = "Promise<boolean>")]
  pub type PromiseBool;

  #[wasm_bindgen(typescript_type = "Promise<string>")]
  pub type PromiseString;

  #[wasm_bindgen(typescript_type = "Promise<string | null>")]
  pub type PromiseOptionString;

  #[wasm_bindgen(typescript_type = "Promise<Uint8Array>")]
  pub type PromiseUint8Array;

  #[wasm_bindgen(typescript_type = "Array<string>")]
  pub type ArrayString;

  #[wasm_bindgen(typescript_type = "Map<string, any>")]
  pub type MapStringAny;

  #[wasm_bindgen(typescript_type = "Record<string, any>")]
  pub type RecordStringAny;

  #[wasm_bindgen(typescript_type = "number | number[]")]
  pub type UOneOrManyNumber;

  #[wasm_bindgen(typescript_type = "string | string[] | null")]
  pub type OptionOneOrManyString;

  #[wasm_bindgen(typescript_type = "VerificationMethod[]")]
  pub type ArrayVerificationMethod;

  #[wasm_bindgen(typescript_type = "Array<DIDUrl | VerificationMethod>")]
  pub type ArrayCoreMethodRef;

  #[wasm_bindgen(typescript_type = "DIDUrl | string")]
  pub type UDIDUrlQuery;

  #[wasm_bindgen(typescript_type = "Service[]")]
  pub type ArrayService;
}

impl TryFrom<Object> for MapStringAny {
  type Error = JsValue;

  fn try_from(properties: Object) -> Result<Self, Self::Error> {
    MapStringAny::try_from(&properties)
  }
}

impl TryFrom<&Object> for MapStringAny {
  type Error = JsValue;

  fn try_from(properties: &Object) -> Result<Self, Self::Error> {
    let map: js_sys::Map = js_sys::Map::new();
    for (key, value) in properties.iter() {
      map.set(
        &JsValue::from_str(key.as_str()),
        #[allow(deprecated)] // will be refactored
        &JsValue::from_serde(&value).wasm_result()?,
      );
    }
    Ok(map.unchecked_into::<MapStringAny>())
  }
}

impl Default for MapStringAny {
  fn default() -> Self {
    js_sys::Map::new().unchecked_into()
  }
}

impl PromiseUint8Array {
  /// Helper function to convert Uint8 arrays from contract calls to the internal `ProgrammableTransactionBcs` type.
  pub async fn to_programmable_transaction_bcs(&self) -> TsSdkResult<ProgrammableTransactionBcs> {
    let promise: Promise = Promise::resolve(self);
    let tx_kind_bcs = JsFuture::from(promise)
      .await
      .map(|v| Uint8Array::from(v).to_vec())
      .map_err(WasmError::from)?;

    let tx_kind = bcs::from_bytes(&tx_kind_bcs).map_err(|e| {
      TsSdkError::TransactionSerializationError(format!("failed to deserialize BCS TransactionKind: {e}"))
    })?;

    #[allow(irrefutable_let_patterns)]
    let TransactionKind::ProgrammableTransaction(pt) = tx_kind
    else {
      return Err(TsSdkError::WasmError(
        "TransactionKind variant".to_owned(),
        "only programmable transactions can be used within this library".to_owned(),
      ));
    };

    bcs::to_bytes(&pt).map_err(|e| {
      TsSdkError::TransactionSerializationError(format!("failed to BCS serialize programmable transaction: {e}"))
    })
  }
}
