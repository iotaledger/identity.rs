use std::rc::Rc;
use crate::error::Result;
use crate::error::WasmResult;
use identity_iota::storage::DidJwkDocumentExt;
use identity_iota::document::CoreDocument;
use identity_iota::storage::key_storage::KeyType;
use identity_iota::verification::jws::JwsAlgorithm;
use crate::storage::WasmStorageInner;
use crate::jose::WasmJwsAlgorithm;
use crate::storage::WasmStorage;
use super::CoreDocumentLock;
use super::WasmCoreDocument;
use wasm_bindgen::prelude::*;

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


/*   #[cfg(feature = "pqc")]
  async fn _new_did_jwk_pqc(
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

  } */

  #[wasm_bindgen(js_name = fragmentJwk)]
  pub fn _fragment(self) -> String {
    "0".to_string()
  }

}

/* impl DidJwkDocumentExt for WasmCoreDocument{




    #[cfg(feature = "jpt-bbs-plus")]
    async fn new_did_jwk_zk<K, I>(
        storage: &Storage<K, I>,
        key_type: KeyType,
        alg: ProofAlgorithm,
      ) -> StorageResult<(CoreDocument, String)>{
        todo!()
    }

/// a
    #[cfg(feature = "hybrid")]
    async fn new_did_compositejwk<K, I>(
        storage: &Storage<K, I>,
        alg: identity_verification::jwk::CompositeAlgId,
      ) -> StorageResult<(CoreDocument, String)> {
        todo!()
    }
/// a 




    
    pub async fn new_did_jwk(
      storage: &WasmStorage,
      key_type: String,
      alg: WasmJwsAlgorithm,
    //) -> Result<(WasmCoreDocument, String)>{
    ) -> Result<WasmCoreDocument>{


    }
}
 */