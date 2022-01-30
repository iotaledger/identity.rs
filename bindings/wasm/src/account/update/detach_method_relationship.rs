// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::rc::Rc;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use identity::account::{Account, Update};
use identity::account::UpdateError::MissingRequiredField;
use identity::core::OneOrMany;
use identity::core::OneOrMany::Many;
use identity::core::OneOrMany::One;
use identity::did::MethodRelationship;
use wasm_bindgen::__rt::WasmRefCell;

use crate::account::wasm_account::WasmAccount;
use crate::account::wasm_method_relationship::WasmMethodRelationship;
use crate::error::wasm_error;
use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen(js_class = Account)]
impl WasmAccount {
  /// Detaches the given relationship from the given method, if the method exists.
  #[wasm_bindgen(js_name = detachMethodRelationships)]
  pub fn detach_relationships(&mut self, options: &DetachMethodRelationshipOptions) -> Result<Promise> {
    let relationships: Vec<MethodRelationship> = options
      .relationships()
      .into_serde::<OneOrMany<WasmMethodRelationship>>()
      .map(OneOrMany::into_vec)
      .wasm_result()?
      .into_iter()
      .map(Into::into)
      .collect();

    if relationships.is_empty() {
      return Err(wasm_error(MissingRequiredField("relationships is missing or empty")));
    }
    let account: Rc<WasmRefCell<Account>> = Rc::clone(&self.0);
    let fragment: String = options.fragment().ok_or(wasm_error(MissingRequiredField("fragment")))?;

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
const TS_DETACH_METHOD_RELATIONSHIP_OPTIONS: &'static str = r#"
export type DetachMethodRelationshipOptions = {
  fragment: string,
  relationships: MethodRelationship | MethodRelationship[]
};
"#;
