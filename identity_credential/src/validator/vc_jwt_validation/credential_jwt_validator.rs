// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use identity_core::common::Object;
use identity_core::common::OneOrMany;
use identity_core::common::Timestamp;
use identity_core::common::Url;
use identity_core::convert::FromJson;
use identity_did::CoreDID;
use identity_did::DIDUrl;
use identity_did::DID;
use identity_document::document::CoreDocument;
use identity_document::verifiable::JwsVerificationOptions;
use identity_verification::jwk::Jwk;
use identity_verification::jws::DecodedJws;
use identity_verification::jws::Decoder;
use identity_verification::jws::EdDSAJwsVerifier;
use identity_verification::jws::JwsValidationItem;
use identity_verification::jws::JwsVerifier;

use super::CompoundCredentialValidationError;
use super::CredentialValidationOptions;
use super::DecodedJwtCredential;
use super::SignerContext;
use super::ValidationError;
use crate::credential::Credential;
use crate::credential::CredentialJwtClaims;
use crate::credential::Jwt;
use crate::validator::FailFast;
use crate::validator::SubjectHolderRelationship;

/// A type for decoding and validating [`Credential`]s.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct CredentialValidator<V: JwsVerifier = EdDSAJwsVerifier>(V);

type ValidationUnitResult<T = ()> = std::result::Result<T, ValidationError>;

