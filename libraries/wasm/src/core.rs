use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use identity_core::{
    did::DID,
    document::DIDDocument,
    utils::{Context, Subject},
};
#[wasm_bindgen]
#[derive(Serialize)]
pub struct Core {}

#[wasm_bindgen]
impl Core {
    #[wasm_bindgen]
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
    #[wasm_bindgen(js_name = "createDIDDocument")]
    pub fn create_did_document(did: &str) -> Result<String, JsValue> {
        console_error_panic_hook::set_once();

        let mut did_doc = DIDDocument {
            context: Context::from("https://w3id.org/did/v1"),
            id: Subject::from(did),
            ..Default::default()
        }
        .init();

        Ok(did_doc.to_string())
    }
}
