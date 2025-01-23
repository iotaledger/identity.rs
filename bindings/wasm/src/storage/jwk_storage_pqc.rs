// Copyright 2024 Fondazione Links
// SPDX-License-Identifier: Apache-2.0

use super::WasmJwkStorage;
use identity_iota::storage::JwkGenOutput;
use identity_iota::storage::JwkStoragePQ;
use identity_iota::storage::KeyId;
use identity_iota::storage::KeyStorageResult;
use identity_iota::storage::KeyType;
use identity_iota::verification::jwk::Jwk;
use wasm_bindgen::prelude::*;
use crate::error::JsValueResult;
use js_sys::Promise;
use identity_iota::storage::KeyStorageErrorKind;
use identity_iota::verification::jose::jws::JwsAlgorithm;
use identity_iota::storage::KeyStorageError;
use wasm_bindgen_futures::JsFuture;
use super::jwk_storage::PromiseJwkGenOutput;
use js_sys::Array;
use crate::jose::WasmJwk;
use js_sys::Uint8Array;

#[wasm_bindgen]
extern "C" {

  #[wasm_bindgen(method, js_name = generatePQKey)]
  pub fn _generate_pq_key(this: &WasmJwkStorage, key_type: String, alg: String) -> PromiseJwkGenOutput;

}


#[async_trait::async_trait(?Send)]
impl JwkStoragePQ for WasmJwkStorage {
  async fn generate_pq_key(&self, key_type: KeyType, alg: JwsAlgorithm) -> KeyStorageResult<JwkGenOutput> {
    let promise: Promise = Promise::resolve(&WasmJwkStorage::_generate_pq_key(self, key_type.into(), alg.name().to_owned()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }

  async fn pq_sign(&self, key_id: &KeyId, data: &[u8], public_key: &Jwk) -> KeyStorageResult<Vec<u8>> {
    let promise: Promise = Promise::resolve(&WasmJwkStorage::sign(
      self,
      key_id.clone().into(),
      data.to_owned(),
      WasmJwk(public_key.clone()),
    ));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.to_key_storage_error().map(uint8array_to_bytes)?
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

fn uint8array_to_bytes(value: JsValue) -> KeyStorageResult<Vec<u8>> {
  if !JsCast::is_instance_of::<Uint8Array>(&value) {
    return Err(
      KeyStorageError::new(KeyStorageErrorKind::SerializationError)
        .with_custom_message("expected Uint8Array".to_owned()),
    );
  }
  let array_js_value = JsValue::from(Array::from(&value));
  array_js_value
    .into_serde()
    .map_err(|e| KeyStorageError::new(KeyStorageErrorKind::SerializationError).with_custom_message(e.to_string()))
}