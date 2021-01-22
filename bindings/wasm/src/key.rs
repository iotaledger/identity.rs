// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::{
    crypto::{KeyPair, PublicKey, SecretKey},
    did_doc::MethodType,
    proof::JcsEd25519Signature2020,
    utils::{decode_b58, encode_b58},
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::js_err;

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug)]
pub struct Key(pub(crate) KeyPair);

#[wasm_bindgen]
impl Key {
    /// Generates a new `Key` object.
    #[wasm_bindgen(constructor)]
    pub fn new(key_type: &str) -> Result<Key, JsValue> {
        match key_type.parse().map_err(js_err)? {
            MethodType::Ed25519VerificationKey2018 => Ok(Self::generate_ed25519()),
            _ => Err("Invalid Key Type".into()),
        }
    }

    /// Generates a new `Key` object suitable for ed25519 signatures.
    #[wasm_bindgen(js_name = generateEd25519)]
    pub fn generate_ed25519() -> Key {
        Self(JcsEd25519Signature2020::new_keypair())
    }

    /// Parses a `Key` object from base58-encoded public/private keys.
    #[wasm_bindgen(js_name = fromBase58)]
    pub fn from_base58(public_key: &str, private_key: &str) -> Result<Key, JsValue> {
        let public: PublicKey = decode_b58(public_key).map_err(js_err)?.into();
        let private: SecretKey = decode_b58(private_key).map_err(js_err)?.into();

        Ok(Self(KeyPair::new(public, private)))
    }

    /// Returns the public key as a base58-encoded string.
    #[wasm_bindgen(getter)]
    pub fn public(&self) -> String {
        encode_b58(self.0.public())
    }

    /// Returns the private key as a base58-encoded string.
    #[wasm_bindgen(getter)]
    pub fn private(&self) -> String {
        encode_b58(self.0.secret())
    }

    /// Serializes a `Key` object as a JSON object.
    #[wasm_bindgen(js_name = toJSON)]
    pub fn to_json(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.to_key_data()).map_err(js_err)
    }

    /// Deserializes a `Key` object from a JSON object.
    #[wasm_bindgen(js_name = fromJSON)]
    pub fn from_json(json: &JsValue) -> Result<Key, JsValue> {
        let data: KeyData = json.into_serde().map_err(js_err)?;
        let this: Self = Self::from_base58(&data.public, &data.private)?;

        Ok(this)
    }

    fn to_key_data(&self) -> KeyData {
        KeyData {
            public: self.public(),
            private: self.private(),
        }
    }
}

#[derive(Deserialize, Serialize)]
struct KeyData {
    public: String,
    private: String,
}
