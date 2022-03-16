// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::core::decode_b58;
use identity::core::encode_b58;
use identity::crypto::merkle_key::Blake2b256;
use identity::crypto::merkle_key::Sha256;
use identity::crypto::merkle_tree::Proof;
use identity::crypto::KeyCollection;
use identity::crypto::PrivateKey;
use identity::crypto::PublicKey;
use wasm_bindgen::prelude::*;

use crate::crypto::Digest;
use crate::crypto::KeyType;
use crate::crypto::WasmKeyPair;
use crate::error::wasm_error;

#[derive(Deserialize, Serialize)]
struct JsonData {
  #[serde(rename = "type")]
  type_: KeyType,
  keys: Vec<KeyData>,
}

#[derive(Deserialize, Serialize)]
struct KeyData {
  public: String,
  private: String,
}

// =============================================================================
// =============================================================================

#[wasm_bindgen(inspectable, js_name = KeyCollection)]
#[derive(Clone, Debug)]
pub struct WasmKeyCollection(pub(crate) KeyCollection);

#[wasm_bindgen(js_class = KeyCollection)]
impl WasmKeyCollection {
  /// Creates a new `KeyCollection` with the specified key type.
  #[wasm_bindgen(constructor)]
  pub fn new(type_: KeyType, count: usize) -> Result<WasmKeyCollection, JsValue> {
    KeyCollection::new(type_.into(), count).map_err(wasm_error).map(Self)
  }

  /// Returns the number of keys in the collection.
  #[wasm_bindgen(getter)]
  pub fn length(&self) -> usize {
    self.0.len()
  }

  /// Returns `true` if the collection contains no keys.
  #[wasm_bindgen(js_name = isEmpty)]
  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }

  /// Returns the keypair at the specified `index`.
  #[wasm_bindgen]
  pub fn keypair(&self, index: usize) -> Option<WasmKeyPair> {
    self.0.keypair(index).map(WasmKeyPair)
  }

  /// Returns the public key at the specified `index` as a base58-encoded string.
  #[wasm_bindgen]
  pub fn public(&self, index: usize) -> Option<String> {
    self.0.public(index).map(encode_b58)
  }

  /// Returns the private key at the specified `index` as a base58-encoded string.
  #[wasm_bindgen]
  pub fn private(&self, index: usize) -> Option<String> {
    self.0.private(index).map(encode_b58)
  }

  #[wasm_bindgen(js_name = merkleRoot)]
  pub fn merkle_root(&self, digest: Digest) -> String {
    match digest {
      Digest::Sha256 => encode_b58(self.0.merkle_root::<Sha256>().as_slice()),
      Digest::Blake2b256 => encode_b58(self.0.merkle_root::<Blake2b256>().as_slice()),
    }
  }

  #[wasm_bindgen(js_name = merkleProof)]
  pub fn merkle_proof(&self, digest: Digest, index: usize) -> Option<String> {
    match digest {
      Digest::Sha256 => {
        let proof: Proof<Sha256> = match self.0.merkle_proof(index) {
          Some(proof) => proof,
          None => return None,
        };

        Some(encode_b58(&proof.encode()))
      }
      Digest::Blake2b256 => {
        let proof: Proof<Blake2b256> = match self.0.merkle_proof(index) {
          Some(proof) => proof,
          None => return None,
        };

        Some(encode_b58(&proof.encode()))
      }
    }
  }

  /// Serializes a `KeyCollection` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    let public: _ = self.0.iter_public();
    let private: _ = self.0.iter_private();

    let keys: Vec<KeyData> = public
      .zip(private)
      .map(|(public, private)| KeyData {
        public: encode_b58(public),
        private: encode_b58(private),
      })
      .collect();

    let data: JsonData = JsonData {
      keys,
      type_: self.0.type_().into(),
    };

    JsValue::from_serde(&data).map_err(wasm_error)
  }

  /// Deserializes a `KeyCollection` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json: &JsValue) -> Result<WasmKeyCollection, JsValue> {
    let data: JsonData = json.into_serde().map_err(wasm_error)?;

    let iter: _ = data.keys.iter().flat_map(|data| {
      let public_key: PublicKey = decode_b58(&data.public).ok()?.into();
      let private_key: PrivateKey = decode_b58(&data.private).ok()?.into();

      Some((public_key, private_key))
    });

    KeyCollection::from_iterator(data.type_.into(), iter)
      .map_err(wasm_error)
      .map(Self)
  }
}

impl_wasm_clone!(WasmKeyCollection, KeyCollection);
