use identity_did::DIDJwk;
use identity_document::document::{self, CoreDocument};
use identity_verification::{jws::JwsAlgorithm, jwu::encode_b64_json, verification_method};
use jsonprooftoken::jpa::algs::ProofAlgorithm;
use async_trait::async_trait;

use crate::{JwkGenOutput, JwkStorage, JwkStorageBbsPlusExt, JwkStorageDocumentError as Error, JwkStoragePQ, KeyIdStorage, KeyType, MethodDigest};

use super::{try_undo_key_generation, Storage, StorageResult};



/// a 
#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
pub trait DidJwkDocumentExt{
/// a 
    async fn new_did_jwk<K, I>(
        storage: &Storage<K, I>,
        key_type: KeyType,
        alg: JwsAlgorithm,
      ) -> StorageResult<(CoreDocument, String)>
      where
        K: JwkStorage,
        I: KeyIdStorage;
/// a 
    #[cfg(feature = "pqc")]
    async fn new_did_jwk_pqc<K, I>(
        storage: &Storage<K, I>,
        key_type: KeyType,
        alg: JwsAlgorithm,
      ) -> StorageResult<(CoreDocument, String)>
      where
        K: JwkStoragePQ,
        I: KeyIdStorage;
/// a 
    #[cfg(feature = "jpt-bbs-plus")]
    async fn new_did_jwk_zk<K, I>(
        storage: &Storage<K, I>,
        key_type: KeyType,
        alg: ProofAlgorithm,
      ) -> StorageResult<(CoreDocument, String)>
      where
        K: JwkStorageBbsPlusExt,
        I: KeyIdStorage;
}


#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
impl DidJwkDocumentExt for CoreDocument {

    async fn new_did_jwk<K, I>(
        storage: &Storage<K, I>,
        key_type: KeyType,
        alg: JwsAlgorithm,
      ) -> StorageResult<(CoreDocument, String)>
      where
        K: JwkStorage,
        I: KeyIdStorage {
        
            let JwkGenOutput { key_id, jwk } = K::generate(storage.key_storage(),
            key_type,
                alg
            ).await
            .map_err(Error::KeyStorageError)?;
        
            let b64 = encode_b64_json(&jwk)
            .map_err(|err| Error::EncodingError(Box::new(err)))?;
        
            let did = DIDJwk::parse(&("did:jwk:".to_string() + &b64))
            .map_err(|err| Error::EncodingError(Box::new(err)))?;
    
        
            let document = CoreDocument::expand_did_jwk(did)
            .map_err(|err| Error::EncodingError(Box::new(err)))?;

        
            let fragment = "0";
        
            let verification_method = document.resolve_method(fragment, None)
            .ok_or(identity_verification::Error::MissingIdFragment)
            .map_err(Error::VerificationMethodConstructionError)?;
        
            let method_digest = MethodDigest::new(verification_method)
            .map_err(|err| Error::MethodDigestConstructionError(err))?;
        
            I::insert_key_id(&storage.key_id_storage(), method_digest, key_id.clone())
            .await
            .map_err(Error::KeyIdStorageError)?;

            Ok((document, fragment.to_string()))
    }

    async fn new_did_jwk_pqc<K, I>(
        storage: &Storage<K, I>,
        key_type: KeyType,
        alg: JwsAlgorithm,
      ) -> StorageResult<(CoreDocument, String)>
      where
        K: JwkStoragePQ,
        I: KeyIdStorage {

            let JwkGenOutput { key_id, jwk } = K::generate_pq_key(storage.key_storage(),
            key_type,
                alg
            ).await
            .map_err(Error::KeyStorageError)?;
        
            let b64 = encode_b64_json(&jwk)
            .map_err(|err| Error::EncodingError(Box::new(err)))?;
        
            let did = DIDJwk::parse(&("did:jwk:".to_string() + &b64))
            .map_err(|err| Error::EncodingError(Box::new(err)))?;
    
        
            let document = CoreDocument::expand_did_jwk(did)
            .map_err(|err| Error::EncodingError(Box::new(err)))?;

        
            let fragment = "0";
        
            let verification_method = document.resolve_method(fragment, None)
            .ok_or(identity_verification::Error::MissingIdFragment)
            .map_err(Error::VerificationMethodConstructionError)?;
        
            let method_digest = MethodDigest::new(verification_method)
            .map_err(|err| Error::MethodDigestConstructionError(err))?;
        
            I::insert_key_id(&storage.key_id_storage(), method_digest, key_id.clone())
            .await
            .map_err(Error::KeyIdStorageError)?;

            Ok((document, fragment.to_string()))
    }

    async fn new_did_jwk_zk<K, I>(
        storage: &Storage<K, I>,
        key_type: KeyType,
        alg: ProofAlgorithm,
      ) -> StorageResult<(CoreDocument, String)>
      where
        K: JwkStorageBbsPlusExt,
        I: KeyIdStorage {
            let JwkGenOutput { key_id, jwk } = K::generate_bbs(storage.key_storage(),
            key_type,
                alg
            ).await
            .map_err(Error::KeyStorageError)?;
        
            let b64 = encode_b64_json(&jwk)
            .map_err(|err| Error::EncodingError(Box::new(err)))?;
        
            let did = DIDJwk::parse(&("did:jwk:".to_string() + &b64))
            .map_err(|err| Error::EncodingError(Box::new(err)))?;
    
        
            let document = CoreDocument::expand_did_jwk(did)
            .map_err(|err| Error::EncodingError(Box::new(err)))?;

        
            let fragment = "0";
        
            let verification_method = document.resolve_method(fragment, None)
            .ok_or(identity_verification::Error::MissingIdFragment)
            .map_err(Error::VerificationMethodConstructionError)?;
        
            let method_digest = MethodDigest::new(verification_method)
            .map_err(|err| Error::MethodDigestConstructionError(err))?;
        
            I::insert_key_id(&storage.key_id_storage(), method_digest, key_id.clone())
            .await
            .map_err(Error::KeyIdStorageError)?;

            Ok((document, fragment.to_string()))
    }
}