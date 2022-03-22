// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::core::encode_b58;
use identity::crypto::merkle_key::Blake2b256;
use identity::crypto::merkle_key::Sha256;
use identity::crypto::merkle_tree::Proof;
use identity::crypto::KeyCollection;
use identity::crypto::PrivateKey;
use identity::crypto::PublicKey;
use wasm_bindgen::prelude::*;

use crate::crypto::Digest;
use crate::crypto::WasmKeyPair;
use crate::crypto::WasmKeyType;
use crate::error::Result;
use crate::error::WasmResult;

#[derive(Deserialize, Serialize)]
pub struct WasmKeyCollectionData {
  #[serde(rename = "type")]
  type_: WasmKeyType,
  keys: Vec<WasmKeyData>,
}

#[derive(Deserialize, Serialize)]
struct WasmKeyData {
  public: Vec<u8>,
  private: Vec<u8>,
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
  pub fn new(type_: WasmKeyType, count: usize) -> Result<WasmKeyCollection> {
    KeyCollection::new(type_.into(), count).map(Self).wasm_result()
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

  /// Returns the public key at the specified `index` as a `UInt8Array`.
  #[wasm_bindgen]
  pub fn public(&self, index: usize) -> Option<Vec<u8>> {
    self.0.public(index).map(|public_key| public_key.as_ref().to_vec())
  }

  /// Returns the private key at the specified `index` as a `UInt8Array`.
  #[wasm_bindgen]
  pub fn private(&self, index: usize) -> Option<Vec<u8>> {
    self.0.private(index).map(|public_key| public_key.as_ref().to_vec())
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
  pub fn to_json(&self) -> Result<JsValue> {
    let data: WasmKeyCollectionData = WasmKeyCollectionData::from(self);
    JsValue::from_serde(&data).wasm_result()
  }

  /// Deserializes a `KeyCollection` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json: &JsValue) -> Result<WasmKeyCollection> {
    let data: WasmKeyCollectionData = json.into_serde().wasm_result()?;
    WasmKeyCollection::try_from(data)
  }
}

impl From<&WasmKeyCollection> for WasmKeyCollectionData {
  fn from(collection: &WasmKeyCollection) -> Self {
    let public = collection.0.iter_public();
    let private = collection.0.iter_private();

    let keys: Vec<WasmKeyData> = public
      .zip(private)
      .map(|(public, private)| WasmKeyData {
        public: public.as_ref().to_vec(),
        private: private.as_ref().to_vec(),
      })
      .collect();

    WasmKeyCollectionData {
      keys,
      type_: collection.0.type_().into(),
    }
  }
}

impl TryFrom<WasmKeyCollectionData> for WasmKeyCollection {
  type Error = JsValue;

  fn try_from(data: WasmKeyCollectionData) -> std::result::Result<Self, Self::Error> {
    let iter: _ = data.keys.into_iter().flat_map(|data| {
      let public_key: PublicKey = data.public.into();
      let private_key: PrivateKey = data.private.into();

      Some((public_key, private_key))
    });

    KeyCollection::from_iterator(data.type_.into(), iter)
      .map(Self)
      .wasm_result()
  }
}

impl_wasm_clone!(WasmKeyCollection, KeyCollection);
