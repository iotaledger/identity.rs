use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use identity_proof::signature::jcsed25519signature2020;
// use multihash::Blake2b256;
use identity_core::{common::OneOrMany, did::DIDDocument, did::{DID, DIDDocumentBuilder}};

mod did;
mod document;

#[wasm_bindgen]
#[derive(Serialize)]
pub struct Core {}

#[wasm_bindgen]
impl Core {
    #[wasm_bindgen(js_name = "CreateDID")]
    pub fn create_did(id: String) -> Result<String, JsValue> {
        console_error_panic_hook::set_once();

        let did = DID {
            method_name: "iota".into(),
            id_segments: vec![id.into()],
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
    pub fn generate_uuid(publicKey: String) -> Result<String, JsValue> {
        console_error_panic_hook::set_once();
        let bs58_auth_key = bs58::encode(publicKey).into_string();
        // console.log("bs58_auth_key: {:?}", bs58_auth_key);
       
        Ok(bs58_auth_key)
    }

    #[wasm_bindgen(js_name = "createDocument")]
    pub fn create_document(auth_key: String) -> Result<JsValue, JsValue> {
        // let did: DID = create_method_id(&auth_key, Some("Main"), None)?;
        // let key: DID = format!("{}#key-1", did).parse()?;

        let mut did_doc = DIDDocumentBuilder::default()
        .context(vec![DID::BASE_CONTEXT.into()])
        .id("did:iota:123456789abcdefghi".parse().unwrap())
        .build()
        .unwrap();

    
        // let public_key: PublicKey = PublicKeyBuilder::default()
        //     .id(key.clone())
        //     .controller(did.clone())
        //     .key_type(KeyType::Ed25519VerificationKey2018)
        //     .key_data(KeyData::PublicKeyBase58(auth_key))
        //     .build()
        //     .unwrap();
    
        // let doc: DIDDocument = DIDDocumentBuilder::default()
        //     .context(OneOrMany::One(DID::BASE_CONTEXT.into()))
        //     .id(did)
        //     .public_keys(vec![public_key])
        //     .auth(vec![key.into()])
        //     .build()
        //     .unwrap();
    
        Ok(JsValue::from("NIY"))
    }
}

#[wasm_bindgen]
#[derive(Serialize)]
pub struct JSKeyPair {
    publicKey: String,
    privateKey: String,
}

#[wasm_bindgen(js_name = "KeyPair")]
impl JSKeyPair {
    // will be accessible on JavaScript with `new Address(5.0, '###')`
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let keypair = jcsed25519signature2020::new_keypair();

        Self {
            publicKey: keypair.public().to_string(),
            privateKey: keypair.secret().to_string()
        }
    }

    #[wasm_bindgen(getter)]
    pub fn publicKey(&self) -> String {
        self.publicKey.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn privateKey(&self) -> String {
        self.privateKey.clone()
    }
}