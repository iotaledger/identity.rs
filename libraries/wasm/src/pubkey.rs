use identity_core::{
    did::DID as CoreDID,
    key::{KeyData, KeyType, PublicKey, PublicKeyBuilder},
};
use serde::Serialize;
use wasm_bindgen::prelude::*;

use crate::{did::DID, js_err};

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PubKey(pub(crate) PublicKey);

#[wasm_bindgen]
impl PubKey {
    #[wasm_bindgen(constructor)]
    pub fn new(id: &DID, controller: &DID, key_data: String) -> Result<PubKey, JsValue> {
        Ok(Self(
            PublicKeyBuilder::default()
                .id(CoreDID::parse(id.0.clone()).map_err(js_err)?)
                .key_type(KeyType::Ed25519VerificationKey2018)
                .controller(CoreDID::parse(controller.0.clone()).map_err(js_err)?)
                .key_data(KeyData::PublicKeyBase58(key_data))
                .build()?,
        ))
    }
    #[wasm_bindgen]
    pub fn id(&self) -> Result<DID, JsValue> {
        Ok(DID::parse_from_did(self.0.id().clone())?)
    }

    #[wasm_bindgen]
    pub fn controller(&self) -> Result<DID, JsValue> {
        Ok(DID::parse_from_did(self.0.controller().clone())?)
    }

    #[wasm_bindgen(getter)]
    pub fn pubkey(&self) -> JsValue {
        JsValue::from_serde(&self.0).ok().unwrap_or(JsValue::NULL)
    }
}
