// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::did_doc::{Method, MethodBuilder, MethodData};
use identity_iota::did::IotaDID;
use wasm_bindgen::prelude::*;

use crate::{did::DID, js_err};

pub const DEFAULT_KEY: &str = "Ed25519VerificationKey2018";
pub const DEFAULT_TAG: &str = "authentication";

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct PubKey(pub(crate) Method);

#[wasm_bindgen]
impl PubKey {
    // TODO: Support non-base58 key data
    #[wasm_bindgen(constructor)]
    pub fn new(did: &DID, key_type: &str, key_data: &str, tag: Option<String>) -> Result<PubKey, JsValue> {
        let tag: &str = tag.as_deref().unwrap_or(DEFAULT_TAG);
        let key: _ = format!("{}#{}", did.0, tag).parse().map_err(js_err)?;

        MethodBuilder::default()
            .id(key)
            .controller(did.0.clone().into())
            .key_type(key_type.parse().map_err(js_err)?)
            .key_data(MethodData::PublicKeyBase58(key_data.into()))
            .build()
            .map_err(js_err)
            .map(Self)
    }

    /// Generates a new `PubKey` object suitable for ed25519 signatures.
    #[wasm_bindgen(js_name = generateEd25519)]
    pub fn generate_ed25519(did: &DID, key_data: &str, tag: Option<String>) -> Result<PubKey, JsValue> {
        Self::new(did, DEFAULT_KEY, key_data, tag)
    }

    /// Returns the `id` DID of the `PubKey` object.
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> Result<DID, JsValue> {
        IotaDID::try_from_owned(self.0.id().clone()).map_err(js_err).map(DID)
    }

    /// Returns the `controller` DID of the `PubKey` object.
    #[wasm_bindgen(getter)]
    pub fn controller(&self) -> Result<DID, JsValue> {
        IotaDID::try_from_owned(self.0.controller().clone())
            .map_err(js_err)
            .map(DID)
    }

    /// Serializes a `PubKey` object as a JSON object.
    #[wasm_bindgen(js_name = toJSON)]
    pub fn to_json(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(js_err)
    }

    /// Deserializes a `PubKey` object from a JSON object.
    #[wasm_bindgen(js_name = fromJSON)]
    pub fn from_json(json: &JsValue) -> Result<PubKey, JsValue> {
        json.into_serde().map_err(js_err).map(Self)
    }
}
