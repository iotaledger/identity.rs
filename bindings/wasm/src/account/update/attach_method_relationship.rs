// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::account::account::WasmAccount;
use crate::did::{WasmDID, WasmMethodScope, WasmMethodType};
use crate::error::{wasm_error, Result, WasmResult};
use identity::account::Error::UpdateError;
use identity::account::UpdateError::MissingRequiredField;
use identity::account::{Account, IdentityUpdater, MethodSecret, Update};
use identity::core::OneOrMany;
use identity::core::OneOrMany::{Many, One};
use identity::did::{MethodRelationship, MethodScope, MethodType};
use identity::iota::TangleRef;
use js_sys::Promise;
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::borrow::Borrow;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Mutex;
use wasm_bindgen::__rt::WasmRefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen(js_class = Account)]
impl WasmAccount {
  #[wasm_bindgen(js_name = attachRelationship)]
  pub fn attach_relationship(&mut self, input: &AttachMethodRelationshipOptions) -> Result<Promise> {
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
  relationships: WasmMethodRelationship | WasmMethodRelationship[]
};
"#;
