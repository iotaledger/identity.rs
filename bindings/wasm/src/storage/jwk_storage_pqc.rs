use super::WasmJwkStorage;

use identity_iota::storage::JwkGenOutput;
use identity_iota::storage::JwkStoragePQ;
use identity_iota::storage::KeyId;
use identity_iota::storage::KeyStorageResult;
use identity_iota::storage::KeyType;
use identity_iota::verification::jwk::Jwk;
use wasm_bindgen::prelude::*;
use identity_iota::verification::jose::jws::JwsAlgorithm;
use crate::error::JsValueResult;
use js_sys::Promise;
use wasm_bindgen_futures::JsFuture;
use super::jwk_storage::PromiseJwkGenOutput;

#[wasm_bindgen]
extern "C" {

  #[wasm_bindgen(method, js_name = generatePQKey)]
  pub fn _generate_pq_key(this: &WasmJwkStorage, key_type: String, alg: String) -> PromiseJwkGenOutput;

}


#[async_trait::async_trait(?Send)]
impl JwkStoragePQ for WasmJwkStorage {
  async fn generate_pq_key(&self, key_type: KeyType, alg: JwsAlgorithm) -> KeyStorageResult<JwkGenOutput> {
    web_sys::console::log_1(&"YYYYYYYYYYYYYYY".into());
    let promise: Promise = Promise::resolve(&WasmJwkStorage::_generate_pq_key(self, key_type.into(), alg.name().to_owned()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }

  async fn pq_sign(&self, _key_id: &KeyId, _data: &[u8], _public_key: &Jwk) -> KeyStorageResult<Vec<u8>> {
    todo!();
  }

}

#[wasm_bindgen(typescript_custom_section)]
const JWK_STORAGE_PQ: &'static str = r#"
/** Secure storage for cryptographic keys represented as JWKs. */
interface JwkStoragePQ {
  /** Generate a new key represented as a JSON Web Key.
   * 
   * It's recommend that the implementer exposes constants for the supported key type string. */
  generatePQKey: (keyType: string, algorithm: JwsAlgorithm) => Promise<JwkGenOutput>;
}"#;