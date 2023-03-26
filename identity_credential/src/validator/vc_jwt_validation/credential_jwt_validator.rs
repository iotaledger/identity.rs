// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use identity_core::common::Object;
use identity_core::common::Timestamp;
use identity_core::common::Url;
use identity_did::CoreDID;
use identity_did::DID;
use identity_document::document::CoreDocument;
use identity_document::verifiable::JwsVerificationOptions;
use identity_verification::jws::Decoder;
use identity_verification::jws::EdDSAJwsSignatureVerifier;
use identity_verification::jws::JwsSignatureVerifier;
use serde::de::DeserializeOwned;
use serde::Serialize; 

use super::CredentialToken;
use super::FailFast;
use super::SignerContext;
use super::SubjectHolderRelationship;
use super::ValidationError;
use super::CompoundCredentialValidationError;
use super::CredentialValidationOptions;
use crate::credential::Credential;

/// A struct for validating [`Credential`]s.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct CredentialValidator<V: JwsSignatureVerifier = EdDSAJwsSignatureVerifier>(V);

impl CredentialValidator {
  /// Creates a new [`CredentialValidator`] capable of verifying a [`Credential`] issued as a JWS
  /// using the [`EdDSA`](::identity_verification::jose::jws::JwsAlgorithm::EdDSA) algorithm. 
  /// 
  /// See [`CredentialValidator::with_signature_verifier`](CredentialValidator::with_signature_verifier()) 
  /// which enables you to supply a custom signature verifier if other JWS algorithms are of interest.
  pub fn new() -> Self {
    Self(EdDSAJwsSignatureVerifier::default())
  }
}


type ValidationUnitResult<T =()> = std::result::Result<T, ValidationError>;

