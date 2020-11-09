use identity_core::{
    common::AsJson as _,
    key::{KeyData, KeyType, PublicKey, PublicKeyBuilder},
};
use identity_iota::did::IotaDID;
use wasm_bindgen::prelude::*;

use crate::{did::DID, js_err};

pub const DEFAULT_KEY: &str = "Ed25519VerificationKey2018";
pub const DEFAULT_TAG: &str = "authentication";

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct PubKey(pub(crate) PublicKey);

#[wasm_bindgen]
impl PubKey {
    // TODO: Support non-base58 key data
    #[wasm_bindgen(constructor)]
    pub fn new(did: &DID, key_type: &str, key_data: &str, tag: Option<String>) -> Result<PubKey, JsValue> {
        let tag: &str = tag.as_deref().unwrap_or(DEFAULT_TAG);
        let key: _ = format!("{}#{}", did.0, tag).parse().map_err(js_err)?;

        PublicKeyBuilder::default()
            .id(key)
            .controller(did.0.clone().into())
            .key_type(KeyType::try_from_str(key_type).map_err(js_err)?)
            .key_data(KeyData::PublicKeyBase58(key_data.into()))
            .build()
            .map_err(Into::into)
            .map(Self)
    }

    /// Generates a new `PubKey` object suitable for ed25519 signatures.
    #[wasm_bindgen(js_name = ed25519)]
    pub fn ed25519(did: &DID, key_data: &str, tag: Option<String>) -> Result<PubKey, JsValue> {
        Self::new(did, DEFAULT_KEY, key_data, tag)
    }

    /// Returns the `id` DID of the `PubKey` object.
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> Result<DID, JsValue> {
        IotaDID::try_from_did(self.0.id().clone()).map_err(js_err).map(DID)
    }

    /// Returns the `controller` DID of the `PubKey` object.
    #[wasm_bindgen(getter)]
    pub fn controller(&self) -> Result<DID, JsValue> {
        IotaDID::try_from_did(self.0.controller().clone())
            .map_err(js_err)
            .map(DID)
    }

    /// Serializes a `PubKey` object as a JSON string.
    #[wasm_bindgen(js_name = toJSON)]
    pub fn to_json(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(js_err)
    }

    /// Deserializes a `PubKey` object from a JSON string.
    #[wasm_bindgen(js_name = fromJSON)]
    pub fn from_json(json: &str) -> Result<PubKey, JsValue> {
        PublicKey::from_json(json).map_err(js_err).map(Self)
    }
}
