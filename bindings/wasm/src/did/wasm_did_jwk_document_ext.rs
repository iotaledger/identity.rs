// Copyright 2024 Fondazione Links
// SPDX-License-Identifier: Apache-2.0

use std::rc::Rc;
use crate::error::Result;
use crate::error::WasmResult;
use identity_iota::storage::DidJwkDocumentExt;
use identity_iota::document::CoreDocument;
use identity_iota::storage::key_storage::KeyType;
use identity_iota::verification::jws::JwsAlgorithm;
use identity_iota::verification::jwk::CompositeAlgId;
use crate::storage::WasmStorageInner;
use crate::jose::WasmJwsAlgorithm;
use crate::jose::WasmCompositeAlgId;
use crate::storage::WasmStorage;
use super::CoreDocumentLock;
use super::WasmCoreDocument;
use wasm_bindgen::prelude::*;

use crate::jpt::WasmProofAlgorithm;
use jsonprooftoken::jpa::algs::ProofAlgorithm;

#[wasm_bindgen(js_class = CoreDocument)]
impl WasmCoreDocument {

  #[wasm_bindgen(js_name = newDidJwk)]
  pub async fn _new_did_jwk(
    storage: &WasmStorage,
    key_type: String,
    alg: WasmJwsAlgorithm,
  ) -> Result<WasmCoreDocument>{
    let storage_clone: Rc<WasmStorageInner> = storage.0.clone();
    let alg: JwsAlgorithm = alg.into_serde().wasm_result()?;
    CoreDocument::new_did_jwk(
      &storage_clone,
      KeyType::from(key_type),
      alg
    ).await
    .map(|doc| WasmCoreDocument(Rc::new(CoreDocumentLock::new(doc.0))))
    .wasm_result()
  }

  #[wasm_bindgen(js_name = newDidJwkPq)]
  pub async fn _new_did_jwk_pqc(
    storage: &WasmStorage,
    key_type: String,
    alg: WasmJwsAlgorithm,
  ) -> Result<WasmCoreDocument> {
    let storage_clone: Rc<WasmStorageInner> = storage.0.clone();
    let alg: JwsAlgorithm = alg.into_serde().wasm_result()?;
    CoreDocument::new_did_jwk_pqc(
      &storage_clone,
      KeyType::from(key_type),
      alg
    ).await
    .map(|doc| WasmCoreDocument(Rc::new(CoreDocumentLock::new(doc.0))))
    .wasm_result()
  }

  #[wasm_bindgen(js_name = newDidCompositeJwk)]
  pub async fn _new_did_compositejwk(
    storage: &WasmStorage,
    alg: WasmCompositeAlgId
  ) -> Result<WasmCoreDocument>{
    let storage_clone: Rc<WasmStorageInner> = storage.0.clone();
    let alg: CompositeAlgId = alg.into_serde().wasm_result()?;
    CoreDocument::new_did_compositejwk(
      &storage_clone,
      alg
    ).await
    .map(|doc| WasmCoreDocument(Rc::new(CoreDocumentLock::new(doc.0))))
    .wasm_result()
  }

  #[wasm_bindgen(js_name = newDidJwkZk)]
  pub async fn _new_did_jwk_zk(
    storage: &WasmStorage,
    alg: WasmProofAlgorithm,
  ) -> Result<WasmCoreDocument> {
    let storage_clone: Rc<WasmStorageInner> = storage.0.clone();
    let alg: ProofAlgorithm = alg.into();
    CoreDocument::new_did_jwk_zk(
      &storage_clone,
      KeyType::from_static_str("BLS12381"),
      alg
    ).await
    .map(|doc| WasmCoreDocument(Rc::new(CoreDocumentLock::new(doc.0))))
    .wasm_result()
  }

  #[wasm_bindgen(js_name = fragmentJwk)]
  pub fn _fragment(self) -> String {
    "0".to_string()
  }

}
