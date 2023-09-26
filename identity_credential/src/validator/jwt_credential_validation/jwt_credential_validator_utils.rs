// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use std::str::FromStr;

use identity_core::common::Object;
use identity_core::common::OneOrMany;
use identity_core::common::Timestamp;
use identity_core::common::Url;
use identity_core::convert::FromJson;
use identity_did::DID;
use identity_verification::jws::Decoder;

use super::JwtValidationError;
use super::SignerContext;
use crate::credential::Credential;
use crate::credential::CredentialJwtClaims;
use crate::credential::Jwt;
use crate::validator::SubjectHolderRelationship;

/// Utility functions for verifying JWT credentials.
#[derive(Debug)]
#[non_exhaustive]
pub struct JwtCredentialValidatorUtils;

type ValidationUnitResult<T = ()> = std::result::Result<T, JwtValidationError>;

impl JwtCredentialValidatorUtils {
  /// Validates the semantic structure of the [`Credential`].
  ///
  /// # Warning
  /// This does not validate against the credential's schema nor the structure of the subject claims.
  pub fn check_structure<T>(credential: &Credential<T>) -> ValidationUnitResult {
    credential
      .check_structure()
      .map_err(JwtValidationError::CredentialStructure)
  }

  /// Validate that the [`Credential`] expires on or after the specified [`Timestamp`].
  pub fn check_expires_on_or_after<T>(credential: &Credential<T>, timestamp: Timestamp) -> ValidationUnitResult {
    let expiration_date: Option<Timestamp> = credential.expiration_date;
    (expiration_date.is_none() || expiration_date >= Some(timestamp))
      .then_some(())
      .ok_or(JwtValidationError::ExpirationDate)
  }

  /// Validate that the [`Credential`] is issued on or before the specified [`Timestamp`].
  pub fn check_issued_on_or_before<T>(credential: &Credential<T>, timestamp: Timestamp) -> ValidationUnitResult {
    (credential.issuance_date <= timestamp)
      .then_some(())
      .ok_or(JwtValidationError::IssuanceDate)
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
      .ok_or(JwtValidationError::SubjectHolderRelationship)
  }

  /// Checks whether the credential status has been revoked.
  ///
  /// Only supports `RevocationBitmap2022`.
  #[cfg(feature = "revocation-bitmap")]
  pub fn check_status<DOC: AsRef<identity_document::document::CoreDocument>, T>(
    credential: &Credential<T>,
    trusted_issuers: &[DOC],
    status_check: crate::validator::StatusCheck,
  ) -> ValidationUnitResult {
    use identity_did::CoreDID;
    use identity_document::document::CoreDocument;

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
          return Err(JwtValidationError::InvalidStatus(crate::Error::InvalidStatus(format!(
            "unsupported type '{}'",
            status.type_
          ))));
        }
        let status: crate::credential::RevocationBitmapStatus =
          crate::credential::RevocationBitmapStatus::try_from(status.clone())
            .map_err(JwtValidationError::InvalidStatus)?;

        // Check the credential index against the issuer's DID Document.
        let issuer_did: CoreDID = Self::extract_issuer(credential)?;
        trusted_issuers
          .iter()
          .find(|issuer| <CoreDocument>::id(issuer.as_ref()) == &issuer_did)
          .ok_or(JwtValidationError::DocumentMismatch(SignerContext::Issuer))
          .and_then(|issuer| Self::check_revocation_bitmap_status(issuer, status))
      }
    }
  }

  /// Check the given `status` against the matching [`RevocationBitmap`] service in the
  /// issuer's DID Document.
  #[cfg(feature = "revocation-bitmap")]
  fn check_revocation_bitmap_status<DOC: AsRef<identity_document::document::CoreDocument> + ?Sized>(
    issuer: &DOC,
    status: crate::credential::RevocationBitmapStatus,
  ) -> ValidationUnitResult {
    use crate::revocation::RevocationDocumentExt;

    let issuer_service_url: identity_did::DIDUrl = status.id().map_err(JwtValidationError::InvalidStatus)?;

    // Check whether index is revoked.
    let revocation_bitmap: crate::revocation::RevocationBitmap = issuer
      .as_ref()
      .resolve_revocation_bitmap(issuer_service_url.into())
      .map_err(|_| JwtValidationError::ServiceLookupError)?;
    let index: u32 = status.index().map_err(JwtValidationError::InvalidStatus)?;
    if revocation_bitmap.is_revoked(index) {
      Err(JwtValidationError::Revoked)
    } else {
      Ok(())
    }
  }

  /// Utility for extracting the issuer field of a [`Credential`] as a DID.
  ///
  /// # Errors
  ///
  /// Fails if the issuer field is not a valid DID.
  pub fn extract_issuer<D, T>(credential: &Credential<T>) -> std::result::Result<D, JwtValidationError>
  where
    D: DID,
    <D as FromStr>::Err: std::error::Error + Send + Sync + 'static,
  {
    D::from_str(credential.issuer.url().as_str()).map_err(|err| JwtValidationError::SignerUrl {
      signer_ctx: SignerContext::Issuer,
      source: err.into(),
    })
  }

  /// Utility for extracting the issuer field of a credential in JWT representation as DID.
  ///
  /// # Errors
  ///
  /// If the JWT decoding fails or the issuer field is not a valid DID.
  pub fn extract_issuer_from_jwt<D>(credential: &Jwt) -> std::result::Result<D, JwtValidationError>
  where
    D: DID,
    <D as FromStr>::Err: std::error::Error + Send + Sync + 'static,
  {
    let validation_item = Decoder::new()
      .decode_compact_serialization(credential.as_str().as_bytes(), None)
      .map_err(JwtValidationError::JwsDecodingError)?;

    let claims: CredentialJwtClaims<'_, Object> = CredentialJwtClaims::from_json_slice(&validation_item.claims())
      .map_err(|err| {
        JwtValidationError::CredentialStructure(crate::Error::JwtClaimsSetDeserializationError(err.into()))
      })?;

    D::from_str(claims.iss.url().as_str()).map_err(|err| JwtValidationError::SignerUrl {
      signer_ctx: SignerContext::Issuer,
      source: err.into(),
    })
  }
}
