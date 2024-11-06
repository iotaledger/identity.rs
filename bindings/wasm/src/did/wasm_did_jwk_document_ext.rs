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

//use crate::jpt::WasmProofAlgorithm;

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



/*   pub enum CompositeAlgId {
    /// DER encoded value in hex = 060B6086480186FA6B50080103
    #[serde(rename = "id-MLDSA44-Ed25519-SHA512")]
    IdMldsa44Ed25519Sha512,
    /// DER encoded value in hex = 060B6086480186FA6B5008010A
    #[serde(rename = "id-MLDSA65-Ed25519-SHA512")]
    IdMldsa65Ed25519Sha512,
  }
  
  impl CompositeAlgId {
    /// Returns the JWS algorithm as a `str` slice.
    pub const fn name(self) -> &'static str {
      match self {
        Self::IdMldsa44Ed25519Sha512 => "id-MLDSA44-Ed25519-SHA512",
        Self::IdMldsa65Ed25519Sha512 => "id-MLDSA65-Ed25519-SHA512",
      }
    }
  } */



  #[wasm_bindgen(js_name = newDidCompositeJwk)]
  pub async fn _new_did_compositejwk(
    storage: &WasmStorage,
    //alg: identity_verification::jwk::CompositeAlgId,
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





  #[wasm_bindgen(js_name = fragmentJwk)]
  pub fn _fragment(self) -> String {
    "0".to_string()
  }

}

/* impl DidJwkDocumentExt for WasmCoreDocument{






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