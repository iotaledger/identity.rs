// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::did::WasmCoreDID;
use crate::error::Result;
use crate::error::WasmResult;
use identity_iota::did::CoreDIDUrl;
use wasm_bindgen::prelude::*;

/// A method agnostic DID Url.
#[wasm_bindgen(js_name = CoreDIDUrl, inspectable)]
pub struct WasmCoreDIDUrl(pub(crate) CoreDIDUrl);

#[wasm_bindgen(js_class = CoreDIDUrl)]
impl WasmCoreDIDUrl {
  /// Parses a `CoreDIDUrl` from the input string.
  #[wasm_bindgen]
  pub fn parse(input: &str) -> Result<WasmCoreDIDUrl> {
    CoreDIDUrl::parse(input).map(WasmCoreDIDUrl).wasm_result()
  }

  /// Return a copy of the `CoreDID` section of the `CoreDIDUrl`.
  #[wasm_bindgen]
  pub fn did(&self) -> WasmCoreDID {
    WasmCoreDID::from(self.0.did().clone())
  }

  /// Return a copy of the relative DID Url as a string, including only the path, query, and fragment.
  #[wasm_bindgen(js_name = urlStr)]
  pub fn url_str(&self) -> String {
    self.0.url().to_string()
  }

  /// Returns a copy of the `CoreDIDUrl` method fragment, if any. Excludes the leading '#'.
  #[wasm_bindgen]
  pub fn fragment(&self) -> Option<String> {
    self.0.fragment().map(str::to_owned)
  }

  /// Sets the `fragment` component of the `CoreDIDUrl`.
  #[wasm_bindgen(js_name = setFragment)]
  pub fn set_fragment(&mut self, value: Option<String>) -> Result<()> {
    self.0.set_fragment(value.as_deref()).wasm_result()
  }

  /// Returns a copy of the `CoreDIDUrl` path.
  #[wasm_bindgen]
  pub fn path(&self) -> Option<String> {
    self.0.path().map(str::to_owned)
  }

  /// Sets the `path` component of the `CoreDIDUrl`.
  #[wasm_bindgen(js_name = setPath)]
  pub fn set_path(&mut self, value: Option<String>) -> Result<()> {
    self.0.set_path(value.as_deref()).wasm_result()
  }

  /// Returns a copy of the `CoreDIDUrl` method query, if any. Excludes the leading '?'.
  #[wasm_bindgen]
  pub fn query(&self) -> Option<String> {
    self.0.query().map(str::to_owned)
  }

  /// Sets the `query` component of the `CoreDIDUrl`.
  #[wasm_bindgen(js_name = setQuery)]
  pub fn set_query(&mut self, value: Option<String>) -> Result<()> {
    self.0.set_query(value.as_deref()).wasm_result()
  }

  /// Append a string representing a path, query, and/or fragment, returning a new `CoreDIDUrl`.
  ///
  /// Must begin with a valid delimiter character: '/', '?', '#'. Overwrites the existing URL
  /// segment and any following segments in order of path, query, then fragment.
  ///
  /// I.e.
  /// - joining a path will clear the query and fragment.
  /// - joining a query will clear the fragment.
  /// - joining a fragment will only overwrite the fragment.
  #[wasm_bindgen]
  pub fn join(&self, segment: &str) -> Result<WasmCoreDIDUrl> {
    self.0.join(segment).map(WasmCoreDIDUrl::from).wasm_result()
  }

  /// Returns the `CoreDIDUrl` as a string.
  #[allow(clippy::inherent_to_string)]
  #[wasm_bindgen(js_name = toString)]
  pub fn to_string(&self) -> String {
    self.0.to_string()
  }
}

impl_wasm_json!(WasmCoreDIDUrl, CoreDIDUrl);
impl_wasm_clone!(WasmCoreDIDUrl, CoreDIDUrl);

impl From<CoreDIDUrl> for WasmCoreDIDUrl {
  fn from(did_url: CoreDIDUrl) -> Self {
    Self(did_url)
  }
}

impl From<WasmCoreDIDUrl> for CoreDIDUrl {
  fn from(wasm_did_url: WasmCoreDIDUrl) -> Self {
    wasm_did_url.0
  }
}
