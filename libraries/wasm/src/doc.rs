use identity_core::{
    common::FromJson as _,
    key::{KeyData, KeyType, PublicKey, PublicKeyBuilder},
    utils::{decode_b58, encode_b58},
};
use identity_iota::did::{DIDDiff, IotaDID, IotaDocument};
use wasm_bindgen::prelude::*;

use crate::{js_err, key::Key};

#[derive(Debug, Deserialize)]
pub struct DocParams {
    key: String,
    did: Option<String>,
    tag: Option<String>,
}

const DEFAULT_TAG: &str = "authentication";

#[wasm_bindgen(inspectable)]
pub struct NewDoc {
    key: Key,
    doc: Doc,
}

#[wasm_bindgen]
impl NewDoc {
    #[wasm_bindgen(getter)]
    pub fn key(&self) -> Key {
        self.key.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn doc(&self) -> Doc {
        self.doc.clone()
    }
}

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct Doc(pub(crate) IotaDocument);

#[wasm_bindgen]
impl Doc {
    fn create(did: IotaDID, public: String, tag: Option<&str>) -> Result<Self, JsValue> {
        let tag: &str = tag.unwrap_or(DEFAULT_TAG);
        let key: IotaDID = format!("{}#{}", did, tag).parse().map_err(js_err)?;

        let method: PublicKey = PublicKeyBuilder::default()
            .id(key.into())
            .controller(did.clone().into())
            .key_type(KeyType::Ed25519VerificationKey2018)
            .key_data(KeyData::PublicKeyBase58(public))
            .build()
            .map_err(js_err)?;

        IotaDocument::new(did, method).map_err(js_err).map(Self)
    }

    fn create_did(key: impl AsRef<str>) -> Result<IotaDID, JsValue> {
        decode_b58(key.as_ref())
            .map_err(js_err)
            .and_then(|key| IotaDID::new(&key).map_err(js_err))
    }

    #[wasm_bindgen(js_name = fromJSON)]
    pub fn from_json(json: String) -> Result<Doc, JsValue> {
        IotaDocument::from_json(&json).map_err(js_err).map(Self)
    }

    #[wasm_bindgen]
    pub fn generate() -> Result<NewDoc, JsValue> {
        let key: Key = Key::new();
        let did: IotaDID = IotaDID::new(key.public().as_ref()).map_err(js_err)?;
        let doc: Self = Self::create(did, key.public(), None)?;

        Ok(NewDoc { doc, key })
    }

    #[wasm_bindgen(js_name = generateCom)]
    pub fn generate_com() -> Result<NewDoc, JsValue> {
        let key: Key = Key::new();
        let did: IotaDID = IotaDID::with_network(key.public().as_ref(), "com").map_err(js_err)?;
        let doc: Self = Self::create(did, key.public(), None)?;

        Ok(NewDoc { doc, key })
    }

    #[wasm_bindgen(constructor)]
    pub fn new(params: &JsValue) -> Result<Doc, JsValue> {
        if params.is_object() {
            let params: DocParams = params.into_serde().map_err(js_err)?;

            let did: IotaDID = if let Some(ref did) = params.did {
                IotaDID::parse(did).map_err(js_err)?
            } else {
                Self::create_did(&params.key)?
            };

            Self::create(did, params.key, params.tag.as_deref())
        } else if let Some(public) = params.as_string() {
            Self::create(Self::create_did(&public)?, public, None)
        } else {
            panic!("Invalid Arguments for `new Doc(..)`");
        }
    }

    #[wasm_bindgen(getter)]
    pub fn did(&self) -> String {
        self.0.did().to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn auth_chain(&self) -> String {
        self.0.did().create_address_hash()
    }

    #[wasm_bindgen(getter)]
    pub fn diff_chain(&self) -> String {
        self.0.diff_address_hash()
    }

    #[wasm_bindgen(getter)]
    pub fn authentication_key(&self) -> String {
        encode_b58(&self.0.authentication_key_bytes())
    }

    #[wasm_bindgen(getter)]
    pub fn document(&self) -> JsValue {
        JsValue::from_serde(&self.0).ok().unwrap_or(JsValue::NULL)
    }

    #[wasm_bindgen(getter)]
    pub fn proof(&self) -> JsValue {
        self.0
            .metadata()
            .get("proof")
            .and_then(|value| JsValue::from_serde(value).ok())
            .unwrap_or(JsValue::NULL)
    }

    #[wasm_bindgen]
    pub fn sign(&mut self, key: &Key) -> Result<JsValue, JsValue> {
        self.0.sign(key.0.secret()).map_err(js_err).map(|_| JsValue::NULL)
    }

    #[wasm_bindgen]
    pub fn verify(&self) -> bool {
        self.0.verify().is_ok()
    }

    #[wasm_bindgen]
    pub fn diff(&self, other: &Doc, key: &Key) -> Result<JsValue, JsValue> {
        let diff: DIDDiff = self.0.diff(&other.0, key.0.secret()).map_err(js_err)?;

        JsValue::from_serde(&diff).map_err(js_err)
    }

    #[wasm_bindgen]
    pub fn verify_diff(&self, diff: String) -> bool {
        match DIDDiff::from_json(&diff) {
            Ok(diff) => self.0.verify_diff(&diff).is_ok(),
            Err(_) => false,
        }
    }
}
