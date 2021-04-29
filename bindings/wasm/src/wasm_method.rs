// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::crypto::merkle_key::Sha256;
use identity::iota::IotaMethod as Method_;
use wasm_bindgen::prelude::*;

use crate::crypto::Digest;
use crate::crypto::KeyCollection;
use crate::crypto::KeyPair;
use crate::utils::err;
use crate::wasm_did::WasmDID;

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct WasmMethod(pub(crate) Method_);

#[wasm_bindgen]
impl WasmMethod {
  /// Creates a new `WasmMethod` object from the given `key`.
  #[wasm_bindgen(constructor)]
  pub fn new(key: &KeyPair, tag: Option<String>) -> Result<WasmMethod, JsValue> {
    Method_::from_keypair(&key.0, tag.as_deref()).map_err(err).map(Self)
  }

  /// Creates a new `WasmMethod` object from the given `did` and `key`.
  #[wasm_bindgen(js_name = fromDID)]
  pub fn from_did(did: &WasmDID, key: &KeyPair, tag: Option<String>) -> Result<WasmMethod, JsValue> {
    Method_::from_did(did.0.clone(), &key.0, tag.as_deref())
      .map_err(err)
      .map(Self)
  }

  /// Creates a new Merkle Key Collection Method from the given key collection.
  #[wasm_bindgen(js_name = createMerkleKey)]
  pub fn create_merkle_key(
    digest: Digest,
    did: &WasmDID,
    keys: &KeyCollection,
    tag: Option<String>,
  ) -> Result<WasmMethod, JsValue> {
    match digest {
      Digest::Sha256 => Method_::create_merkle_key::<Sha256, _>(did.0.clone(), &keys.0, tag.as_deref())
        .map_err(err)
        .map(Self),
    }
  }

  /// Returns the `id` DID of the `WasmMethod` object.
  #[wasm_bindgen(getter)]
  pub fn id(&self) -> WasmDID {
    WasmDID(self.0.id().clone())
  }

  /// Returns the `controller` DID of the `WasmMethod` object.
  #[wasm_bindgen(getter)]
  pub fn controller(&self) -> WasmDID {
    WasmDID(self.0.controller().clone())
  }

  /// Returns the `WasmMethod` type.
  #[wasm_bindgen(getter, js_name = type)]
  pub fn type_(&self) -> String {
    self.0.key_type().as_str().into()
  }

  /// Returns the `WasmMethod` public key data.
  #[wasm_bindgen(getter)]
  pub fn data(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(self.0.key_data()).map_err(err)
  }

  /// Serializes a `WasmMethod` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&self.0).map_err(err)
  }

  /// Deserializes a `WasmMethod` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<WasmMethod, JsValue> {
    value.into_serde().map_err(err).map(Self)
  }
}
