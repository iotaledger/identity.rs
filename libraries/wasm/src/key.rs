use identity_core::utils::encode_b58;
use identity_crypto::KeyPair;
use identity_proof::signature::jcsed25519signature2020;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct Key(pub(crate) KeyPair);

#[wasm_bindgen]
impl Key {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(jcsed25519signature2020::new_keypair())
    }

    #[wasm_bindgen(getter)]
    pub fn public(&self) -> String {
        encode_b58(self.0.public())
    }

    #[wasm_bindgen(getter)]
    pub fn secret(&self) -> String {
        encode_b58(self.0.secret())
    }
}