impl<V> CredentialValidator<V>
where
  V: JwsVerifier,
{
  /// Create a new [`CredentialValidator`] that delegates cryptographic signature verification to the given
  /// `signature_verifier`. If you are only interested in `EdDSA` signatures (with `Ed25519`) then the default
  /// constructor can be used. See [`CredentialValidator::new`](CredentialValidator::new).
  pub fn with_signature_verifier(signature_verifier: V) -> Self {
    Self(signature_verifier)
  }

  /// Decodes and validates a [`Credential`] issued as a JWT. A [`DecodedJwtCredential`] is returned upon success.
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
  /// `proof`, `credentialStatus`, `type`, `credentialSchema`, `refreshService` **and more**.
  /// These should be manually checked after validation, according to your requirements.
  ///
  /// # Errors
  /// An error is returned whenever a validated condition is not satisfied.
  pub fn validate<DOC, T>(
    &self,
    credential_jwt: &Jwt,
    issuer: &DOC,
    options: &CredentialValidationOptions,
    fail_fast: FailFast,
  ) -> Result<DecodedJwtCredential<T>, CompoundCredentialValidationError>
  where
    T: ToOwned<Owned = T> + serde::Serialize + serde::de::DeserializeOwned,
    DOC: AsRef<CoreDocument>,
  {
    Self::validate_extended::<CoreDocument, V, T>(
      &self.0,
      credential_jwt,
      std::slice::from_ref(issuer.as_ref()),
      options,
      fail_fast,
    )
  }

  /// Decode and verify the JWS signature of a [`Credential`] issued as a JWT using the DID Document of a trusted
  /// issuer.
  ///
  /// A [`DecodedJwtCredential`] is returned upon success.
  ///
  /// # Warning
  /// The caller must ensure that the DID Documents of the trusted issuers are up-to-date.
  ///
  /// ## Proofs
  ///  Only the JWS signature is verified. If the [`Credential`] contains a `proof` property this will not be verified
  /// by this method.
  ///
  /// # Errors
  /// This method immediately returns an error if
  /// the credential issuer' url cannot be parsed to a DID belonging to one of the trusted issuers. Otherwise an attempt
  /// to verify the credential's signature will be made and an error is returned upon failure.
  pub fn verify_signature<DOC, T>(
    &self,
    credential: &Jwt,
    trusted_issuers: &[DOC],
    options: &JwsVerificationOptions,
  ) -> Result<DecodedJwtCredential<T>, ValidationError>
  where
    T: ToOwned<Owned = T> + serde::Serialize + serde::de::DeserializeOwned,
    DOC: AsRef<CoreDocument>,
  {
    Self::verify_signature_with_verifier(&self.0, credential, trusted_issuers, options)
  }

  // This method takes a slice of issuer's instead of a single issuer in order to better accommodate presentation
  // validation. It also validates the relationship between a holder and the credential subjects when
  // `relationship_criterion` is Some.
  pub(crate) fn validate_extended<DOC, S, T>(
    signature_verifier: &S,
    credential: &Jwt,
    issuers: &[DOC],
    options: &CredentialValidationOptions,
    fail_fast: FailFast,
  ) -> Result<DecodedJwtCredential<T>, CompoundCredentialValidationError>
  where
    T: ToOwned<Owned = T> + serde::Serialize + serde::de::DeserializeOwned,
    S: JwsVerifier,
    DOC: AsRef<CoreDocument>,
  {
    // First verify the JWS signature and decode the result into a credential token, then apply all other validations.
    // If this errors we have to return early regardless of the `fail_fast` flag as all other validations require a
    // `&Credential`.
    let credential_token =
      Self::verify_signature_with_verifier(signature_verifier, credential, issuers, &options.verification_options)
        .map_err(|err| CompoundCredentialValidationError {
          validation_errors: [err].into(),
        })?;

    let credential: &Credential<T> = &credential_token.credential;
    // Run all single concern Credential validations in turn and fail immediately if `fail_fast` is true.

    let expiry_date_validation = std::iter::once_with(|| {
      CredentialValidator::check_expires_on_or_after(
        &credential_token.credential,
        options.earliest_expiry_date.unwrap_or_default(),
      )
    });

    let issuance_date_validation = std::iter::once_with(|| {
      CredentialValidator::check_issued_on_or_before(credential, options.latest_issuance_date.unwrap_or_default())
    });

    let structure_validation = std::iter::once_with(|| CredentialValidator::check_structure(credential));

    let subject_holder_validation = std::iter::once_with(|| {
      options
        .subject_holder_relationship
        .as_ref()
        .map(|(holder, relationship)| {
          CredentialValidator::check_subject_holder_relationship(credential, holder, *relationship)
        })
        .unwrap_or(Ok(()))
    });

    let validation_units_iter = issuance_date_validation
      .chain(expiry_date_validation)
      .chain(structure_validation)
      .chain(subject_holder_validation);

    #[cfg(feature = "revocation-bitmap")]
    let validation_units_iter = {
      let revocation_validation =
        std::iter::once_with(|| CredentialValidator::check_status(credential, issuers, options.status));
      validation_units_iter.chain(revocation_validation)
    };

    let validation_units_error_iter = validation_units_iter.filter_map(|result| result.err());
    let validation_errors: Vec<ValidationError> = match fail_fast {
      FailFast::FirstError => validation_units_error_iter.take(1).collect(),
      FailFast::AllErrors => validation_units_error_iter.collect(),
    };

    if validation_errors.is_empty() {
      Ok(credential_token)
    } else {
      Err(CompoundCredentialValidationError { validation_errors })
    }
  }

  /// Stateless version of [`Self::verify_signature`]
  fn verify_signature_with_verifier<DOC, S, T>(
    signature_verifier: &S,
    credential: &Jwt,
    trusted_issuers: &[DOC],
    options: &JwsVerificationOptions,
  ) -> Result<DecodedJwtCredential<T>, ValidationError>
  where
    T: ToOwned<Owned = T> + serde::Serialize + serde::de::DeserializeOwned,
    DOC: AsRef<CoreDocument>,
    S: JwsVerifier,
  {
    // Note the below steps are necessary because `CoreDocument::verify_jws` decodes the JWS and then searches for a
    // method with a fragment (or full DID Url) matching `kid` in the given document. We do not want to carry out
    // that process for potentially every document in `trusted_issuers`.

    // Start decoding the credential
    let decoded: JwsValidationItem<'_> = Self::decode(credential.as_str())?;

    let nonce: Option<&str> = options.nonce.as_deref();
    // Validate the nonce
    if decoded.nonce() != nonce {
      return Err(ValidationError::JwsDecodingError(
        identity_verification::jose::error::Error::InvalidParam("invalid nonce value"),
      ));
    }

    // Parse the `kid` to a DID Url which should be the identifier of a verification method in a trusted issuer's DID
    // document.
    let method_id: DIDUrl = {
      let kid: &str =
        decoded
          .protected_header()
          .and_then(|header| header.kid())
          .ok_or(ValidationError::MethodDataLookupError {
            source: None,
            message: "could not extract kid from protected header",
            signer_ctx: SignerContext::Issuer,
          })?;

      // Convert kid to DIDUrl
      DIDUrl::parse(kid).map_err(|err| ValidationError::MethodDataLookupError {
        source: Some(err.into()),
        message: "could not parse kid as a DID Url",
        signer_ctx: SignerContext::Issuer,
      })
    }?;

    // locate the corresponding issuer
    let issuer: &CoreDocument = trusted_issuers
      .iter()
      .map(AsRef::as_ref)
      .find(|issuer_doc| <CoreDocument>::id(issuer_doc) == method_id.did())
      .ok_or(ValidationError::DocumentMismatch(SignerContext::Issuer))?;

    // Obtain the public key from the issuer's DID document
    let public_key: &Jwk = issuer
      .resolve_method(&method_id, options.method_scope)
      .and_then(|method| method.data().public_key_jwk())
      .ok_or_else(|| ValidationError::MethodDataLookupError {
        source: None,
        message: "could not extract JWK from a method identified by kid",
        signer_ctx: SignerContext::Issuer,
      })?;

    let credential_token = Self::verify_decoded_signature(decoded, public_key, signature_verifier)?;

    // Check that the DID component of the parsed `kid` does indeed correspond to the issuer in the credential before
    // returning.
    let issuer_id: CoreDID = CredentialValidator::extract_issuer(&credential_token.credential)?;
    if &issuer_id != method_id.did() {
      return Err(ValidationError::IdentifierMismatch {
        signer_ctx: SignerContext::Issuer,
      });
    };
    Ok(credential_token)
  }

  /// Decode the credential into a [`JwsValidationItem`].
  fn decode(credential_jws: &str) -> Result<JwsValidationItem<'_>, ValidationError> {
    let decoder: Decoder = Decoder::new();

    decoder
      .decode_compact_serialization(credential_jws.as_bytes(), None)
      .map_err(ValidationError::JwsDecodingError)
  }

  /// Verify the signature using the given `public_key` and `signature_verifier`.
  fn verify_decoded_signature<S: JwsVerifier, T>(
    decoded: JwsValidationItem<'_>,
    public_key: &Jwk,
    signature_verifier: &S,
  ) -> Result<DecodedJwtCredential<T>, ValidationError>
  where
    T: ToOwned<Owned = T> + serde::Serialize + serde::de::DeserializeOwned,
  {
    // Verify the JWS signature and obtain the decoded token containing the protected header and raw claims
    let DecodedJws { protected, claims, .. } =
      decoded
        .verify(signature_verifier, public_key)
        .map_err(|err| ValidationError::Signature {
          source: err,
          signer_ctx: SignerContext::Issuer,
        })?;

    // Deserialize the raw claims
    let credential_claims: CredentialJwtClaims<'_, T> =
      CredentialJwtClaims::from_json_slice(&claims).map_err(|err| {
        ValidationError::CredentialStructure(crate::Error::JwtClaimsSetDeserializationError(err.into()))
      })?;

    // Construct the credential token containing the credential and the protected header.
    let credential: Credential<T> = credential_claims
      .try_into_credential()
      .map_err(ValidationError::CredentialStructure)?;

    Ok(DecodedJwtCredential {
      credential,
      header: Box::new(protected),
    })
  }
}

