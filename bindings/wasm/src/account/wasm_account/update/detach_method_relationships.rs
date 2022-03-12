// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::cell::RefCell;
use std::cell::RefMut;
use std::rc::Rc;

use identity::account::DetachMethodRelationshipBuilder;
use identity::account::IdentityUpdater;
use identity::account::UpdateError::MissingRequiredField;
use identity::core::OneOrMany;
use identity::did::MethodRelationship;
use identity::iota::Client;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;

use crate::account::wasm_account::account::AccountRc;
use crate::account::wasm_account::WasmAccount;
use crate::common::PromiseVoid;
use crate::did::WasmMethodRelationship;
use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen(js_class = Account)]
impl WasmAccount {
  /// Detaches the given relationship from the given method, if the method exists.
  #[wasm_bindgen(js_name = detachMethodRelationships)]
  pub fn detach_method_relationships(&mut self, options: &DetachMethodRelationshipOptions) -> Result<PromiseVoid> {
    let relationships: Vec<MethodRelationship> = options
      .relationships()
      .into_serde::<OneOrMany<WasmMethodRelationship>>()
      .map(OneOrMany::into_vec)
      .wasm_result()?
      .into_iter()
      .map(MethodRelationship::from)
      .collect();

    let fragment: String = options
      .fragment()
      .ok_or(MissingRequiredField("fragment"))
      .wasm_result()?;

    let account: Rc<RefCell<AccountRc>> = Rc::clone(&self.0);

    let promise: Promise = future_to_promise(async move {
      if relationships.is_empty() {
        return Ok(JsValue::undefined());
      }
      let mut account: RefMut<AccountRc> = account.borrow_mut();
      let mut updater: IdentityUpdater<'_, Rc<Client>> = account.update_identity();
      let mut detach_relationship: DetachMethodRelationshipBuilder<'_, Rc<Client>> =
        updater.detach_method_relationship().fragment(fragment);

      for relationship in relationships {
        detach_relationship = detach_relationship.relationship(relationship);
      }

      detach_relationship
        .apply()
        .await
        .wasm_result()
        .map(|_| JsValue::undefined())
    });
    Ok(promise.unchecked_into::<PromiseVoid>())
  }
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "DetachMethodRelationshipOptions")]
  pub type DetachMethodRelationshipOptions;

  #[wasm_bindgen(getter, method)]
  pub fn fragment(this: &DetachMethodRelationshipOptions) -> Option<String>;

  #[wasm_bindgen(getter, method)]
  pub fn relationships(this: &DetachMethodRelationshipOptions) -> JsValue;
}

#[wasm_bindgen(typescript_custom_section)]
const TS_DETACH_METHOD_RELATIONSHIP_OPTIONS: &'static str = r#"
/**
 * Options for detaching one or more verification relationships from a method on an identity.
 */
export type DetachMethodRelationshipOptions = {
    /**
     * The identifier of the method in the document.
     */
    fragment: string,

    /**
     * The relationships to remove.
     */
    relationships: MethodRelationship | MethodRelationship[]
};
"#;
