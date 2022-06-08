// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::core::Url;
use identity::did::RevocationBitmap;
use identity::did::ServiceEndpoint;
use wasm_bindgen::prelude::*;

use crate::did::service_endpoint_to_js_value;
use crate::did::UServiceEndpoint;
use crate::error::Result;
use crate::error::WasmResult;

/// A compressed bitmap for managing credential revocation.
#[wasm_bindgen(js_name = RevocationBitmap, inspectable)]
pub struct WasmRevocationBitmap(pub(crate) RevocationBitmap);

#[allow(clippy::new_without_default)]
#[wasm_bindgen(js_class = RevocationBitmap)]
impl WasmRevocationBitmap {
  /// Creates a new `RevocationBitmap` instance.
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    WasmRevocationBitmap(RevocationBitmap::new())
  }

  /// The name of the service type.
  #[wasm_bindgen(js_name = type)]
  pub fn type_() -> String {
    RevocationBitmap::TYPE.to_owned()
  }

  /// Returns `true` if the credential at the given `index` is revoked.
  #[wasm_bindgen(js_name = isRevoked)]
  pub fn is_revoked(&self, index: u32) -> bool {
    self.0.is_revoked(index)
  }

  /// Mark the given index as revoked.
  ///
  /// Returns true if the index was absent from the set.
  #[wasm_bindgen]
  pub fn revoke(&mut self, index: u32) -> bool {
    self.0.revoke(index)
  }

  /// Mark the index as not revoked.
  ///
  /// Returns true if the index was present in the set.
  #[wasm_bindgen]
  pub fn unrevoke(&mut self, index: u32) -> bool {
    self.0.unrevoke(index)
  }

  /// Return the bitmap as a data url embedded in a service endpoint.
  #[wasm_bindgen(js_name = toEndpoint)]
  pub fn to_enpdoint(&self) -> Result<UServiceEndpoint> {
    Ok(service_endpoint_to_js_value(&self.0.to_endpoint().wasm_result()?))
  }

  /// Construct a `RevocationBitmap` from a data `url`.
  #[wasm_bindgen(js_name = fromEndpoint)]
  pub fn from_endpoint(url: &str) -> Result<WasmRevocationBitmap> {
    RevocationBitmap::from_endpoint(&ServiceEndpoint::One(Url::parse(url).wasm_result()?))
      .map(Self)
      .wasm_result()
  }
}

impl From<RevocationBitmap> for WasmRevocationBitmap {
  fn from(revocation_list: RevocationBitmap) -> Self {
    WasmRevocationBitmap(revocation_list)
  }
}

impl From<WasmRevocationBitmap> for RevocationBitmap {
  fn from(revocation_list: WasmRevocationBitmap) -> Self {
    revocation_list.0
  }
}
