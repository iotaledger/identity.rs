// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::core::Object;
use identity::core::ToJson;
use identity::credential::Presentation;
use identity::credential::PresentationBuilder;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::common::ArrayString;
use crate::common::MapStringAny;
use crate::credential::ArrayContext;
use crate::credential::ArrayPolicy;
use crate::credential::ArrayRefreshService;
use crate::credential::IPresentation;
use crate::credential::WasmCredential;
use crate::crypto::WasmProof;
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
  /// Returns the base JSON-LD context.
  #[wasm_bindgen(js_name = "BaseContext")]
  pub fn base_context() -> Result<String> {
    Presentation::<Object, Object>::base_context().to_json().wasm_result()
  }

  /// Returns the base type.
  #[wasm_bindgen(js_name = "BaseType")]
  pub fn base_type() -> String {
    Presentation::<Object, Object>::base_type().to_owned()
  }

  /// Constructs a new `Presentation`.
  #[wasm_bindgen(constructor)]
  pub fn new(values: IPresentation) -> Result<WasmPresentation> {
    let builder: PresentationBuilder = PresentationBuilder::try_from(values)?;
    builder.build().map(Self).wasm_result()
  }

  /// Returns a copy of the JSON-LD context(s) applicable to the `Presentation`.
  #[wasm_bindgen]
  pub fn context(&self) -> Result<ArrayContext> {
    self
      .0
      .context
      .iter()
      .map(JsValue::from_serde)
      .collect::<std::result::Result<js_sys::Array, _>>()
      .wasm_result()
      .map(|value| value.unchecked_into::<ArrayContext>())
  }

  /// Returns a copy of the unique `URI` of the `Presentation`.
  #[wasm_bindgen]
  pub fn id(&self) -> Option<String> {
    self.0.id.as_ref().map(|url| url.to_string())
  }

  /// Returns a copy of the URIs defining the type of the `Presentation`.
  #[wasm_bindgen(js_name = "type")]
  pub fn types(&self) -> ArrayString {
    self
      .0
      .types
      .iter()
      .map(|s| s.as_str())
      .map(JsValue::from_str)
      .collect::<js_sys::Array>()
      .unchecked_into::<ArrayString>()
  }

  /// Returns a copy of the {@link Credential}(s) expressing the claims of the `Presentation`.
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

  /// Returns a copy of the URI of the entity that generated the `Presentation`.
  #[wasm_bindgen]
  pub fn holder(&self) -> Option<String> {
    self.0.holder.as_ref().map(|url| url.to_string())
  }

  /// Returns a copy of the service(s) used to refresh an expired {@link Credential} in the `Presentation`.
  #[wasm_bindgen(js_name = "refreshService")]
  pub fn refresh_service(&self) -> Result<ArrayRefreshService> {
    self
      .0
      .refresh_service
      .iter()
      .map(JsValue::from_serde)
      .collect::<std::result::Result<js_sys::Array, _>>()
      .wasm_result()
      .map(|value| value.unchecked_into::<ArrayRefreshService>())
  }

  /// Returns a copy of the terms-of-use specified by the `Presentation` holder
  #[wasm_bindgen(js_name = "termsOfUse")]
  pub fn terms_of_use(&self) -> Result<ArrayPolicy> {
    self
      .0
      .terms_of_use
      .iter()
      .map(JsValue::from_serde)
      .collect::<std::result::Result<js_sys::Array, _>>()
      .wasm_result()
      .map(|value| value.unchecked_into::<ArrayPolicy>())
  }

  /// Returns a copy of the proof used to verify the `Presentation`.
  #[wasm_bindgen]
  pub fn proof(&self) -> Option<WasmProof> {
    self.0.proof.clone().map(WasmProof)
  }

  /// Returns a copy of the miscellaneous properties on the `Presentation`.
  #[wasm_bindgen]
  pub fn properties(&self) -> Result<MapStringAny> {
    MapStringAny::try_from(&self.0.properties)
  }

  /// Serializes a `Presentation` as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes a `Presentation` from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json: &JsValue) -> Result<WasmPresentation> {
    json.into_serde().map(Self).wasm_result()
  }
}

impl_wasm_clone!(WasmPresentation, Presentation);

impl From<Presentation> for WasmPresentation {
  fn from(presentation: Presentation) -> WasmPresentation {
    Self(presentation)
  }
}
