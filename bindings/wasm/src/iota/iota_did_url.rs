// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::iota_core::IotaDIDUrl;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;
use crate::iota::WasmIotaDID;

/// A DID URL conforming to the IOTA DID method specification.
#[wasm_bindgen(js_name = IotaDIDUrl, inspectable)]
pub struct WasmIotaDIDUrl(pub(crate) IotaDIDUrl);

#[wasm_bindgen(js_class = IotaDIDUrl)]
impl WasmIotaDIDUrl {
  /// Parses a `IotaDIDUrl` from the input string.
  #[wasm_bindgen]
  pub fn parse(input: &str) -> Result<WasmIotaDIDUrl> {
    IotaDIDUrl::parse(input).map(WasmIotaDIDUrl).wasm_result()
  }

  /// Return a copy of the `IotaDID` section of the `IotaDIDUrl`.
  #[wasm_bindgen]
  pub fn did(&self) -> WasmIotaDID {
    WasmIotaDID::from(self.0.did().clone())
  }

  /// Return a copy of the relative DID Url as a string, including only the path, query, and fragment.
  #[wasm_bindgen(js_name = urlStr)]
  pub fn url_str(&self) -> String {
    self.0.url().to_string()
  }

  /// Returns a copy of the `IotaDIDUrl` method fragment, if any. Excludes the leading '#'.
  #[wasm_bindgen]
  pub fn fragment(&self) -> Option<String> {
    self.0.fragment().map(str::to_owned)
  }

  /// Sets the `fragment` component of the `IotaDIDUrl`.
  #[wasm_bindgen(js_name = setFragment)]
  pub fn set_fragment(&mut self, value: Option<String>) -> Result<()> {
    self.0.set_fragment(value.as_deref()).wasm_result()
  }

  /// Returns a copy of the `IotaDIDUrl` path.
  #[wasm_bindgen]
  pub fn path(&self) -> Option<String> {
    self.0.path().map(str::to_owned)
  }

  /// Sets the `path` component of the `IotaDIDUrl`.
  #[wasm_bindgen(js_name = setPath)]
  pub fn set_path(&mut self, value: Option<String>) -> Result<()> {
    self.0.set_path(value.as_deref()).wasm_result()
  }

  /// Returns a copy of the `IotaDIDUrl` method query, if any. Excludes the leading '?'.
  #[wasm_bindgen]
  pub fn query(&self) -> Option<String> {
    self.0.query().map(str::to_owned)
  }

  /// Sets the `query` component of the `IotaDIDUrl`.
  #[wasm_bindgen(js_name = setQuery)]
  pub fn set_query(&mut self, value: Option<String>) -> Result<()> {
    self.0.set_query(value.as_deref()).wasm_result()
  }

  /// Append a string representing a path, query, and/or fragment, returning a new `IotaDIDUrl`.
  ///
  /// Must begin with a valid delimiter character: '/', '?', '#'. Overwrites the existing URL
  /// segment and any following segments in order of path, query, then fragment.
  ///
  /// I.e.
  /// - joining a path will clear the query and fragment.
  /// - joining a query will clear the fragment.
  /// - joining a fragment will only overwrite the fragment.
  #[wasm_bindgen]
  pub fn join(&self, segment: &str) -> Result<WasmIotaDIDUrl> {
    self.0.join(segment).map(WasmIotaDIDUrl::from).wasm_result()
  }

  /// Returns the `IotaDIDUrl` as a string.
  #[allow(clippy::inherent_to_string)]
  #[wasm_bindgen(js_name = toString)]
  pub fn to_string(&self) -> String {
    self.0.to_string()
  }
}

impl_wasm_json!(WasmIotaDIDUrl, IotaDIDUrl);
impl_wasm_clone!(WasmIotaDIDUrl, IotaDIDUrl);

impl From<IotaDIDUrl> for WasmIotaDIDUrl {
  fn from(did_url: IotaDIDUrl) -> Self {
    Self(did_url)
  }
}

impl From<WasmIotaDIDUrl> for IotaDIDUrl {
  fn from(wasm_did_url: WasmIotaDIDUrl) -> Self {
    wasm_did_url.0
  }
}
