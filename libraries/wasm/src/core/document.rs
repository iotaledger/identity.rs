use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use identity_core::did::{DID, DIDDocument, DIDDocumentBuilder};

#[wasm_bindgen]
#[derive(Serialize, Clone)]
pub struct JSDIDDocument {
    document: DIDDocument,
}

#[wasm_bindgen(js_name = "DIDDocument")]
impl JSDIDDocument {
    #[wasm_bindgen(constructor)]
    pub fn new(did: String) -> Result<JsValue, JsValue> {
        console_error_panic_hook::set_once();

        let mut document = DIDDocumentBuilder::default()
        .context(vec![DID::BASE_CONTEXT.into()])
        .id("did:iota:123456789abcdefghi".parse().unwrap())
        .build()
        .unwrap();

        let object = JSDIDDocument {
            document
        };
        Ok(JsValue::from(object))
    }

    #[wasm_bindgen(getter)]
    pub fn document(&self) -> String {
        self.document.to_string().clone()
    }
 }