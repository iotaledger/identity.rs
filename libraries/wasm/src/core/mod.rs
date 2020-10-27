use identity_proof::signature::jcsed25519signature2020;
use serde::Serialize;
use wasm_bindgen::prelude::*;
// use multihash::Blake2b256;
use identity_core::did::{DIDDocumentBuilder, DID};

mod did;
mod document;

#[wasm_bindgen]
#[derive(Serialize)]
#[allow(non_camel_case_types)]
pub struct core {}

#[wasm_bindgen]
impl core {
    #[wasm_bindgen(js_name = "CreateDID")]
    pub fn create_did(method_name: String, id: String) -> Result<String, JsValue> {
        console_error_panic_hook::set_once();

        let did = DID {
            method_name,
            id_segments: vec![id],
            ..Default::default()
        }
        .init()
        .unwrap();
        Ok(did.to_string())
    }
    // #[wasm_bindgen(js_name = "createDIDDocument")]
    // pub fn create_did_document(did: &str) -> Result<String, JsValue> {
    //     console_error_panic_hook::set_once();

    //     let doc: DIDDocument = DIDDocumentBuilder::default()
    //         .context(OneOrMany::One(DID::BASE_CONTEXT.into()))
    //         .id(DID::parse_from_str(did).unwrap())
    //         .build()
    //         .unwrap();

    //     Ok(doc.to_string())
    // }

    #[wasm_bindgen(js_name = "GenerateKeypair")]
    pub fn generate_keypair() -> Result<JsValue, JsValue> {
        console_error_panic_hook::set_once();
        //     let keypair = Ed25519::generate(&Ed25519, Default::default())?;

        //let bs58_auth_key = bs58::encode(keypair.public()).into_string();
        // console.log("bs58_auth_key: {:?}", bs58_auth_key);
        let js_object = JSKeyPair::new();
        Ok(JsValue::from(js_object))
    }

    #[wasm_bindgen(js_name = "GenerateUUID")]
    pub fn generate_uuid(private_key: String) -> Result<String, JsValue> {
        console_error_panic_hook::set_once();
        let bs58_auth_key = bs58::encode(private_key).into_string();
        // console.log("bs58_auth_key: {:?}", bs58_auth_key);
        Ok(bs58_auth_key)
    }

    #[wasm_bindgen(js_name = "createDocument")]
    pub fn create_document(_auth_key: String) -> Result<JsValue, JsValue> {
        // let did: DID = create_method_id(&auth_key, Some("Main"), None)?;
        // let key: DID = format!("{}#key-1", did).parse()?;

        let did_doc = DIDDocumentBuilder::default()
            .context(vec![DID::BASE_CONTEXT.into()])
            .id("did:iota:123456789abcdefghi".parse().unwrap())
            .build()
            .unwrap();

        // let publicKey: PublicKey = PublicKeyBuilder::default()
        //     .id(key.clone())
        //     .controller(did.clone())
        //     .key_type(KeyType::Ed25519VerificationKey2018)
        //     .key_data(KeyData::PublicKeyBase58(auth_key))
        //     .build()
        //     .unwrap();
        // let doc: DIDDocument = DIDDocumentBuilder::default()
        //     .context(OneOrMany::One(DID::BASE_CONTEXT.into()))
        //     .id(did)
        //     .publicKeys(vec![publicKey])
        //     .auth(vec![key.into()])
        //     .build()
        //     .unwrap();
        Ok(JsValue::from(&did_doc.to_string()))
    }
}

#[wasm_bindgen]
#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct JSKeyPair {
    publicKey: String,
    privateKey: String,
}

#[wasm_bindgen(js_name = "KeyPair")]
#[allow(non_snake_case)]
impl JSKeyPair {
    // will be accessible on JavaScript with `new Address(5.0, '###')`

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        let keypair = jcsed25519signature2020::new_keypair();

        Self {
            publicKey: keypair.public().to_string(),
            privateKey: keypair.secret().to_string(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn publicKey(&self) -> String {
        console_error_panic_hook::set_once();

        self.publicKey.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn privateKey(&self) -> String {
        console_error_panic_hook::set_once();

        self.privateKey.clone()
    }
}

impl Default for JSKeyPair {
    fn default() -> Self {
        Self::new()
    }
}