impl<V> CredentialValidator<V> 
where 
  V: JwsSignatureVerifier
{
  /// Create a new [`CredentialValidator`] that delegates cryptographic signature verification to the given 
  /// `signature_verifier`. 
  pub fn with_signature_verifier(signature_verifier: V) -> Self {
    Self(signature_verifier)
  }

  /// Decodes and validates a [`Credential`] issued as a JWS. A [`CredentialToken`] is returned upon success. 
  ///
  /// The following properties are validated according to `options`:
  /// - the issuer's signature on the JWS,
  /// - the expiration date,
  /// - the issuance date,
  /// - the semantic structure.
  ///
  /// # Warning
  /// The lack of an error returned from this method is in of itself not enough to conclude that the credential can be
  /// trusted. This section contains more information on additional checks that should be carried out before and after
  /// calling this method.
  ///
  /// ## The state of the issuer's DID Document
  /// The caller must ensure that `issuer` represents an up-to-date DID Document.
  ///
  /// ## Properties that are not validated
  ///  There are many properties defined in [The Verifiable Credentials Data Model](https://www.w3.org/TR/vc-data-model/) that are **not** validated, such as:
  /// `credentialStatus`, `type`, `credentialSchema`, `refreshService`, `proof` **and more**.
  /// These should be manually checked after validation, according to your requirements.
  ///
  /// # Errors
  /// An error is returned whenever a validated condition is not satisfied.
  // TODO: Consider making this more generic so it returns CredentialToken<T>
  pub fn validate<DOC>(
    &self,
    credential_jws: &str,
    issuer: &DOC,
    options: &CredentialValidationOptions,
    fail_fast: FailFast,
  ) -> Result<CredentialToken, CompoundCredentialValidationError>
  where
    DOC: AsRef<CoreDocument>,
  {
    Self::validate_extended::<CoreDocument,Object,V>(
      &self.0,
      credential_jws,
      std::slice::from_ref(issuer.as_ref()),
      options,
      None,
      fail_fast,
    )
  }

  /// Validates the semantic structure of the [`Credential`].
  ///
  /// # Warning
  /// This does not validate against the credential's schema nor the structure of the subject claims.
  pub fn check_structure<T>(credential: &Credential<T>) -> ValidationUnitResult {
    credential
      .check_structure()
      .map_err(ValidationError::CredentialStructure)
  }

  /// Validate that the [`Credential`] expires on or after the specified [`Timestamp`].
  pub fn check_expires_on_or_after<T>(credential: &Credential<T>, timestamp: Timestamp) -> ValidationUnitResult {
    let expiration_date: Option<Timestamp> = credential.expiration_date;
    (expiration_date.is_none() || expiration_date >= Some(timestamp))
      .then_some(())
      .ok_or(ValidationError::ExpirationDate)
  }

  /// Validate that the [`Credential`] is issued on or before the specified [`Timestamp`].
  pub fn check_issued_on_or_before<T>(credential: &Credential<T>, timestamp: Timestamp) -> ValidationUnitResult {
    (credential.issuance_date <= timestamp)
      .then_some(())
      .ok_or(ValidationError::IssuanceDate)
  }

  /// Decode and verify the JWS signature of a [`Credential`] issued as a JWS using the DID Document of a trusted issuer.
  /// 
  /// A [`CredentialToken`] is returned upon success. 
  ///
  /// # Warning
  /// The caller must ensure that the DID Documents of the trusted issuers are up-to-date.
  ///
  /// # Errors
  /// This method immediately returns an error if
  /// the credential issuer' url cannot be parsed to a DID belonging to one of the trusted issuers. Otherwise an attempt
  /// to verify the credential's signature will be made and an error is returned upon failure.
  // TODO: Consider making this more generic so it returns CredentialToken<T>
  pub fn verify_signature<DOC>(
    &self,
    credential: &str,
    trusted_issuers: &[DOC],
    options: &JwsVerificationOptions,
  ) -> Result<CredentialToken, ValidationError>
  where 
    DOC: AsRef<CoreDocument>
  { 
    // Start decoding the signature 
    let decoder: Decoder<'_> = options.crits.as_deref().map(Decoder::new_with_crits).unwrap_or(Decoder::new());
    decoder.decode_compact_serialization(credential.as_bytes(), None).map_err();

    /*
     
    trusted_issuers
      .iter()
      .map(AsRef::as_ref)
      .find(|issuer_doc| <CoreDocument>::id(issuer_doc) == &issuer_did)
      .ok_or(ValidationError::DocumentMismatch(SignerContext::Issuer))
      .and_then(|issuer| {
        issuer
          .verify_data(credential, options)
          .map_err(|err| ValidationError::Signature {
            source: err.into(),
            signer_ctx: SignerContext::Issuer,
          })
      })
     */
    todo!()

    

  }

  /// Validate that the relationship between the `holder` and the credential subjects is in accordance with
  /// `relationship`.
  pub fn check_subject_holder_relationship<T>(
    credential: &Credential<T>,
    holder: &Url,
    relationship: SubjectHolderRelationship,
  ) -> ValidationUnitResult {
    let url_matches: bool = match &credential.credential_subject {
      OneOrMany::One(ref credential_subject) => credential_subject.id.as_ref() == Some(holder),
      OneOrMany::Many(subjects) => {
        // need to check the case where the Many variant holds a vector of exactly one subject
        if let [credential_subject] = subjects.as_slice() {
          credential_subject.id.as_ref() == Some(holder)
        } else {
          // zero or > 1 subjects is interpreted to mean that the holder is not the subject
          false
        }
      }
    };

    Some(relationship)
      .filter(|relationship| match relationship {
        SubjectHolderRelationship::AlwaysSubject => url_matches,
        SubjectHolderRelationship::SubjectOnNonTransferable => {
          url_matches || !credential.non_transferable.unwrap_or(false)
        }
        SubjectHolderRelationship::Any => true,
      })
      .map(|_| ())
      .ok_or(ValidationError::SubjectHolderRelationship)
  }

  /// Checks whether the credential status has been revoked.
  ///
  /// Only supports `BitmapRevocation2022`.
  #[cfg(feature = "revocation-bitmap")]
  pub fn check_status<DOC: AsRef<CoreDocument>, T>(
    credential: &Credential<T>,
    trusted_issuers: &[DOC],
    status_check: StatusCheck,
  ) -> ValidationUnitResult {
    use identity_did::CoreDID;
    use identity_document::document::CoreDocument;

    use crate::{revocation::RevocationBitmap, credential::RevocationBitmapStatus, validator::StatusCheck};

    if status_check == StatusCheck::SkipAll {
      return Ok(());
    }

    match &credential.credential_status {
      None => Ok(()),
      Some(status) => {
        // Check status is supported.
        if status.type_ != RevocationBitmap::TYPE {
          if status_check == StatusCheck::SkipUnsupported {
            return Ok(());
          }
          return Err(ValidationError::InvalidStatus(crate::Error::InvalidStatus(format!(
            "unsupported type '{}'",
            status.type_
          ))));
        }
        let status: RevocationBitmapStatus =
          RevocationBitmapStatus::try_from(status.clone()).map_err(ValidationError::InvalidStatus)?;

        // Check the credential index against the issuer's DID Document.
        let issuer_did: CoreDID = Self::extract_issuer(credential)?;
        trusted_issuers
          .iter()
          .find(|issuer| <CoreDocument>::id(issuer.as_ref()) == &issuer_did)
          .ok_or(ValidationError::DocumentMismatch(SignerContext::Issuer))
          .and_then(|issuer| CredentialValidator::check_revocation_bitmap_status(issuer, status))
      }
    }
  }

  /// Check the given `status` against the matching [`RevocationBitmap`] service in the
  /// issuer's DID Document.
  #[cfg(feature = "revocation-bitmap")]
  fn check_revocation_bitmap_status<DOC: AsRef<CoreDocument> + ?Sized>(
    issuer: &DOC,
    status: crate::credential::RevocationBitmapStatus,
  ) -> ValidationUnitResult {
    use crate::revocation::{RevocationDocumentExt, RevocationBitmap};

    let issuer_service_url: identity_did::DIDUrl = status.id().map_err(ValidationError::InvalidStatus)?;

    // Check whether index is revoked.
    let revocation_bitmap: RevocationBitmap = issuer
      .as_ref()
      .resolve_revocation_bitmap(issuer_service_url.into())
      .map_err(|_| ValidationError::ServiceLookupError)?;
    let index: u32 = status.index().map_err(ValidationError::InvalidStatus)?;
    if revocation_bitmap.is_revoked(index) {
      Err(ValidationError::Revoked)
    } else {
      Ok(())
    }
  }

  // This method takes a slice of issuer's instead of a single issuer in order to better accommodate presentation
  // validation. It also validates the relation ship between a holder and the credential subjects when
  // `relationship_criterion` is Some.
  pub(crate) fn validate_extended<DOC, T, S>(
    signature_verifier: &S, 
    credential: &str,
    issuers: &[DOC],
    options: &CredentialValidationOptions,
    relationship_criterion: Option<(&Url, SubjectHolderRelationship)>,
    fail_fast: FailFast,
  ) ->  Result<CredentialToken<T>, CompoundCredentialValidationError>
  where 
    S: JwsSignatureVerifier, 
    DOC: AsRef<CoreDocument>, 
    T: ToOwned + Serialize,
    <T as ToOwned>::Owned: DeserializeOwned
  {
    /* 
    // Run all single concern validations in turn and fail immediately if `fail_fast` is true.
    let signature_validation =
      std::iter::once_with(|| Self::verify_signature(credential, issuers, &options.verifier_options));

    let expiry_date_validation = std::iter::once_with(|| {
      Self::check_expires_on_or_after(credential, options.earliest_expiry_date.unwrap_or_default())
    });

    let issuance_date_validation = std::iter::once_with(|| {
      Self::check_issued_on_or_before(credential, options.latest_issuance_date.unwrap_or_default())
    });

    let structure_validation = std::iter::once_with(|| Self::check_structure(credential));

    let subject_holder_validation = std::iter::once_with(|| {
      relationship_criterion
        .map(|(holder, relationship)| Self::check_subject_holder_relationship(credential, holder, relationship))
        .unwrap_or(Ok(()))
    });

    let validation_units_iter = issuance_date_validation
      .chain(expiry_date_validation)
      .chain(structure_validation)
      .chain(subject_holder_validation)
      .chain(signature_validation);

    #[cfg(feature = "revocation-bitmap")]
    let validation_units_iter = {
      let revocation_validation = std::iter::once_with(|| Self::check_status(credential, issuers, options.status));
      validation_units_iter.chain(revocation_validation)
    };

    let validation_units_error_iter = validation_units_iter.filter_map(|result| result.err());
    let validation_errors: Vec<ValidationError> = match fail_fast {
      FailFast::FirstError => validation_units_error_iter.take(1).collect(),
      FailFast::AllErrors => validation_units_error_iter.collect(),
    };

    if validation_errors.is_empty() {
      Ok(())
    } else {
      Err(CompoundCredentialValidationError { validation_errors })
    }
    */
    todo!()
  }

  /// Utility for extracting the issuer field of a [`Credential`] as a DID.
  ///
  /// # Errors
  ///
  /// Fails if the issuer field is not a valid DID.
  pub fn extract_issuer<D, T>(credential: &Credential<T>) -> std::result::Result<D, ValidationError>
  where
     D: DID,
    <D as FromStr>::Err: std::error::Error + Send + Sync + 'static,
  {
    D::from_str(credential.issuer.url().as_str()).map_err(|err| ValidationError::SignerUrl {
      signer_ctx: SignerContext::Issuer,
      source: err.into(),
    })
  }
}