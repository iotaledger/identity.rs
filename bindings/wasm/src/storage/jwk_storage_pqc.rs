use std::str::FromStr;

use crate::error::Result as WasmResult;
use crate::error::WasmResult as _;
use crate::jose::WasmJwk;
use crate::jose::WasmJwsAlgorithm;

use super::WasmJwkGenOutput;
use super::WasmJwkStorage;
use super::WasmProofUpdateCtx;

use identity_iota::storage::bls::encode_bls_jwk;
use identity_iota::storage::bls::expand_bls_jwk;
use identity_iota::storage::bls::generate_bbs_keypair;
use identity_iota::storage::bls::sign_bbs;
use identity_iota::storage::bls::update_bbs_signature;
use identity_iota::storage::JwkGenOutput;
use identity_iota::storage::JwkStorage;
use identity_iota::storage::JwkStoragePQ;
use identity_iota::storage::KeyId;
use identity_iota::storage::KeyStorageError;
use identity_iota::storage::KeyStorageErrorKind;
use identity_iota::storage::KeyStorageResult;
use identity_iota::storage::KeyType;
use identity_iota::storage::ProofUpdateCtx;
use identity_iota::verification::jwk::Jwk;
use jsonprooftoken::jpa::algs::ProofAlgorithm;
use wasm_bindgen::prelude::*;
use identity_iota::verification::jose::jws::JwsAlgorithm;
use crate::error::JsValueResult;
use js_sys::Array;
use js_sys::Promise;
use js_sys::Uint8Array;
use wasm_bindgen_futures::JsFuture;

#[async_trait::async_trait(?Send)]
impl JwkStoragePQ for WasmJwkStorage {
  async fn generate_pq_key(&self, key_type: KeyType, alg: JwsAlgorithm) -> KeyStorageResult<JwkGenOutput> {
    web_sys::console::log_1(&"EEEEEEEEEEEEET".into());
    let promise: Promise = Promise::resolve(&WasmJwkStorage::generate_pq_key(self, key_type.into(), alg.name().to_owned()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }

  async fn pq_sign(&self, key_id: &KeyId, data: &[u8], public_key: &Jwk) -> KeyStorageResult<Vec<u8>> {
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
  generate_pq_key: (keyType: string, algorithm: JwsAlgorithm) => Promise<JwkGenOutput>;
}"#;