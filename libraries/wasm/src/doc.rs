use identity_core::{
    common::FromJson as _,
    did::{DIDDocument, ServiceBuilder, ServiceEndpoint},
    key::{KeyIndex, KeyRelation},
};
use identity_iota::did::{DIDDiff, IotaDocument};
use wasm_bindgen::prelude::*;

use crate::{
    did::DID,
    js_err,
    key::Key,
    pubkey::{PubKey, DEFAULT_KEY},
};

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
    #[wasm_bindgen(constructor)]
    pub fn new(authentication: &PubKey) -> Result<Doc, JsValue> {
        IotaDocument::try_from_key(authentication.0.clone())
            .map_err(js_err)
            .map(Self)
    }

    #[wasm_bindgen(js_name = generateRandom)]
    pub fn generate_random(key_type: &str, network: Option<String>, tag: Option<String>) -> Result<NewDoc, JsValue> {
        let key: Key = Key::new(key_type)?;
        let did: DID = DID::new(&key, network)?;
        let pkey: PubKey = PubKey::new(&did, key_type, &key.public(), tag)?;

        Ok(NewDoc {
            doc: Self::new(&pkey)?,
            key,
        })
    }

    #[wasm_bindgen(js_name = generateEd25519)]
    pub fn generate_ed25519(network: Option<String>, tag: Option<String>) -> Result<NewDoc, JsValue> {
        Self::generate_random(DEFAULT_KEY, network, tag)
    }

    #[wasm_bindgen]
    pub fn did(&self) -> DID {
        DID(self.0.did().clone())
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.0.did().to_string()
    }

    #[wasm_bindgen(getter, js_name = authChain)]
    pub fn auth_chain(&self) -> String {
        self.0.did().create_address_hash()
    }

    #[wasm_bindgen(getter, js_name = diffChain)]
    pub fn diff_chain(&self) -> String {
        self.0.diff_address_hash()
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
        let doc: DIDDocument = other.0.clone().into();
        let diff: DIDDiff = self.0.diff(doc, key.0.secret()).map_err(js_err)?;

        JsValue::from_serde(&diff).map_err(js_err)
    }

    #[wasm_bindgen(js_name = verifyDiff)]
    pub fn verify_diff(&self, diff: String) -> bool {
        match DIDDiff::from_json(&diff) {
            Ok(diff) => self.0.verify_diff(&diff).is_ok(),
            Err(_) => false,
        }
    }

    #[wasm_bindgen(js_name = updateService)]
    pub fn update_service(&mut self, did: DID, url: String, service_type: String) -> Result<(), JsValue> {
        let service = ServiceBuilder::default()
            .id(did.0.into())
            .service_type(service_type)
            .endpoint(ServiceEndpoint::Url(url.parse().map_err(js_err)?))
            .build()?;

        self.0.update_service(service);

        Ok(())
    }

    #[wasm_bindgen(js_name = clearServices)]
    pub fn clear_services(&mut self) {
        self.0.clear_services();
    }

    #[wasm_bindgen(js_name = updatePublicKey)]
    pub fn update_public_key(&mut self, public_key: &PubKey) {
        self.0.update_public_key(public_key.0.clone());
    }

    #[wasm_bindgen(js_name = clearPublicKeys)]
    pub fn clear_public_keys(&mut self) {
        self.0.clear_public_keys();
    }

    #[wasm_bindgen(js_name = updateAuth)]
    pub fn update_auth(&mut self, public_key: &PubKey) {
        self.0.update_auth(public_key.0.clone());
    }

    #[wasm_bindgen(js_name = clearAuth)]
    pub fn clear_auth(&mut self) {
        self.0.clear_auth();
    }

    #[wasm_bindgen(js_name = updateAssert)]
    pub fn update_assert(&mut self, public_key: &PubKey) {
        self.0.update_assert(public_key.0.clone());
    }

    #[wasm_bindgen(js_name = clearAssert)]
    pub fn clear_assert(&mut self) {
        self.0.clear_assert();
    }

    #[wasm_bindgen(js_name = updateVerification)]
    pub fn update_verification(&mut self, public_key: &PubKey) {
        self.0.update_verification(public_key.0.clone());
    }

    #[wasm_bindgen(js_name = clearVerification)]
    pub fn clear_verification(&mut self) {
        self.0.clear_verification();
    }

    #[wasm_bindgen(js_name = updateDelegation)]
    pub fn update_delegation(&mut self, public_key: &PubKey) {
        self.0.update_delegation(public_key.0.clone());
    }

    #[wasm_bindgen(js_name = clearDelegation)]
    pub fn clear_delegation(&mut self) {
        self.0.clear_delegation();
    }

    #[wasm_bindgen(js_name = updateInvocation)]
    pub fn update_invocation(&mut self, public_key: &PubKey) {
        self.0.update_invocation(public_key.0.clone());
    }

    #[wasm_bindgen(js_name = clearInvocation)]
    pub fn clear_invocation(&mut self) {
        self.0.clear_invocation();
    }

    #[wasm_bindgen(js_name = updateAgreement)]
    pub fn update_agreement(&mut self, public_key: &PubKey) {
        self.0.update_agreement(public_key.0.clone());
    }

    #[wasm_bindgen(js_name = clearAgreement)]
    pub fn clear_agreement(&mut self) {
        self.0.clear_agreement();
    }

    #[wasm_bindgen(js_name = updateTime)]
    pub fn update_time(&mut self) {
        self.0.update_time();
    }

    #[wasm_bindgen(js_name = resolveKey)]
    pub fn resolve_key(&mut self, ident: JsValue, scope: Option<String>) -> Result<PubKey, JsValue> {
        let borrow: String;

        let ident: KeyIndex = if let Some(number) = ident.as_f64() {
            KeyIndex::Index(number.to_string().parse().map_err(js_err)?)
        } else if let Some(ident) = ident.as_string() {
            borrow = ident;
            KeyIndex::Ident(&borrow)
        } else {
            return Err("Invalid Key Identifier".into());
        };

        let scope: KeyRelation = scope
            .map(|scope| scope.parse::<KeyRelation>())
            .transpose()
            .map_err(js_err)?
            .unwrap_or(KeyRelation::Authentication);

        self.0
            .resolve_key(ident, scope)
            .cloned()
            .map(PubKey)
            .ok_or_else(|| "Key Not Found".into())
    }

    /// Serializes a `Doc` object as a JSON string.
    #[wasm_bindgen(js_name = toJSON)]
    pub fn to_json(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(js_err)
    }

    /// Deserializes a `Doc` object from a JSON string.
    #[wasm_bindgen(js_name = fromJSON)]
    pub fn from_json(json: &str) -> Result<Doc, JsValue> {
        IotaDocument::from_json(json).map_err(js_err).map(Self)
    }
}
