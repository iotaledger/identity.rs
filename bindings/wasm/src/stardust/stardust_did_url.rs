// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_stardust::StardustDIDUrl;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;
use crate::stardust::WasmStardustDID;

/// A DID URL conforming to the IOTA Stardust UTXO DID method specification.
#[wasm_bindgen(js_name = StardustDIDUrl, inspectable)]
pub struct WasmStardustDIDUrl(pub(crate) StardustDIDUrl);

#[wasm_bindgen(js_class = StardustDIDUrl)]
impl WasmStardustDIDUrl {
  /// Parses a `StardustDIDUrl` from the input string.
  #[wasm_bindgen]
  pub fn parse(input: &str) -> Result<WasmStardustDIDUrl> {
    StardustDIDUrl::parse(input).map(WasmStardustDIDUrl).wasm_result()
  }

  /// Return a copy of the `StardustDID` section of the `StardustDIDUrl`.
  #[wasm_bindgen]
  pub fn did(&self) -> WasmStardustDID {
    WasmStardustDID::from(self.0.did().clone())
  }

  /// Return a copy of the relative DID Url as a string, including only the path, query, and fragment.
  #[wasm_bindgen(js_name = urlStr)]
  pub fn url_str(&self) -> String {
    self.0.url().to_string()
  }

  /// Returns a copy of the `StardustDIDUrl` method fragment, if any. Excludes the leading '#'.
  #[wasm_bindgen]
  pub fn fragment(&self) -> Option<String> {
    self.0.fragment().map(str::to_owned)
  }

  /// Sets the `fragment` component of the `StardustDIDUrl`.
  #[wasm_bindgen(js_name = setFragment)]
  pub fn set_fragment(&mut self, value: Option<String>) -> Result<()> {
    self.0.set_fragment(value.as_deref()).wasm_result()
  }

  /// Returns a copy of the `StardustDIDUrl` path.
  #[wasm_bindgen]
  pub fn path(&self) -> Option<String> {
    self.0.path().map(str::to_owned)
  }

  /// Sets the `path` component of the `StardustDIDUrl`.
  #[wasm_bindgen(js_name = setPath)]
  pub fn set_path(&mut self, value: Option<String>) -> Result<()> {
    self.0.set_path(value.as_deref()).wasm_result()
  }

  /// Returns a copy of the `StardustDIDUrl` method query, if any. Excludes the leading '?'.
  #[wasm_bindgen]
  pub fn query(&self) -> Option<String> {
    self.0.query().map(str::to_owned)
  }

  /// Sets the `query` component of the `StardustDIDUrl`.
  #[wasm_bindgen(js_name = setQuery)]
  pub fn set_query(&mut self, value: Option<String>) -> Result<()> {
    self.0.set_query(value.as_deref()).wasm_result()
  }

  /// Append a string representing a path, query, and/or fragment, returning a new `StardustDIDUrl`.
  ///
  /// Must begin with a valid delimiter character: '/', '?', '#'. Overwrites the existing URL
  /// segment and any following segments in order of path, query, then fragment.
  ///
  /// I.e.
  /// - joining a path will clear the query and fragment.
  /// - joining a query will clear the fragment.
  /// - joining a fragment will only overwrite the fragment.
  #[wasm_bindgen]
  pub fn join(&self, segment: &str) -> Result<WasmStardustDIDUrl> {
    self.0.join(segment).map(WasmStardustDIDUrl::from).wasm_result()
  }

  /// Returns the `StardustDIDUrl` as a string.
  #[allow(clippy::inherent_to_string)]
  #[wasm_bindgen(js_name = toString)]
  pub fn to_string(&self) -> String {
    self.0.to_string()
  }
}

impl_wasm_json!(WasmStardustDIDUrl, StardustDIDUrl);
impl_wasm_clone!(WasmStardustDIDUrl, StardustDIDUrl);

impl From<StardustDIDUrl> for WasmStardustDIDUrl {
  fn from(did_url: StardustDIDUrl) -> Self {
    Self(did_url)
  }
}

impl From<WasmStardustDIDUrl> for StardustDIDUrl {
  fn from(wasm_did_url: WasmStardustDIDUrl) -> Self {
    wasm_did_url.0
  }
}
