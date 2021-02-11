// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::core::OneOrMany;
use identity::core::Url;
use identity::credential::Presentation;
use identity::credential::PresentationBuilder;
use identity::credential::VerifiableCredential;
use identity::credential::VerifiablePresentation as VerifiablePresentation_;
use wasm_bindgen::prelude::*;

use crate::crypto::KeyPair;
use crate::document::Document;
use crate::utils::err;

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct VerifiablePresentation(pub(crate) VerifiablePresentation_);

#[wasm_bindgen]
impl VerifiablePresentation {
  /// Serializes a `VerifiablePresentation` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&self.0).map_err(err)
  }

  /// Deserializes a `VerifiablePresentation` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json: &JsValue) -> Result<VerifiablePresentation, JsValue> {
    json.into_serde().map_err(err).map(Self)
  }
}