impl CredentialValidator {
  /// Creates a new [`CredentialValidator`] capable of verifying a [`Credential`] issued as a JWS
  /// using the [`EdDSA`](::identity_verification::jose::jws::JwsAlgorithm::EdDSA) algorithm.
  ///
  /// See [`CredentialValidator::with_signature_verifier`](CredentialValidator::with_signature_verifier())
  /// which enables you to supply a custom signature verifier if other JWS algorithms are of interest.
  pub fn new() -> Self {
    Self(EdDSAJwsVerifier::default())
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
  /// Only supports `RevocationBitmap2022`.
  #[cfg(feature = "revocation-bitmap")]
  pub fn check_status<DOC: AsRef<CoreDocument>, T>(
    credential: &Credential<T>,
    trusted_issuers: &[DOC],
    status_check: crate::validator::StatusCheck,
  ) -> ValidationUnitResult {
    if status_check == crate::validator::StatusCheck::SkipAll {
      return Ok(());
    }

    match &credential.credential_status {
      None => Ok(()),
      Some(status) => {
        // Check status is supported.
        if status.type_ != crate::revocation::RevocationBitmap::TYPE {
          if status_check == crate::validator::StatusCheck::SkipUnsupported {
            return Ok(());
          }
          return Err(ValidationError::InvalidStatus(crate::Error::InvalidStatus(format!(
            "unsupported type '{}'",
            status.type_
          ))));
        }
        let status: crate::credential::RevocationBitmapStatus =
          crate::credential::RevocationBitmapStatus::try_from(status.clone())
            .map_err(ValidationError::InvalidStatus)?;

        // Check the credential index against the issuer's DID Document.
        let issuer_did: CoreDID = Self::extract_issuer(credential)?;
        trusted_issuers
          .iter()
          .find(|issuer| <CoreDocument>::id(issuer.as_ref()) == &issuer_did)
          .ok_or(ValidationError::DocumentMismatch(SignerContext::Issuer))
          .and_then(|issuer| Self::check_revocation_bitmap_status(issuer, status))
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
    use crate::revocation::RevocationDocumentExt;

    let issuer_service_url: identity_did::DIDUrl = status.id().map_err(ValidationError::InvalidStatus)?;

    // Check whether index is revoked.
    let revocation_bitmap: crate::revocation::RevocationBitmap = issuer
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

  /// Utility for extracting the issuer field of a credential in JWT representation as DID.
  ///
  /// # Errors
  ///
  /// If the JWT decoding fails or the issuer field is not a valid DID.
  pub fn extract_issuer_from_jwt<D>(credential: &Jwt) -> std::result::Result<D, ValidationError>
  where
    D: DID,
    <D as FromStr>::Err: std::error::Error + Send + Sync + 'static,
  {
    let validation_item = Decoder::new()
      .decode_compact_serialization(credential.as_str().as_bytes(), None)
      .map_err(ValidationError::JwsDecodingError)?;

    let claims: CredentialJwtClaims<'_, Object> = CredentialJwtClaims::from_json_slice(&validation_item.claims())
      .map_err(|err| {
        ValidationError::CredentialStructure(crate::Error::JwtClaimsSetDeserializationError(err.into()))
      })?;

    D::from_str(claims.iss.url().as_str()).map_err(|err| ValidationError::SignerUrl {
      signer_ctx: SignerContext::Issuer,
      source: err.into(),
    })
  }
}

impl Default for CredentialValidator {
  fn default() -> Self {
    Self::new()
  }
}

#[cfg(test)]
mod tests {
  use crate::credential::Subject;
  use identity_core::common::Duration;

  // All tests here are essentially adaptations of the old CredentialValidator tests.
  use super::*;
  use identity_core::common::Object;
  use identity_core::common::Timestamp;
  use proptest::proptest;
  const LAST_RFC3339_COMPATIBLE_UNIX_TIMESTAMP: i64 = 253402300799; // 9999-12-31T23:59:59Z
  const FIRST_RFC3999_COMPATIBLE_UNIX_TIMESTAMP: i64 = -62167219200; // 0000-01-01T00:00:00Z

  const SIMPLE_CREDENTIAL_JSON: &str = r#"{
    "@context": [
      "https://www.w3.org/2018/credentials/v1",
      "https://www.w3.org/2018/credentials/examples/v1"
    ],
    "id": "http://example.edu/credentials/3732",
    "type": ["VerifiableCredential", "UniversityDegreeCredential"],
    "issuer": "https://example.edu/issuers/14",
    "issuanceDate": "2010-01-01T19:23:24Z",
    "expirationDate": "2020-01-01T19:23:24Z",
    "credentialSubject": {
      "id": "did:example:ebfeb1f712ebc6f1c276e12ec21",
      "degree": {
        "type": "BachelorDegree",
        "name": "Bachelor of Science in Mechanical Engineering"
      }
    }
  }"#;

  lazy_static::lazy_static! {
    // A simple credential shared by some of the tests in this module
    static ref SIMPLE_CREDENTIAL: Credential = Credential::<Object>::from_json(SIMPLE_CREDENTIAL_JSON).unwrap();
  }

  #[test]
  fn issued_on_or_before() {
    assert!(CredentialValidator::check_issued_on_or_before(
      &SIMPLE_CREDENTIAL,
      SIMPLE_CREDENTIAL
        .issuance_date
        .checked_sub(Duration::minutes(1))
        .unwrap()
    )
    .is_err());

    // and now with a later timestamp
    assert!(CredentialValidator::check_issued_on_or_before(
      &SIMPLE_CREDENTIAL,
      SIMPLE_CREDENTIAL
        .issuance_date
        .checked_add(Duration::minutes(1))
        .unwrap()
    )
    .is_ok());
  }

  #[test]
  fn check_subject_holder_relationship() {
    let mut credential: Credential = SIMPLE_CREDENTIAL.clone();

    // first ensure that holder_url is the subject and set the nonTransferable property
    let actual_holder_url = credential.credential_subject.first().unwrap().id.clone().unwrap();
    assert_eq!(credential.credential_subject.len(), 1);
    credential.non_transferable = Some(true);

    // checking with holder = subject passes for all defined subject holder relationships:
    assert!(CredentialValidator::check_subject_holder_relationship(
      &credential,
      &actual_holder_url,
      SubjectHolderRelationship::AlwaysSubject
    )
    .is_ok());

    assert!(CredentialValidator::check_subject_holder_relationship(
      &credential,
      &actual_holder_url,
      SubjectHolderRelationship::SubjectOnNonTransferable
    )
    .is_ok());

    assert!(CredentialValidator::check_subject_holder_relationship(
      &credential,
      &actual_holder_url,
      SubjectHolderRelationship::Any
    )
    .is_ok());

    // check with a holder different from the subject of the credential:
    let issuer_url = Url::parse("did:core:0x1234567890").unwrap();
    assert!(actual_holder_url != issuer_url);

    assert!(CredentialValidator::check_subject_holder_relationship(
      &credential,
      &issuer_url,
      SubjectHolderRelationship::AlwaysSubject
    )
    .is_err());

    assert!(CredentialValidator::check_subject_holder_relationship(
      &credential,
      &issuer_url,
      SubjectHolderRelationship::SubjectOnNonTransferable
    )
    .is_err());

    assert!(CredentialValidator::check_subject_holder_relationship(
      &credential,
      &issuer_url,
      SubjectHolderRelationship::Any
    )
    .is_ok());

    let mut credential_transferable = credential.clone();

    credential_transferable.non_transferable = Some(false);

    assert!(CredentialValidator::check_subject_holder_relationship(
      &credential_transferable,
      &issuer_url,
      SubjectHolderRelationship::SubjectOnNonTransferable
    )
    .is_ok());

    credential_transferable.non_transferable = None;

    assert!(CredentialValidator::check_subject_holder_relationship(
      &credential_transferable,
      &issuer_url,
      SubjectHolderRelationship::SubjectOnNonTransferable
    )
    .is_ok());

    // two subjects (even when they are both the holder) should fail for all defined values except "Any"
    let mut credential_duplicated_holder = credential;
    credential_duplicated_holder
      .credential_subject
      .push(Subject::with_id(actual_holder_url));

    assert!(CredentialValidator::check_subject_holder_relationship(
      &credential_duplicated_holder,
      &issuer_url,
      SubjectHolderRelationship::AlwaysSubject
    )
    .is_err());

    assert!(CredentialValidator::check_subject_holder_relationship(
      &credential_duplicated_holder,
      &issuer_url,
      SubjectHolderRelationship::SubjectOnNonTransferable
    )
    .is_err());

    assert!(CredentialValidator::check_subject_holder_relationship(
      &credential_duplicated_holder,
      &issuer_url,
      SubjectHolderRelationship::Any
    )
    .is_ok());
  }

  #[test]
  fn simple_expires_on_or_after_with_expiration_date() {
    let later_than_expiration_date = SIMPLE_CREDENTIAL
      .expiration_date
      .unwrap()
      .checked_add(Duration::minutes(1))
      .unwrap();
    assert!(CredentialValidator::check_expires_on_or_after(&SIMPLE_CREDENTIAL, later_than_expiration_date).is_err());
    // and now with an earlier date
    let earlier_date = Timestamp::parse("2019-12-27T11:35:30Z").unwrap();
    assert!(CredentialValidator::check_expires_on_or_after(&SIMPLE_CREDENTIAL, earlier_date).is_ok());
  }

  // test with a few timestamps that should be RFC3339 compatible
  proptest! {
    #[test]
    fn property_based_expires_after_with_expiration_date(seconds in 0..1_000_000_000_u32) {
      let after_expiration_date = SIMPLE_CREDENTIAL.expiration_date.unwrap().checked_add(Duration::seconds(seconds)).unwrap();
      let before_expiration_date = SIMPLE_CREDENTIAL.expiration_date.unwrap().checked_sub(Duration::seconds(seconds)).unwrap();
      assert!(CredentialValidator::check_expires_on_or_after(&SIMPLE_CREDENTIAL, after_expiration_date).is_err());
      assert!(CredentialValidator::check_expires_on_or_after(&SIMPLE_CREDENTIAL, before_expiration_date).is_ok());
    }
  }

  proptest! {
    #[test]
    fn property_based_expires_after_no_expiration_date(seconds in FIRST_RFC3999_COMPATIBLE_UNIX_TIMESTAMP..LAST_RFC3339_COMPATIBLE_UNIX_TIMESTAMP) {
      let mut credential = SIMPLE_CREDENTIAL.clone();
      credential.expiration_date = None;
      // expires after whatever the timestamp may be because the expires_after field is None.
      assert!(CredentialValidator::check_expires_on_or_after(&credential, Timestamp::from_unix(seconds).unwrap()).is_ok());
    }
  }

  proptest! {
    #[test]
    fn property_based_issued_before(seconds in 0 ..1_000_000_000_u32) {

      let earlier_than_issuance_date = SIMPLE_CREDENTIAL.issuance_date.checked_sub(Duration::seconds(seconds)).unwrap();
      let later_than_issuance_date = SIMPLE_CREDENTIAL.issuance_date.checked_add(Duration::seconds(seconds)).unwrap();
      assert!(CredentialValidator::check_issued_on_or_before(&SIMPLE_CREDENTIAL, earlier_than_issuance_date).is_err());
      assert!(CredentialValidator::check_issued_on_or_before(&SIMPLE_CREDENTIAL, later_than_issuance_date).is_ok());
    }
  }
}
