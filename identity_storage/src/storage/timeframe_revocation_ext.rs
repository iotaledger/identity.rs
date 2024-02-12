use crate::{JwkStorageExt, KeyIdStorage, MethodDigest, Storage, StorageResult};
use super::JwkStorageDocumentError as Error;
use async_trait::async_trait;
use identity_core::common::{Duration, Timestamp};
use identity_credential::{credential::Jpt, revocation::{RevocationError, RevocationTimeframeStatus}};
use identity_document::document::CoreDocument;
use identity_verification::{MethodData, VerificationMethod};
use jsonprooftoken::{encoding::SerializationType, jpa::algs::ProofAlgorithm, jpt::{claims::JptClaims, payloads::{self, Payloads}}, jwp::issued::JwpIssued};
use serde_json::Value;
use zkryptium::bbsplus::signature::BBSplusSignature;


//TODO: TimeframeRevocationExtension

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
pub trait TimeframeRevocationExtension {
    
    async fn update<K, I>(
        &self,
        storage: &Storage<K, I>,
        fragment: &str,
        start_validity: Option<Timestamp>,
        duration: Duration, 
        credential_jwp: &mut JwpIssued, 
    ) -> StorageResult<Jpt>
    where
        K: JwkStorageExt,
        I: KeyIdStorage;
}


// ====================================================================================================================
// CoreDocument
// ====================================================================================================================

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
impl TimeframeRevocationExtension for CoreDocument {

    async fn update<K, I>(
        &self,
        storage: &Storage<K, I>,
        fragment: &str,
        start_validity: Option<Timestamp>,
        duration: Duration, 
        credential_jwp: &mut JwpIssued, 
    ) -> StorageResult<Jpt>
    where
        K: JwkStorageExt,
        I: KeyIdStorage
    {

        // Obtain the method corresponding to the given fragment.
        let method: &VerificationMethod = self.resolve_method(fragment, None).ok_or(Error::MethodNotFound)?;
        let MethodData::PublicKeyJwk(ref jwk) = method.data() else {
        return Err(Error::NotPublicKeyJwk);
        };


        // Get the key identifier corresponding to the given method from the KeyId storage.
        let method_digest: MethodDigest = MethodDigest::new(method).map_err(Error::MethodDigestConstructionError)?;
        let key_id = <I as KeyIdStorage>::get_key_id(storage.key_id_storage(), &method_digest)
        .await
        .map_err(Error::KeyIdStorageError)?;



        let new_start_validity_timeframe = start_validity.unwrap_or(Timestamp::now_utc());
        let new_end_validity_timeframe = new_start_validity_timeframe.checked_add(duration).ok_or(Error::ProofUpdateError("Invalid granularity".to_owned()))?;
        let new_start_validity_timeframe = new_start_validity_timeframe.to_rfc3339();
        let new_end_validity_timeframe = new_end_validity_timeframe.to_rfc3339();
        
        let proof = credential_jwp.get_proof();
        let claims = credential_jwp.get_claims().ok_or(Error::ProofUpdateError("Should not happen".to_owned()))?;
        let mut payloads: Payloads = credential_jwp.get_payloads().clone();

        let index_start_validity_timeframe = claims.get_claim_index(format!("vc.credentialStatus.{}", RevocationTimeframeStatus::START_TIMEFRAME_PROPERTY)).ok_or(Error::ProofUpdateError("'startValidityTimeframe' property NOT found".to_owned()))?;
        let index_end_validity_timeframe = claims.get_claim_index(format!("vc.credentialStatus.{}", RevocationTimeframeStatus::END_TIMEFRAME_PROPERTY)).ok_or(Error::ProofUpdateError("'endValidityTimeframe' property NOT found".to_owned()))?;


        let old_start_validity_timeframe = payloads.replace_payload_at_index(index_start_validity_timeframe, Value::String(new_start_validity_timeframe.clone()))
        .map_err(|_| Error::ProofUpdateError("'startValidityTimeframe' value NOT found".to_owned()))?
        .to_string();
        let old_end_validity_timeframe = payloads.replace_payload_at_index(index_end_validity_timeframe, Value::String(new_end_validity_timeframe.clone()))
        .map_err(|_| Error::ProofUpdateError("'endValidityTimeframe' value NOT found".to_owned()))?
        .to_string();
        

        let proof: [u8; 112] = proof.try_into().map_err(|_| Error::ProofUpdateError("Invalid bytes length of JWP proof".to_owned()))?;

        let new_proof =  <K as JwkStorageExt>::update_proof(
            storage.key_storage(), 
            &key_id, 
            jwk, 
            &proof, 
            old_start_validity_timeframe, 
            new_start_validity_timeframe, 
            old_end_validity_timeframe, 
            new_end_validity_timeframe, 
            index_start_validity_timeframe, 
            index_end_validity_timeframe, 
            payloads.0.len()
        )
        .await
        .map_err(Error::KeyStorageError)?;

        credential_jwp.set_proof(&new_proof);
        credential_jwp.set_payloads(payloads);
        let jpt = credential_jwp.encode(SerializationType::COMPACT)
        .map_err(|e| Error::EncodingError(Box::new(e)))?;

        Ok(Jpt::new(jpt))
    }
}



// ====================================================================================================================
// IotaDocument
// ====================================================================================================================
#[cfg(feature = "iota-document")]
mod iota_document {
  use super::*;
  use identity_iota_core::IotaDocument;

  #[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
  #[cfg_attr(feature = "send-sync-storage", async_trait)]
  impl TimeframeRevocationExtension for IotaDocument {

        async fn update<K, I>(
            &self,
            storage: &Storage<K, I>,
            fragment: &str,
            start_validity: Option<Timestamp>,
            duration: Duration, 
            credential_jwp: &mut JwpIssued, 
        ) -> StorageResult<Jpt>
        where
            K: JwkStorageExt,
            I: KeyIdStorage
        {
            self
            .core_document()
            .update(storage, fragment, start_validity, duration, credential_jwp)
            .await
        }

    }

}   