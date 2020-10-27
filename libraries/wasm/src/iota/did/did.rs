use identity_iota::did::IotaDID as _IotaDID;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub struct IotaDID {
    did: _IotaDID,
}

#[wasm_bindgen]
impl IotaDID {
    #[wasm_bindgen(constructor)]
    pub fn new(id: String) -> Self {
        console_error_panic_hook::set_once();
        let iota_did = _IotaDID::new(&id.as_bytes()).unwrap();
        IotaDID { did: iota_did }
    }
    #[wasm_bindgen(js_name = "CreateIotaDIDWithNetwork")]
    pub fn new_with_network(id: String, network: String) -> Self {
        console_error_panic_hook::set_once();
        let iota_did = _IotaDID::with_network(&id.as_bytes(), &network).unwrap();
        IotaDID { did: iota_did }
    }

    #[wasm_bindgen(getter)]
    pub fn did(&self) -> String {
        console_error_panic_hook::set_once();
        self.did.to_string()
    }
    #[wasm_bindgen(getter)]
    pub fn create_address(&self) -> String {
        console_error_panic_hook::set_once();
        self.did.create_address_hash()
    }
}
