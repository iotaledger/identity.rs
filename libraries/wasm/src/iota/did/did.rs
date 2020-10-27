use identity_iota::did::IotaDID;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct JSIotaDID {
    did: IotaDID,
}

#[wasm_bindgen(js_name = "IotaDID")]
impl JSIotaDID {
    #[wasm_bindgen(constructor)]
    pub fn new(id: String) -> Self {
        console_error_panic_hook::set_once();
        let iota_did = IotaDID::new(&id.as_bytes()).unwrap();
        JSIotaDID { did: iota_did }
    }
    #[wasm_bindgen(js_name = "CreateIotaDIDWithNetwork")]
    pub fn new_with_network(id: String, network: String) -> Self {
        console_error_panic_hook::set_once();
        let iota_did = IotaDID::with_network(&id.as_bytes(), &network).unwrap();
        JSIotaDID { did: iota_did }
    }

    #[wasm_bindgen(getter)]
    pub fn did(&self) -> String {
        console_error_panic_hook::set_once();
        self.did.to_string()
    }
}
