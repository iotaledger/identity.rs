use std::str::FromStr;
use std::rc::Rc;
use crate::error::Result;
use crate::error::WasmResult;
use crate::jose::WasmJwk;
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
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen(js_class = CoreDocument)]
impl WasmCoreDocument {
    
    #[wasm_bindgen(js_name = newDidJwk)]
    pub async fn _new_did_jwk(
      storage: &WasmStorage,
      key_type: String,
      alg: WasmJwsAlgorithm,
    //) -> Result<(WasmCoreDocument, String)>{
    ) -> Result<WasmCoreDocument>{
        let storage_clone: Rc<WasmStorageInner> = storage.0.clone();
        let alg: JwsAlgorithm = alg.into_serde().wasm_result()?;
        
        CoreDocument::new_did_jwk(
            &storage_clone,
            KeyType::from(key_type),
            alg)
            .await
            .map(|doc| WasmCoreDocument(Rc::new(CoreDocumentLock::new(doc.0))))
            .wasm_result()
    }






}



/* impl DidJwkDocumentExt for WasmCoreDocument{


    async fn new_did_jwk<K, I>(
        storage: &Storage<K, I>,
        key_type: KeyType,
        alg: JwsAlgorithm,
      ) -> StorageResult<(CoreDocument, String)>
    where
        K: JwkStorage,
        I: KeyIdStorage {
            todo!()
        }
/// a 
    #[cfg(feature = "pqc")]
    async fn new_did_jwk_pqc<K, I>(
        storage: &Storage<K, I>,
        key_type: KeyType,
        alg: JwsAlgorithm,
      ) -> StorageResult<(CoreDocument, String)> {
        todo!()
    }

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