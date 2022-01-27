// Copyright 2020-2022 IOTA Stiftun
// SPDX-License-Identifier: Apache-2.0

use serde_repr::Deserialize_repr;
use serde_repr::Serialize_repr;
use wasm_bindgen::prelude::*;

use identity::did::MethodRelationship;

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
