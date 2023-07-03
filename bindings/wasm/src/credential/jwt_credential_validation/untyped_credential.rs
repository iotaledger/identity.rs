// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::core::Object;
use identity_iota::credential::Credential;
use identity_iota::credential::Jwt;
use wasm_bindgen::prelude::*;

use crate::credential::WasmCredential;
use crate::credential::WasmJwt;

#[wasm_bindgen(js_name = UntypedCredential)]
pub struct WasmUntypedCredentialContainer(UntypedCredential);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UntypedCredential {
  Jwt(Jwt),
  Credential(Credential),
  Other(Object),
}

#[wasm_bindgen(js_class = UntypedCredential)]
impl WasmUntypedCredentialContainer {
  pub(crate) fn new(inner: UntypedCredential) -> Self {
    WasmUntypedCredentialContainer(inner)
  }

  /// Returns a `Jwt` if the credential is of type string, `undefined` otherwise.
  #[wasm_bindgen(js_name = tryIntoJwt)]
  pub fn try_into_jwt(&self) -> Option<WasmJwt> {
    match &self.0 {
      UntypedCredential::Jwt(jwt) => Some(WasmJwt::new(jwt.clone())),
      _ => None,
    }
  }

  /// Returns a `Credential` if the credential is of said type, `undefined` otherwise.
  #[wasm_bindgen(js_name = tryIntoCredential)]
  pub fn try_into_credential(&self) -> Option<WasmCredential> {
    match &self.0 {
      UntypedCredential::Credential(credential) => Some(WasmCredential::from(credential.clone())),
      _ => None,
    }
  }
}

impl_wasm_json!(WasmUntypedCredentialContainer, UntypedCredential);
impl_wasm_clone!(WasmUntypedCredentialContainer, UntypedCredential);
