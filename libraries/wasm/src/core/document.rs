use identity_core::{
    common::OneOrMany,
    did::{DIDDocument as _DIDDocument, DIDDocumentBuilder, DID},
    key::{KeyData, KeyType, PublicKey, PublicKeyBuilder},
};
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize)]
pub struct DIDDocument {
    document: _DIDDocument,
}

#[wasm_bindgen]
impl DIDDocument {
    #[wasm_bindgen(constructor)]
    pub fn new(method_name: String, auth_key: String) -> Self {
        console_error_panic_hook::set_once();

        let id_string = format!("did:{}:{}", method_name, auth_key);
        let did = DID::parse(id_string).unwrap();
        let key: DID = format!("{}#key-1", did).parse().unwrap();
        let public_key: PublicKey = PublicKeyBuilder::default()
            .id(key.clone())
            .controller(did.clone())
            .key_type(KeyType::Ed25519VerificationKey2018)
            .key_data(KeyData::PublicKeyBase58(auth_key))
            .build()
            .unwrap();
        let document: _DIDDocument = DIDDocumentBuilder::default()
            .context(OneOrMany::One(DID::BASE_CONTEXT.into()))
            .id(did)
            .public_keys(vec![public_key])
            .auth(vec![key.into()])
            .build()
            .unwrap();

        DIDDocument { document }
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
