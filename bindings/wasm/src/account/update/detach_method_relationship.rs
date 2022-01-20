// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::account::account::WasmAccount;
use crate::account::update::attach_method_relationship::WasmMethodRelationship;
use crate::error::{wasm_error, Result, WasmResult};
use identity::account::Update;
use identity::account::UpdateError::MissingRequiredField;
use identity::core::OneOrMany::{Many, One};
use identity::did::MethodRelationship;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen(js_class = Account)]
impl WasmAccount {
  #[wasm_bindgen(js_name = detachMethodRelationships)]
  pub fn detach_relationships(&mut self, input: &DetachMethodRelationshipOptions) -> Result<Promise> {
    let relationships: Vec<WasmMethodRelationship> = match input.relationships().into_serde().wasm_result()? {
      One(r) => vec![r],
      Many(r) => r,
    };

    let relationships: Vec<MethodRelationship> = relationships.into_iter().map(Into::into).collect();

    let account = self.0.clone();
    let fragment = match input.fragment() {
      Some(value) => value.clone(),
      None => return Err(wasm_error(MissingRequiredField("fragment"))),
    };
    let promise: Promise = future_to_promise(async move {
      let update = Update::DetachMethodRelationship {
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

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "DetachMethodRelationshipOptions")]
  pub type DetachMethodRelationshipOptions;

  #[wasm_bindgen(structural, getter, method)]
  pub fn fragment(this: &DetachMethodRelationshipOptions) -> Option<String>;

  #[wasm_bindgen(structural, getter, method)]
  pub fn relationships(this: &DetachMethodRelationshipOptions) -> JsValue;
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type DetachMethodRelationshipOptions = {
  fragment: string,
  relationships: MethodRelationship | MethodRelationship[]
};
"#;
