use wasm_bindgen::prelude::*;

use identity::account::derive_encryption_key as derive_encryption_key_;
use identity::account::EncryptionKey as EncryptionKey_;

#[wasm_bindgen]
pub struct WasmEncryptionKey(EncryptionKey_);

#[wasm_bindgen]
impl WasmEncryptionKey {
  #[wasm_bindgen(js_name = deriveEncryptionKey)]
  pub fn derive_encryption_key(password: &str) -> WasmEncryptionKey {
    WasmEncryptionKey(derive_encryption_key_(password))
  }
}
