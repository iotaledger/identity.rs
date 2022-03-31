// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::core::OneOrMany;
use identity::core::Url;
use identity::credential::Credential;
use identity::credential::Presentation;
use identity::credential::PresentationBuilder;
use identity::did::DID;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::credential::WasmCredential;
use crate::did::WasmDocument;
use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen(js_name = Presentation, inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct WasmPresentation(pub(crate) Presentation);

// Workaround for Typescript type annotations for returned arrays.
#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "Array<Credential>")]
  pub type ArrayCredential;
}

#[wasm_bindgen(js_class = Presentation)]
impl WasmPresentation {
  #[wasm_bindgen(constructor)]
  pub fn new(
    holder_doc: &WasmDocument,
    credential_data: JsValue,
    presentation_type: Option<String>,
    presentation_id: Option<String>,
  ) -> Result<WasmPresentation> {
    let credentials: OneOrMany<Credential> = credential_data.into_serde().wasm_result()?;
    let holder_url: Url = Url::parse(holder_doc.0.id().as_str()).wasm_result()?;

    let mut builder: PresentationBuilder = PresentationBuilder::default().holder(holder_url);

    for credential in credentials.into_vec() {
      builder = builder.credential(credential);
    }

    if let Some(presentation_type) = presentation_type {
      builder = builder.type_(presentation_type);
    }

    if let Some(presentation_id) = presentation_id {
      builder = builder.id(Url::parse(presentation_id).wasm_result()?);
    }

    builder.build().map(Self).wasm_result()
  }

  /// Serializes a `Presentation` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes a `Presentation` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json: &JsValue) -> Result<WasmPresentation> {
    json.into_serde().map(Self).wasm_result()
  }

  /// Returns a copy of the credentials contained in the presentation.
  #[wasm_bindgen(js_name = verifiableCredential)]
  pub fn verifiable_credential(&self) -> ArrayCredential {
    self
      .0
      .verifiable_credential
      .iter()
      .cloned()
      .map(WasmCredential::from)
      .map(JsValue::from)
      .collect::<js_sys::Array>()
      .unchecked_into::<ArrayCredential>()
  }
}

impl_wasm_clone!(WasmPresentation, Presentation);
