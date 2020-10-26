use identity_core::{
    common::OneOrMany,
    did::{DIDDocument, DIDDocumentBuilder, DID},
    key::{KeyData, KeyType, PublicKey, PublicKeyBuilder},
};
use serde::Serialize;
use wasm_bindgen::prelude::*;

use multihash::Blake2b256;

#[wasm_bindgen]
#[derive(Serialize)]
pub struct JSDIDDocument {
    document: DIDDocument,
}

#[wasm_bindgen(js_name = "DIDDocument")]
impl JSDIDDocument {
    #[wasm_bindgen(constructor)]
    pub fn new(auth_key: String) -> Result<JsValue, JsValue> {
        console_error_panic_hook::set_once();

        let did: DID = create_method_id(&auth_key, Some("com"), None).unwrap();
        let key: DID = format!("{}#key-1", did).parse().unwrap();
        let public_key: PublicKey = PublicKeyBuilder::default()
            .id(key.clone())
            .controller(did.clone())
            .key_type(KeyType::Ed25519VerificationKey2018)
            .key_data(KeyData::PublicKeyBase58(auth_key))
            .build()
            .unwrap();
        let document: DIDDocument = DIDDocumentBuilder::default()
            .context(OneOrMany::One(DID::BASE_CONTEXT.into()))
            .id(did)
            .public_keys(vec![public_key])
            .auth(vec![key.into()])
            .build()
            .unwrap();

        let object = JSDIDDocument { document };

        Ok(JsValue::from(object))
    }

    #[wasm_bindgen(getter)]
    pub fn document(&self) -> String {
        console_error_panic_hook::set_once();
        self.document.to_string()
    }
    // #[wasm_bindgen(setter)]
    // pub fn set_sign_unchecked(&self, id: String) {
    //     // hmm sign_unchecked is not here anymore :(
    //it's in identity_iota
    // }
}

pub fn create_method_id(pub_key: &str, network: Option<&str>, network_shard: Option<String>) -> Result<DID, JsValue> {
    let hash = Blake2b256::digest(pub_key.as_bytes());
    let bs58key = bs58::encode(&hash.digest()).into_string();
    let network_string = match network {
        Some(network_str) => match network_str {
            "com" => "com:".to_string(),
            "dev" => "dev:".to_string(),
            _ => "".to_string(),
        },
        _ => "".to_string(), // default: "main" also can be written as ""
    };
    let shard_string = match &network_shard {
        Some(shard) => format!("{}:", shard),
        _ => String::new(),
    };
    let id_string = format!("did:iota:{}{}{}", network_string, shard_string, bs58key);
    Ok(DID::parse_from_str(id_string).unwrap())
}
