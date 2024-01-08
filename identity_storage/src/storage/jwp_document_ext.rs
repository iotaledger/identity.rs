//TODO:: JwpDocumentExt

use identity_core::common::Object;
use identity_credential::credential::Credential;
use identity_credential::credential::JwpCredentialOptions;
use identity_credential::presentation;
use identity_credential::presentation::JwpPresentationOptions;
use identity_credential::presentation::SelectiveDisclosurePresentation;
use identity_document::document::CoreDocument;
use identity_verification::MethodData;
use jsonprooftoken::encoding::SerializationType;
use jsonprooftoken::jpa::algs::PresentationProofAlgorithm;
use jsonprooftoken::jpt::claims;
use jsonprooftoken::jpt::claims::Claims;
use jsonprooftoken::jpt::claims::JptClaims;
use jsonprooftoken::jpt::payloads::Payloads;
use jsonprooftoken::jwk::key::Jwk;
use jsonprooftoken::jwp::header::IssuerProtectedHeader;
use jsonprooftoken::jwp::header::PresentationProtectedHeader;
use jsonprooftoken::jwp::issued::JwpIssued;
use jsonprooftoken::jwp::issued::JwpIssuedBuilder;
use jsonprooftoken::jwp::presented;
use jsonprooftoken::jwp::presented::JwpPresented;
use serde::Serialize;
use serde::de::DeserializeOwned;
use crate::Storage;
use jsonprooftoken::jpa::algs::ProofAlgorithm;
use crate::KeyType;
use identity_verification::MethodScope;
use crate::StorageResult;
use crate::JwkStorageExt;
use crate::KeyIdStorage;
use crate::JwkGenOutput;
use crate::key_id_storage::MethodDigest;
use super::JwkStorageDocumentError as Error;
use async_trait::async_trait;
use identity_did::DIDUrl;
use identity_verification::VerificationMethod;
use crate::try_undo_key_generation;
use identity_credential::credential::Jpt;



///New trait to handle JWP-based operations on DID Documents
#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
pub trait JwpDocumentExt {

  /// Generate new key material in the given `storage` and insert a new verification method with the corresponding
  /// public key material into the DID document. This support BBS+ keys.
  async fn generate_method_jwp<K, I>(
    &mut self,
    storage: &Storage<K, I>,
    key_type: KeyType,
    alg: ProofAlgorithm,
    fragment: Option<&str>,
    scope: MethodScope,
  ) -> StorageResult<String>
  where
    K: JwkStorageExt,
    I: KeyIdStorage;


  async fn create_issued_jwp<K, I>(
    &self,
    storage: &Storage<K, I>,
    fragment: &str,
    jpt_claims: &JptClaims,
    options: &JwpCredentialOptions,
  ) -> StorageResult<String>
  where
    K: JwkStorageExt,
    I: KeyIdStorage;


  async fn create_presented_jwp(
    &self,
    presentation: &mut SelectiveDisclosurePresentation,
    method_id: &str,
    options: &JwpPresentationOptions,
  ) -> StorageResult<String>;

  /// Produces a JPT where the payload is produced from the given `credential`.
  /// TODO: add references to Drafts
  async fn create_credential_jpt<K, I, T>(
    &self,
    credential: &Credential<T>,
    storage: &Storage<K, I>,
    fragment: &str,
    options: &JwpCredentialOptions,
    custom_claims: Option<Object>,
  ) -> StorageResult<Jpt>
  where
    K: JwkStorageExt,
    I: KeyIdStorage,
    T: ToOwned<Owned = T> + Serialize + DeserializeOwned + Sync;


  
  /// Produces a JPT in the pre where the payload is produced from the given `credential`.
  /// TODO: add references to Drafts
  async fn create_presentation_jpt(
    &self,
    presentation: &mut SelectiveDisclosurePresentation,
    method_id: &str,
    options: &JwpPresentationOptions,
  ) -> StorageResult<Jpt>;


 
}


