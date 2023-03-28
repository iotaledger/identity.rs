// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::credential::vc_jwt_validation::CredentialToken;
use wasm_bindgen::prelude::*;

use crate::credential::WasmCredential;
use crate::jose::WasmJwsHeader;

/// A cryptographically verified and decoded Credential.
// TODO: Explain that only the JWS signature has been verified and not necessarily the proof property if it is set.
#[wasm_bindgen(js_name = CredentialToken)]
pub struct WasmCredentialToken(pub(crate) CredentialToken);

#[wasm_bindgen(js_class = CredentialToken)]
impl WasmCredentialToken {
  /// Returns a copy of the credential.
  #[wasm_bindgen]
  pub fn credential(&self) -> WasmCredential {
    WasmCredential(self.0.credential.clone())
  }

  /// Returns a copy of the protected header parsed from the decoded JWS.
  #[wasm_bindgen(js_name = protectedHeader)]
  pub fn protected_header(&self) -> WasmJwsHeader {
    WasmJwsHeader(self.0.header.as_ref().clone())
  }

  /// Consumes the object and returns the decoded credential.
  ///
  /// ### Warning
  /// This destroys the `CredentialToken` object.
  #[wasm_bindgen(js_name = intoCredential)]
  pub fn into_credential(self) -> WasmCredential {
    WasmCredential(self.0.credential)
  }
}
