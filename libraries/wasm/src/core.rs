use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use identity_core::{common::OneOrMany, did::DIDDocument, did::DIDDocumentBuilder, did::DID};
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

        let doc: DIDDocument = DIDDocumentBuilder::default()
            .context(OneOrMany::One(DID::BASE_CONTEXT.into()))
            .id(DID::parse_from_str(did).unwrap())
            .build()
            .unwrap();

        Ok(doc.to_string())
    }
}
