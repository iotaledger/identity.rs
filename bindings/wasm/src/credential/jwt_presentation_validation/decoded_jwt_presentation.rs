// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::credential::DecodedJwtPresentation;
use wasm_bindgen::prelude::*;

use crate::common::WasmTimestamp;
use crate::credential::jwt_presentation::WasmJwtPresentation;
use crate::credential::ArrayDecodedJwtCredential;
use crate::credential::WasmDecodedJwtCredential;
use crate::jose::WasmJwsHeader;

/// A cryptographically verified and decoded presentation.
///
/// Note that having an instance of this type only means the JWS it was constructed from was verified.
/// It does not imply anything about a potentially present proof property on the presentation itself.
#[wasm_bindgen(js_name = DecodedJwtPresentation)]
pub struct WasmDecodedJwtPresentation(pub(crate) DecodedJwtPresentation);

#[wasm_bindgen(js_class = DecodedJwtPresentation)]
impl WasmDecodedJwtPresentation {
  #[wasm_bindgen]
  pub fn presentation(&self) -> WasmJwtPresentation {
    WasmJwtPresentation(self.0.presentation.clone())
  }

  /// Returns a copy of the protected header parsed from the decoded JWS.
  #[wasm_bindgen(js_name = protectedHeader)]
  pub fn protected_header(&self) -> WasmJwsHeader {
    WasmJwsHeader(self.0.header.as_ref().clone())
  }

  /// Consumes the object and returns the decoded presentation.
  ///
  /// ### Warning
  /// This destroys the `DecodedJwtPresentation` object.
  #[wasm_bindgen(js_name = intoPresentation)]
  pub fn into_presentation(self) -> WasmJwtPresentation {
    WasmJwtPresentation(self.0.presentation)
  }

  /// The expiration date parsed from the JWT claims.
  #[wasm_bindgen(js_name = expirationDate)]
  pub fn expiration_date(&self) -> Option<WasmTimestamp> {
    self.0.expiration_date.map(WasmTimestamp::from)
  }

  /// The issuance date parsed from the JWT claims.
  #[wasm_bindgen(js_name = "issuanceDate")]
  pub fn issuance_date(&self) -> Option<WasmTimestamp> {
    self.0.issuance_date.map(WasmTimestamp::from)
  }

  /// The `aud` property parsed from JWT claims.
  #[wasm_bindgen]
  pub fn audience(&self) -> Option<String> {
    self.0.aud.clone().map(|aud| aud.to_string())
  }

  /// The credentials included in the presentation (decoded).
  #[wasm_bindgen(js_name = "credentials")]
  pub fn credentials(&self) -> ArrayDecodedJwtCredential {
    self
      .0
      .credentials
      .iter()
      .cloned()
      .map(WasmDecodedJwtCredential::from)
      .map(JsValue::from)
      .collect::<js_sys::Array>()
      .unchecked_into::<ArrayDecodedJwtCredential>()
  }
}

impl From<DecodedJwtPresentation> for WasmDecodedJwtPresentation {
  fn from(decoded_presentation: DecodedJwtPresentation) -> Self {
    Self(decoded_presentation)
  }
}
