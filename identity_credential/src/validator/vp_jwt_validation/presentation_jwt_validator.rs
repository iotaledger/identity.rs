// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Timestamp;
use identity_core::convert::FromJson;
use identity_document::document::CoreDocument;
use identity_verification::jws::EdDSAJwsSignatureVerifier;
use identity_verification::jws::JwsSignatureVerifier;

use crate::credential::Jwt;
use crate::presentation::JwtPresentation;
use crate::presentation::PresentationJwtClaims;
use crate::validator::vc_jwt_validation::CredentialValidator;
use crate::validator::vc_jwt_validation::ValidationError;
use crate::validator::FailFast;

use super::CompoundPresentationValidationError;
use super::DecodedJwtPresentation;
use super::JwtPresentationValidationOptions;

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct PresentationJwtValidator<V: JwsSignatureVerifier = EdDSAJwsSignatureVerifier>(V);
// type PresentationValidationResult<T = ()> =
//   std::result::Result<DecodedJwtPresentation<T>, CompoundPresentationValidationError>;

impl<V> PresentationJwtValidator<V>
where
  V: JwsSignatureVerifier,
{
  /// todo
  pub fn validate<HDOC, IDOC, T>(
    presentation: &Jwt,
    holder: &HDOC,
    issuers: &[IDOC],
    options: &JwtPresentationValidationOptions,
    fail_fast: FailFast,
  ) -> Result<DecodedJwtPresentation<T>, CompoundPresentationValidationError>
  where
    HDOC: AsRef<CoreDocument> + ?Sized,
    IDOC: AsRef<CoreDocument>,
    T: ToOwned<Owned = T> + serde::Serialize + serde::de::DeserializeOwned,
  {
    let decoded_jws = holder
      .as_ref()
      .verify_jws(
        presentation.as_str(),
        None,
        &EdDSAJwsSignatureVerifier::default(),
        &options.presentation_verifier_options,
      )
      .unwrap();

    let claims: PresentationJwtClaims<'_, T> =
      PresentationJwtClaims::from_json_slice(&decoded_jws.claims).map_err(|err| {
        CompoundPresentationValidationError::one_prsentation_error(ValidationError::PresentationStructure(
          crate::Error::JwtClaimsSetDeserializationError(err.into()),
        ))
      })?;

    // Check the expiration date
    let expiration_date: Option<Timestamp> = claims
      .exp
      .map(|exp| {
        Timestamp::from_unix(exp).map_err(|err| {
          CompoundPresentationValidationError::one_prsentation_error(ValidationError::PresentationStructure(
            crate::Error::JwtClaimsSetDeserializationError(err.into()),
          ))
        })
      })
      .transpose()?;

    (expiration_date.is_none() || expiration_date >= Some(options.earliest_expiry_date.unwrap_or_default()))
      .then_some(())
      .ok_or(CompoundPresentationValidationError::one_prsentation_error(
        ValidationError::ExpirationDate,
      ))?;

    // Check issuance date.
    let issuance_date: Option<Timestamp> = claims
      .issuance_date
      .map(|iss| {
        iss.to_issuance_date().map_err(|err| {
          CompoundPresentationValidationError::one_prsentation_error(ValidationError::PresentationStructure(
            crate::Error::JwtClaimsSetDeserializationError(err.into()),
          ))
        })
      })
      .transpose()?;

    (issuance_date.is_none() || issuance_date <= Some(options.latest_issuance_date.unwrap_or_default()))
      .then_some(())
      .ok_or(CompoundPresentationValidationError::one_prsentation_error(
        ValidationError::ExpirationDate,
      ))?;

    // Check credentials.
    let credential_validator = CredentialValidator::new();

    let aud = claims.aud.clone();

    let presentation: JwtPresentation<T> = claims.try_into_presentation().map_err(|err| {
      CompoundPresentationValidationError::one_prsentation_error(ValidationError::PresentationStructure(err))
    })?;

    let decoded_jwt_presentation: DecodedJwtPresentation<T> = DecodedJwtPresentation {
      presentation,
      header: Box::new(decoded_jws.protected),
      expiration_date,
      aud,
    };

    for credential in decoded_jwt_presentation.presentation.verifiable_credential.to_vec() {
      credential_validator.validate_extended()
    }

    Ok(decoded_jwt_presentation)
  }
}
