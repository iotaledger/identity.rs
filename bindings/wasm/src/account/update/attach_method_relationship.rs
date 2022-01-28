// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use js_sys::Promise;
use serde_repr::Deserialize_repr;
use serde_repr::Serialize_repr;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use identity::account::Update;
use identity::account::UpdateError::MissingRequiredField;
use identity::core::OneOrMany;
use identity::core::OneOrMany::Many;
use identity::core::OneOrMany::One;
use identity::did::MethodRelationship;

use crate::account::wasm_account::WasmAccount;
use crate::error::wasm_error;
use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen(js_class = Account)]
impl WasmAccount {
  /// Attach one or more verification relationships to a method.
  ///
  /// Note: the method must exist and be in the set of verification methods;
  /// it cannot be an embedded method.
  #[wasm_bindgen(js_name = attachMethodRelationships)]
  pub fn attach_relationships(&mut self, input: &AttachMethodRelationshipOptions) -> Result<Promise> {
    let relationships: Vec<WasmMethodRelationship> = input
      .relationships()
      .into_serde::<OneOrMany<WasmMethodRelationship>>()
      .wasm_result()?
      .into();

    let relationships: Vec<MethodRelationship> = relationships.into_iter().map(Into::into).collect();

    let account = self.0.clone();
    let fragment = match input.fragment() {
      Some(value) => value,
      None => return Err(wasm_error(MissingRequiredField("fragment"))),
    };

    let promise: Promise = future_to_promise(async move {
      let update = Update::AttachMethodRelationship {
        fragment,
        relationships,
      };

      account
        .as_ref()
        .borrow_mut()
        .process_update(update)
        .await
        .wasm_result()
        .and_then(|output| JsValue::from_serde(&output).wasm_result())
    });

    Ok(promise)
  }
}

#[wasm_bindgen (js_name = MethodRelationship)]
#[derive(Clone, Copy, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum WasmMethodRelationship {
  Authentication = 0,
  AssertionMethod = 1,
  KeyAgreement = 2,
  CapabilityDelegation = 3,
  CapabilityInvocation = 4,
}

impl From<WasmMethodRelationship> for MethodRelationship {
  fn from(r: WasmMethodRelationship) -> Self {
    match r {
      WasmMethodRelationship::Authentication => MethodRelationship::Authentication,
      WasmMethodRelationship::AssertionMethod => MethodRelationship::AssertionMethod,
      WasmMethodRelationship::KeyAgreement => MethodRelationship::KeyAgreement,
      WasmMethodRelationship::CapabilityDelegation => MethodRelationship::CapabilityDelegation,
      WasmMethodRelationship::CapabilityInvocation => MethodRelationship::CapabilityInvocation,
    }
  }
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "AttachMethodRelationshipOptions")]
  pub type AttachMethodRelationshipOptions;

  #[wasm_bindgen(structural, getter, method)]
  pub fn fragment(this: &AttachMethodRelationshipOptions) -> Option<String>;

  #[wasm_bindgen(structural, getter, method)]
  pub fn relationships(this: &AttachMethodRelationshipOptions) -> JsValue;
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type AttachMethodRelationshipOptions = {
  fragment: string,
  relationships: MethodRelationship | MethodRelationship[]
};
"#;
