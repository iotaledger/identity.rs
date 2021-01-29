// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::utils::decode_b58;
use identity_iota::did::IotaDID;
use wasm_bindgen::prelude::*;

use crate::{js_err, key::Key};

/// @typicalname did
#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct DID(pub(crate) IotaDID);

#[wasm_bindgen]
impl DID {
  fn create(pubkey: &[u8], network: Option<&str>) -> Result<DID, JsValue> {
    IotaDID::with_network(pubkey, network).map_err(js_err).map(Self)
  }

  /// Creates a new `DID` from a `Key` object.
  #[wasm_bindgen(constructor)]
  pub fn new(key: &Key, network: Option<String>) -> Result<DID, JsValue> {
    Self::create(key.0.public().as_ref(), network.as_deref())
  }

  /// Creates a new `DID` from a base58-encoded public key.
  #[wasm_bindgen(js_name = fromBase58Key)]
  pub fn from_base58_key(key: &str, network: Option<String>) -> Result<DID, JsValue> {
    Self::create(&decode_b58(key).map_err(js_err)?, network.as_deref())
  }

  /// Parses a `DID` from the input string.
  #[wasm_bindgen]
  pub fn parse(input: String) -> Result<DID, JsValue> {
    IotaDID::parse(input).map_err(js_err).map(Self)
  }

  /// Returns the IOTA tangle network of the `DID`.
  #[wasm_bindgen(getter)]
  pub fn network(&self) -> String {
    self.0.network().into()
  }

  /// Returns the IOTA tangle shard of the `DID` (if any).
  #[wasm_bindgen(getter)]
  pub fn shard(&self) -> Option<String> {
    self.0.shard().map(Into::into)
  }

  /// Returns the unique tag of the `DID`.
  #[wasm_bindgen(getter)]
  pub fn tag(&self) -> String {
    self.0.method_id().into()
  }

  /// Returns the IOTA tangle address of the `DID`.
  #[wasm_bindgen(getter)]
  pub fn address(&self) -> String {
    self.0.address()
  }

  /// Returns the `DID` object as a string.
  #[allow(clippy::inherent_to_string)]
  #[wasm_bindgen(js_name = toString)]
  pub fn to_string(&self) -> String {
    self.0.to_string()
  }
}
