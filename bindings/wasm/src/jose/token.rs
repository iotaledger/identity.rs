// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use wasm_bindgen::prelude::*;
use crate::jose::WasmJwsHeader; 
use crate::error::Result;
use crate::error::WasmResult; 

/// A cryptographically verified decoded token from a JWS.
///
/// Contains the decoded headers and the raw claims.
#[wasm_bindgen(js_name = Token, inspectable)]
#[derive(serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WasmToken {
    // As long as we only expose functionality that operates with the compact serialization the `protected_header` will always be present. 
    // We wrap it in an option in case we want to include JWS JSON serialization operations down the line without introducing too many breaking changes.  
    pub(crate) protected_header: Option<WasmJwsHeader>, 
    pub(crate) claims: String
}

#[wasm_bindgen(js_class = Token)]
impl WasmToken {
    /// Returns a copy of the parsed claims. 
    #[wasm_bindgen]
    pub fn claims(&self) -> String {
        self.claims.clone()
    }

    /// Returns a copy of the protected header. 
    #[wasm_bindgen(js_name = protectedHeader)]
    pub fn protected_header(&self) -> Option<WasmJwsHeader> {
        self.protected_header.as_ref().map(ToOwned::to_owned)
    }

    /// Deep clones the object.
    #[wasm_bindgen(js_name = clone)]
    pub fn deep_clone(&self) -> WasmToken {
      return self.clone()
    }

    /// Serializes this to a JSON object.
    #[wasm_bindgen(js_name = toJSON)]
    pub fn to_json(&self) -> Result<JsValue> {
        JsValue::from_serde(&self).wasm_result()
    }
}
