// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::core::decode_b58;
use identity::iota::DID as IotaDID;
use identity::iota::try_did;
use wasm_bindgen::prelude::*;

use crate::crypto::KeyPair;
use crate::utils::err;

// =============================================================================
// =============================================================================

/// @typicalname did
#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct DID(pub(crate) IotaDID);

#[wasm_bindgen]
impl DID {
  pub(crate) fn create(public: &[u8], network: Option<&str>, shard: Option<&str>) -> Result<DID, JsValue> {
    let did: Result<IotaDID, _> = match (network, shard) {
      (Some(network), Some(shard)) => try_did!(public, network, shard),
      (Some(network), None) => try_did!(public, network),
      (None, Some(shard)) => try_did!(public, IotaDID::DEFAULT_NETWORK, shard),
      (None, None) => try_did!(public),
    };

    did.map_err(err).map(Self)
  }

  /// Creates a new `DID` from a `KeyPair` object.
  #[wasm_bindgen(constructor)]
  pub fn new(key: &KeyPair, network: Option<String>, shard: Option<String>) -> Result<DID, JsValue> {
    Self::create(key.0.public().as_ref(), network.as_deref(), shard.as_deref())
  }

  /// Creates a new `DID` from a base58-encoded public key.
  #[wasm_bindgen(js_name = fromBase58)]
  pub fn from_base58(key: &str, network: Option<String>, shard: Option<String>) -> Result<DID, JsValue> {
    decode_b58(key)
      .map_err(err)
      .and_then(|key| Self::create(&key, network.as_deref(), shard.as_deref()))
  }

  /// Parses a `DID` from the input string.
  #[wasm_bindgen]
  pub fn parse(input: &str) -> Result<DID, JsValue> {
    IotaDID::parse(input).map_err(err).map(Self)
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
    self.0.tag().into()
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
