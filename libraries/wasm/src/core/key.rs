use identity_proof::signature::jcsed25519signature2020;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct KeyPair {
    publicKey: String,
    privateKey: String,
}

#[wasm_bindgen]
#[allow(non_snake_case)]
impl KeyPair {
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

impl Default for KeyPair {
    fn default() -> Self {
        Self::new()
    }
}