// ====================================================================================================================
// CoreDocument
// ====================================================================================================================


generate_method_for_document_type!(CoreDocument, ProofAlgorithm, JwkStorageExt, JwkStorageExt::generate_bbs_key, generate_method_core_document);


#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
impl JwpDocumentExt for CoreDocument {
  async fn generate_method_jwp<K, I>(
    &mut self,
    storage: &Storage<K, I>,
    key_type: KeyType,
    alg: ProofAlgorithm,
    fragment: Option<&str>,
    scope: MethodScope,
  ) -> StorageResult<String>
  where
    K: JwkStorageExt,
    I: KeyIdStorage,
  {
    generate_method_core_document(self, storage, key_type, alg, fragment, scope).await
  }


  async fn create_issued_jwp<K, I>(
    &self,
    storage: &Storage<K, I>,
    fragment: &str,
    jpt_claims: &JptClaims,
    options: &JwpCredentialOptions,
  ) -> StorageResult<String>
  where
    K: JwkStorageExt,
    I: KeyIdStorage
  {
    // Obtain the method corresponding to the given fragment.
    let method: &VerificationMethod = self.resolve_method(fragment, None).ok_or(Error::MethodNotFound)?;
    let MethodData::PublicKeyJwk(ref jwk) = method.data() else {
      return Err(Error::NotPublicKeyJwk);
    };

    // Extract JwsAlgorithm.
    let alg: ProofAlgorithm = jwk
      .alg()
      .unwrap_or("")
      .parse()
      .map_err(|_| Error::InvalidJwpAlgorithm)?;


    // https://www.w3.org/TR/vc-data-model/#jwt-encoding
    let typ = "JPT".to_string();
      

    let kid = if let Some(ref kid) = options.kid {
      kid.clone()
    } else {
      method.id().to_string()
    };

    let mut issuer_header = IssuerProtectedHeader::new(alg);
    issuer_header.set_typ(Some(typ));
    issuer_header.set_kid(Some(kid));


    // Get the key identifier corresponding to the given method from the KeyId storage.
    let method_digest: MethodDigest = MethodDigest::new(method).map_err(Error::MethodDigestConstructionError)?;
    let key_id = <I as KeyIdStorage>::get_key_id(storage.key_id_storage(), &method_digest)
      .await
      .map_err(Error::KeyIdStorageError)?;


    let jwp = <K as JwkStorageExt>::generate_issuer_proof(storage.key_storage(), &key_id, issuer_header, jpt_claims.clone(), jwk)
      .await
      .map_err(Error::KeyStorageError)?;

    Ok(jwp)

  }


  async fn create_presented_jwp(
    &self,
    presentation: &mut SelectiveDisclosurePresentation,
    method_id: &str,
    options: &JwpPresentationOptions,
  ) -> StorageResult<String>
  {
    // Obtain the method corresponding to the given fragment.
    let method: &VerificationMethod = self.resolve_method(method_id, None).ok_or(Error::MethodNotFound)?;
    let MethodData::PublicKeyJwk(ref jwk) = method.data() else {
      return Err(Error::NotPublicKeyJwk);
    };

    // Extract JwsAlgorithm.
    let alg: ProofAlgorithm = jwk
      .alg()
      .unwrap_or("")
      .parse()
      .map_err(|_| Error::InvalidJwpAlgorithm)?;

    let public_key: Jwk = jwk.try_into().map_err(|_| Error::NotPublicKeyJwk)?;


    let mut presentation_header = PresentationProtectedHeader::new(alg.into());
    presentation_header.set_nonce(options.nonce.clone());
    presentation_header.set_aud(options.audience.as_ref().and_then(|u| Some(u.to_string())));

    presentation.set_presentation_header(presentation_header);

    let jwp_builder = presentation.builder();

    let presented_jwp = jwp_builder.build(&public_key).map_err(|_| Error::JwpBuildingError)?;

    Ok(presented_jwp.encode(SerializationType::COMPACT).map_err(|e| Error::EncodingError(Box::new(e)))?)
  

  }

