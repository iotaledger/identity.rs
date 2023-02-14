mod jwk_gen_output;
mod jwk_storage;
mod jws_algorithm;
mod wasm_jwk;

pub use jwk_storage::*;
pub use wasm_jwk::*;

pub mod athing {
  use super::*;
  use identity_iota::storage::key_storage::JwkGenOutput;
  use identity_iota::storage::key_storage::JwkStorage;
  use identity_iota::storage::key_storage::KeyType;
  use identity_jose::jws::JwsAlgorithm;
  use js_sys::Promise;
  use wasm_bindgen::prelude::*;
  use wasm_bindgen_futures::future_to_promise;

  #[wasm_bindgen]
  pub fn call_storage(storage: WasmJwkStorage) -> Promise {
    future_to_promise(async move {
      let JwkGenOutput { key_id, .. } = JwkStorage::generate(&storage, KeyType::new("Ed25519"), JwsAlgorithm::EdDSA)
        .await
        .unwrap();

      let sig = JwkStorage::sign(&storage, &key_id, b"test".to_vec()).await.unwrap();

      Ok(JsValue::from(format!("{sig:?}")))
    })
  }
}
