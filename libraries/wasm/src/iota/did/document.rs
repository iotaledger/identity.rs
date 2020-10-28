use identity_core::{
    common::AsJson,
    did::{DIDDocument, DID},
    key::{KeyData, KeyType, PublicKey, PublicKeyBuilder},
    utils::encode_b58,
};
use identity_crypto::KeyPair;
use identity_iota::did::{DIDDiff, IotaDID, IotaDocument as _IotaDocument};
use std::str::FromStr;
use wasm_bindgen::prelude::*;

//remove Keypair from core and rename it here to Keypair?
#[wasm_bindgen]
pub struct Key(KeyPair);

#[wasm_bindgen]
impl Key {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(_IotaDocument::generate_ed25519_keypair())
    }

    #[wasm_bindgen(getter)]
    pub fn public(&self) -> String {
        encode_b58(self.0.public())
    }

    #[wasm_bindgen(getter)]
    pub fn secret(&self) -> String {
        encode_b58(self.0.secret())
    }
}

impl Default for Key {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
pub struct IotaDocument {
    document: _IotaDocument,
}

#[wasm_bindgen]
impl IotaDocument {
    #[wasm_bindgen(constructor)]
    pub fn new(did: String, auth_key: String) -> Self {
        console_error_panic_hook::set_once();
        let did = IotaDID::parse(did).unwrap();
        let key: DID = format!("{}#key-1", did).parse().unwrap();

        let public_key: PublicKey = PublicKeyBuilder::default()
            .id(key)
            .controller(DID::parse(did.clone()).unwrap())
            .key_type(KeyType::Ed25519VerificationKey2018)
            .key_data(KeyData::PublicKeyBase58(auth_key))
            .build()
            .unwrap();
        let iota_document = _IotaDocument::new(did, public_key).unwrap();
        IotaDocument {
            document: iota_document,
        }
    }

    #[wasm_bindgen(js_name = "TryFromDocument")]
    pub fn try_from_document(document_string: String) -> Self {
        console_error_panic_hook::set_once();
        let document = DIDDocument::from_str(&document_string).unwrap();
        let iota_doc = _IotaDocument::try_from_document(document).unwrap();
        IotaDocument { document: iota_doc }
    }

    #[wasm_bindgen(getter)]
    pub fn document(&self) -> String {
        console_error_panic_hook::set_once();
        self.document.to_string()
    }
    #[wasm_bindgen(getter)]
    pub fn did(&self) -> String {
        self.document.did().to_string()
    }
    #[wasm_bindgen(getter)]
    pub fn authentication_key(&self) -> String {
        let key = self.document.authentication_key();
        match key.key_data() {
            KeyData::PublicKeyBase58(_key) => _key.to_string(),
            _ => panic!("Can't get key as string"),
        }
    }
    #[wasm_bindgen(getter)]
    pub fn create_diff_address(&self) -> String {
        _IotaDocument::create_diff_address_hash(self.authentication_key().as_bytes())
    }

    //How can we convert the key?
    #[wasm_bindgen(js_name = "sign")]
    pub fn sign(&mut self, secret: &Key) -> String {
        console_error_panic_hook::set_once();
        self.document.sign(secret.0.secret()).unwrap();
        self.document.to_string()
    }

    #[wasm_bindgen(js_name = "generate_ed25519_keypair")]
    pub fn generate_ed25519_keypair(&self) -> Key {
        Key(_IotaDocument::generate_ed25519_keypair())
    }
    #[wasm_bindgen(js_name = "verify")]
    pub fn verify(&self) -> bool {
        console_error_panic_hook::set_once();
        matches!(self.document.verify(), Ok(_))
    }
    #[wasm_bindgen(js_name = "diff")]
    pub fn diff(&self, other: &IotaDocument, key: &Key) -> String {
        console_error_panic_hook::set_once();

        self.document
            .diff(
                DIDDocument::from_str(&other.document.to_string()).unwrap(),
                &key.0.secret(),
            )
            .unwrap()
            .to_json()
            .unwrap()
    }
    #[wasm_bindgen(js_name = "verify_diff")]
    pub fn verify_diff(&self, diff: String) -> bool {
        console_error_panic_hook::set_once();
        let diff = DIDDiff::from_json(&diff).unwrap();
        matches!(self.document.verify_diff(&diff), Ok(_))
    }
}
