// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::iota_core::IotaDIDUrl;
use wasm_bindgen::prelude::*;

use crate::did::WasmDID;
use crate::error::Result;
use crate::error::WasmResult;

/// @typicalname didUrl
#[wasm_bindgen(js_name = DIDUrl, inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct WasmDIDUrl(pub(crate) IotaDIDUrl);

#[wasm_bindgen(js_class = DIDUrl)]
impl WasmDIDUrl {
  /// Parses a `DIDUrl` from the input string.
  #[wasm_bindgen]
  pub fn parse(input: &str) -> Result<WasmDIDUrl> {
    IotaDIDUrl::parse(input).map(WasmDIDUrl::from).wasm_result()
  }

  /// Return the `DID` section of the `DIDUrl`.
  ///
  /// Note: clones the data
  #[wasm_bindgen(getter)]
  pub fn did(&self) -> WasmDID {
    WasmDID::from(self.0.did().clone())
  }

  /// Return the relative DID Url as a string, including only the path, query, and fragment.
  #[wasm_bindgen(getter)]
  pub fn url_str(&self) -> String {
    self.0.url().to_string()
  }

  /// Returns the `DIDUrl` method fragment, if any. Excludes the leading '#'.
  #[wasm_bindgen(getter)]
  pub fn fragment(&self) -> Option<String> {
    self.0.fragment().map(str::to_owned)
  }

  /// Sets the `fragment` component of the `DIDUrl`.
  #[wasm_bindgen(setter = fragment)]
  pub fn set_fragment(&mut self, value: Option<String>) -> Result<()> {
    self.0.set_fragment(value.as_deref()).wasm_result()
  }

  /// Returns the `DIDUrl` path.
  #[wasm_bindgen(getter)]
  pub fn path(&self) -> Option<String> {
    self.0.path().map(str::to_owned)
  }

  /// Sets the `path` component of the `DIDUrl`.
  #[wasm_bindgen(setter = path)]
  pub fn set_path(&mut self, value: Option<String>) -> Result<()> {
    self.0.set_path(value.as_deref()).wasm_result()
  }

  /// Returns the `DIDUrl` method query, if any. Excludes the leading '?'.
  #[wasm_bindgen(getter)]
  pub fn query(&self) -> Option<String> {
    self.0.query().map(str::to_owned)
  }

  /// Sets the `query` component of the `DIDUrl`.
  #[wasm_bindgen(setter = query)]
  pub fn set_query(&mut self, value: Option<String>) -> Result<()> {
    self.0.set_query(value.as_deref()).wasm_result()
  }

  /// Append a string representing a path, query, and/or fragment to this `DIDUrl`.
  ///
  /// Must begin with a valid delimiter character: '/', '?', '#'. Overwrites the existing URL
  /// segment and any following segments in order of path, query, then fragment.
  ///
  /// I.e.
  /// - joining a path will clear the query and fragment.
  /// - joining a query will clear the fragment.
  /// - joining a fragment will only overwrite the fragment.
  #[wasm_bindgen]
  pub fn join(self, segment: &str) -> Result<WasmDIDUrl> {
    self.0.join(segment).map(WasmDIDUrl::from).wasm_result()
  }

  /// Returns the `DIDUrl` as a string.
  #[allow(clippy::inherent_to_string)]
  #[wasm_bindgen(js_name = toString)]
  pub fn to_string(&self) -> String {
    self.0.to_string()
  }

  /// Serializes a `DIDUrl` as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }
}

impl_wasm_clone!(WasmDIDUrl, DIDUrl);

impl From<IotaDIDUrl> for WasmDIDUrl {
  fn from(did_url: IotaDIDUrl) -> Self {
    Self(did_url)
  }
}
