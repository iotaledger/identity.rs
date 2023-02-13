use crate::error::JsValueResult;
use identity_iota::storage::key_storage::JwkGenOutput;
use identity_iota::storage::key_storage::JwkStorage;
use identity_iota::storage::key_storage::KeyId;
use identity_iota::storage::key_storage::KeyStorageError;
use identity_iota::storage::key_storage::KeyStorageErrorKind;
use identity_iota::storage::key_storage::KeyStorageResult;
use identity_iota::storage::key_storage::KeyType;
use identity_jose::jwk::Jwk;
use identity_jose::jws::JwsAlgorithm;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

use super::WasmJwk;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "Promise<JwkGenOutput>")]
  pub type PromiseJwkGenOutput;
  #[wasm_bindgen(typescript_type = "Promise<string>")]
  pub type PromiseKeyId;
  // #[wasm_bindgen(typescript_type = "Promise<ChainState | undefined>")]
  // pub type PromiseOptionChainState;
  // #[wasm_bindgen(typescript_type = "Promise<Document | undefined>")]
  // pub type PromiseOptionDocument;
  // #[wasm_bindgen(typescript_type = "Promise<KeyLocation>")]
  // pub type PromiseKeyLocation;
  // #[wasm_bindgen(typescript_type = "Promise<Array<DID>>")]
  // pub type PromiseArrayDID;
  // #[wasm_bindgen(typescript_type = "Promise<[DID, KeyLocation]>")]
  // pub type PromiseDIDKeyLocation;
  // #[wasm_bindgen(typescript_type = "Promise<EncryptedData>")]
  // pub type PromiseEncryptedData;
  // #[wasm_bindgen(typescript_type = "Promise<Uint8Array>")]
  // pub type PromiseData;
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "JwkStorage")]
  pub type WasmJwkStorage;

  #[wasm_bindgen(method)]
  pub fn generate(this: &WasmJwkStorage, key_type: String, algorithm: String) -> PromiseJwkGenOutput;

  #[wasm_bindgen(method)]
  pub fn insert(this: &WasmJwkStorage, jwk: WasmJwk) -> PromiseKeyId;
}

#[async_trait::async_trait(?Send)]
impl JwkStorage for WasmJwkStorage {
  async fn generate(&self, key_type: KeyType, alg: JwsAlgorithm) -> KeyStorageResult<JwkGenOutput> {
    let promise: Promise = Promise::resolve(&WasmJwkStorage::generate(&self, key_type.into(), alg.name().to_owned()));
    let result: JsValueResult = JsFuture::from(promise).await.into();

    result
      .to_key_storage_error()?
      .into_serde()
      .map_err(|err| KeyStorageError::new(KeyStorageErrorKind::SerializationError).with_source(err))
  }

  async fn insert(&self, jwk: Jwk) -> KeyStorageResult<KeyId> {
    let promise: Promise = Promise::resolve(&WasmJwkStorage::insert(&self, WasmJwk::new(jwk)));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    let js_value: JsValue = result.to_key_storage_error()?;
    js_value
      .into_serde()
      .map_err(|err| KeyStorageError::new(KeyStorageErrorKind::SerializationError).with_source(err))
  }

  async fn sign(&self, key_id: &KeyId, data: Vec<u8>) -> KeyStorageResult<Vec<u8>> {
    todo!()
  }

  async fn public(&self, key_id: &KeyId) -> KeyStorageResult<Jwk> {
    todo!()
  }

  async fn delete(&self, key_id: &KeyId) -> KeyStorageResult<()> {
    todo!()
  }

  async fn exists(&self, key_id: &KeyId) -> KeyStorageResult<bool> {
    todo!()
  }
}
