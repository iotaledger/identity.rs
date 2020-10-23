use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use identity_core::did::DID;

#[wasm_bindgen]
#[derive(Serialize)]
pub struct JSDID {
    did: DID,
}

#[wasm_bindgen(js_name = "DID")]
impl JSDID {
    #[wasm_bindgen(constructor)]
    pub fn new(id: String) -> Result<JsValue, JsValue> {
        console_error_panic_hook::set_once();

        let did = DID {
            method_name: "iota".into(),
            id_segments: vec![id.into()],
            ..Default::default()
        }
        .init()
        .unwrap();

        let object = JSDID {
            did
        };
        Ok(JsValue::from(object))
    }

    #[wasm_bindgen(getter)]
    pub fn did(&self) -> String {
        self.did.to_string().clone()
    }
 }