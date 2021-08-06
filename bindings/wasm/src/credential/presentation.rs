// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::core::OneOrMany;
use identity::core::Url;
use identity::credential::Credential;
use identity::credential::Presentation;
use identity::credential::PresentationBuilder;
use wasm_bindgen::prelude::*;

use crate::error::wasm_error;
use crate::wasm_document::WasmDocument;

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct VerifiablePresentation(pub(crate) Presentation);

#[wasm_bindgen]
impl VerifiablePresentation {
  #[wasm_bindgen(constructor)]
  pub fn new(
    holder_doc: &WasmDocument,
    credential_data: JsValue,
    presentation_type: Option<String>,
    presentation_id: Option<String>,
  ) -> Result<VerifiablePresentation, JsValue> {
    let credentials: OneOrMany<Credential> = credential_data.into_serde().map_err(wasm_error)?;
    let holder_url: Url = Url::parse(holder_doc.0.id().as_str()).map_err(wasm_error)?;

    let mut builder: PresentationBuilder = PresentationBuilder::default().holder(holder_url);

    for credential in credentials.into_vec() {
      builder = builder.credential(credential);
    }

    if let Some(presentation_type) = presentation_type {
      builder = builder.type_(presentation_type);
    }

    if let Some(presentation_id) = presentation_id {
      builder = builder.id(Url::parse(presentation_id).map_err(wasm_error)?);
    }

    builder.build().map_err(wasm_error).map(Self)
  }

  /// Serializes a `VerifiablePresentation` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&self.0).map_err(wasm_error)
  }

  /// Deserializes a `VerifiablePresentation` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json: &JsValue) -> Result<VerifiablePresentation, JsValue> {
    json.into_serde().map_err(wasm_error).map(Self)
  }
}