  /// Produces a JPT where the payload is produced from the given `credential`.
  /// TODO: add references to Drafts
  async fn create_credential_jpt<K, I, T>(
    &self,
    credential: &Credential<T>,
    storage: &Storage<K, I>,
    fragment: &str,
    options: &JwpCredentialOptions,
    custom_claims: Option<Object>,
  ) -> StorageResult<Jpt>
  where
    K: JwkStorageExt,
    I: KeyIdStorage,
    T: ToOwned<Owned = T> + Serialize + DeserializeOwned + Sync
  {
    let jpt_claims = credential
      .serialize_jpt(custom_claims)
      .map_err(Error::ClaimsSerializationError)?;
    
    self
      .create_issued_jwp(storage, fragment, &jpt_claims, options)
      .await
      .map(|jwp| Jpt::new(jwp))
  }


  async fn create_presentation_jpt(
    &self,
    presentation: &mut SelectiveDisclosurePresentation,
    method_id: &str,
    options: &JwpPresentationOptions,
  ) -> StorageResult<Jpt> {
    
    self
      .create_presented_jwp(presentation, method_id, options)
      .await
      .map(|jwp| Jpt::new(jwp))

  }

  
}





// ====================================================================================================================
// IotaDocument
// ====================================================================================================================
#[cfg(feature = "iota-document")]
mod iota_document {
  use super::*;
  use identity_iota_core::IotaDocument;

  generate_method_for_document_type!(IotaDocument, ProofAlgorithm, JwkStorageExt, JwkStorageExt::generate_bbs_key, generate_method_iota_document);

  #[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
  #[cfg_attr(feature = "send-sync-storage", async_trait)]
  impl JwpDocumentExt for IotaDocument {
    async fn generate_method_jwp<K, I>(
      &mut self,
      storage: &Storage<K, I>,
      key_type: KeyType,
      alg: ProofAlgorithm,
      fragment: Option<&str>,
      scope: MethodScope,
    ) -> StorageResult<String>
    where
      K: JwkStorageExt,
      I: KeyIdStorage,
    {
      generate_method_iota_document(self, storage, key_type, alg, fragment, scope).await
    }


    async fn create_issued_jwp<K, I>(
      &self,
      storage: &Storage<K, I>,
      fragment: &str,
      jpt_claims: &JptClaims,
      options: &JwpCredentialOptions,
    ) -> StorageResult<String>
    where
      K: JwkStorageExt,
      I: KeyIdStorage
    {
      self
        .core_document()
        .create_issued_jwp(storage, fragment, jpt_claims, options)
        .await
    }
  
  
    async fn create_presented_jwp(
      &self,
      presentation: &mut SelectiveDisclosurePresentation,
      method_id: &str,
      options: &JwpPresentationOptions,
    ) -> StorageResult<String>
    {
      self
        .core_document()
        .create_presented_jwp(presentation, method_id, options)
        .await
    }
  
    /// Produces a JWP where the payload is produced from the given `credential`.
    /// TODO: add references to Drafts
    async fn create_credential_jpt<K, I, T>(
      &self,
      credential: &Credential<T>,
      storage: &Storage<K, I>,
      fragment: &str,
      options: &JwpCredentialOptions,
      custom_claims: Option<Object>,
    ) -> StorageResult<Jpt>
    where
      K: JwkStorageExt,
      I: KeyIdStorage,
      T: ToOwned<Owned = T> + Serialize + DeserializeOwned + Sync
    {
      self
        .core_document()
        .create_credential_jpt(credential, storage, fragment, options, custom_claims)
        .await
    }


    async fn create_presentation_jpt(
      &self,
      presentation: &mut SelectiveDisclosurePresentation,
      method_id: &str,
      options: &JwpPresentationOptions,
    ) -> StorageResult<Jpt> {
      self
        .core_document()
        .create_presentation_jpt(presentation, method_id, options)
        .await
    }
}
}