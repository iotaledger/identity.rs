// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::did::CoreDID;
use identity_iota::did::DID;
use wasm_bindgen::prelude::*;

use crate::did::wasm_did_url::WasmDIDUrl;
use crate::error::Result;
use crate::error::WasmResult;

/// A method-agnostic Decentralized Identifier (DID).
#[wasm_bindgen(js_name = CoreDID, inspectable)]
pub struct WasmCoreDID(pub(crate) CoreDID);

#[wasm_bindgen(js_class = CoreDID)]
impl WasmCoreDID {
  /// Parses a `CoreDID` from the given `input`.
  ///
  /// ### Errors
  ///
  /// Throws an error if the input is not a valid `CoreDID`.
  #[wasm_bindgen]
  pub fn parse(input: &str) -> Result<WasmCoreDID> {
    CoreDID::parse(input).wasm_result().map(Self)
  }

  /// Set the method name of the `CoreDID`.
  #[wasm_bindgen(js_name = "setMethodName")]
  pub fn set_method_name(&mut self, value: String) -> Result<()> {
    self.0.set_method_name(&value).wasm_result()
  }

  /// Validates whether a string is a valid DID method name.
  #[wasm_bindgen(js_name = "validMethodName")]
  pub fn valid_method_name(value: String) -> bool {
    CoreDID::valid_method_name(&value).is_ok()
  }

  /// Set the method-specific-id of the `DID`.
  #[wasm_bindgen(js_name = "setMethodId")]
  pub fn set_method_id(&mut self, value: String) -> Result<()> {
    self.0.set_method_id(&value).wasm_result()
  }

  /// Validates whether a string is a valid `DID` method-id.
  #[wasm_bindgen(js_name = "validMethodId")]
  pub fn valid_method_id(value: String) -> bool {
    CoreDID::valid_method_id(&value).is_ok()
  }

  // ===========================================================================
  // DID trait
  // ===========================================================================

  /// Returns the `CoreDID` scheme.
  ///
  /// E.g.
  /// - `"did:example:12345678" -> "did"`
  /// - `"did:iota:smr:12345678" -> "did"`
  #[wasm_bindgen]
  pub fn scheme(&self) -> String {
    self.0.scheme().to_owned()
  }

  /// Returns the `CoreDID` authority: the method name and method-id.
  ///
  /// E.g.
  /// - `"did:example:12345678" -> "example:12345678"`
  /// - `"did:iota:smr:12345678" -> "iota:smr:12345678"`
  #[wasm_bindgen]
  pub fn authority(&self) -> String {
    self.0.authority().to_owned()
  }

  /// Returns the `CoreDID` method name.
  ///
  /// E.g.
  /// - `"did:example:12345678" -> "example"`
  /// - `"did:iota:smr:12345678" -> "iota"`
  #[wasm_bindgen]
  pub fn method(&self) -> String {
    self.0.method().to_owned()
  }

  /// Returns the `CoreDID` method-specific ID.
  ///
  /// E.g.
  /// - `"did:example:12345678" -> "12345678"`
  /// - `"did:iota:smr:12345678" -> "smr:12345678"`
  #[wasm_bindgen(js_name = methodId)]
  pub fn method_id(&self) -> String {
    self.0.method_id().to_owned()
  }

  /// Construct a new `DIDUrl` by joining with a relative DID Url string.
  #[wasm_bindgen]
  pub fn join(&self, segment: &str) -> Result<WasmDIDUrl> {
    self.0.clone().join(segment).wasm_result().map(WasmDIDUrl)
  }

  /// Clones the `CoreDID` into a `DIDUrl`.
  #[wasm_bindgen(js_name = toUrl)]
  pub fn to_url(&self) -> WasmDIDUrl {
    WasmDIDUrl::from(self.0.to_url())
  }

  /// Converts the `CoreDID` into a `DIDUrl`, consuming it.
  #[wasm_bindgen(js_name = intoUrl)]
  pub fn into_url(self) -> WasmDIDUrl {
    WasmDIDUrl::from(self.0.into_url())
  }

  /// Returns the `CoreDID` as a string.
  #[allow(clippy::inherent_to_string)]
  #[wasm_bindgen(js_name = toString)]
  pub fn to_string(&self) -> String {
    self.0.to_string()
  }

  #[wasm_bindgen(js_name = toCoreDid, skip_typescript)]
  // Only intended to be called internally.
  pub fn to_core_did(&self) -> WasmCoreDID {
    WasmCoreDID(self.0.clone())
  }
}

impl_wasm_json!(WasmCoreDID, CoreDID);
impl_wasm_clone!(WasmCoreDID, CoreDID);

impl From<CoreDID> for WasmCoreDID {
  fn from(did: CoreDID) -> Self {
    WasmCoreDID(did)
  }
}

impl From<&IToCoreDID> for CoreDID {
  fn from(value: &IToCoreDID) -> Self {
    get_core_did_clone(value).0
  }
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "CoreDID | IToCoreDID")]
  pub type IToCoreDID;

  // Specially crafted JS function called internally that ensures
  // Custom DID implementations built on `CoreDID` don't get nulled
  // out by Rust. Also avoids double clones when passing an instance of `CoreDID`
  // or `IotaDID`.
  #[wasm_bindgen(js_name = _getCoreDidCloneInternal, skip_typescript)]
  pub fn get_core_did_clone(input: &IToCoreDID) -> WasmCoreDID;

}

#[wasm_bindgen(typescript_custom_section)]
pub const TS_AS_REF_CORE_DID: &'static str = r#"
interface IToCoreDID {

  /** Returns a `CoreDID` representation of this DID. */
  toCoreDid(): CoreDID;
}"#;
