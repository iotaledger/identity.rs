// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;
use std::str::FromStr;

use identity_core::common::Timestamp;
use identity_core::common::Url;
use identity_core::convert::FromJson;
use identity_did::CoreDID;
use identity_did::DID;
use identity_document::document::CoreDocument;
use identity_verification::jws::DecodedJws;
use identity_verification::jws::Decoder;
use identity_verification::jws::EdDSAJwsVerifier;
use identity_verification::jws::JwsVerifier;

use crate::credential::Jwt;
use crate::presentation::JwtPresentation;
use crate::presentation::PresentationJwtClaims;
use crate::validator::vc_jwt_validation::CompoundCredentialValidationError;
use crate::validator::vc_jwt_validation::CredentialValidator;
use crate::validator::vc_jwt_validation::DecodedJwtCredential;
use crate::validator::vc_jwt_validation::SignerContext;
use crate::validator::vc_jwt_validation::ValidationError;
use crate::validator::FailFast;

use super::CompoundJwtPresentationValidationError;
use super::DecodedJwtPresentation;
use super::JwtPresentationValidationOptions;

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct PresentationJwtValidator<V: JwsVerifier = EdDSAJwsVerifier>(V);

impl PresentationJwtValidator {
  pub fn new() -> Self {
    Self(EdDSAJwsVerifier::default())
  }
}
impl<V> PresentationJwtValidator<V>
where
  V: JwsVerifier,
{
  pub fn with_signature_verifier(signature_verifier: V) -> Self {
    Self(signature_verifier)
  }

  pub fn validate<HDOC, IDOC, T, U>(
    &self,
    presentation: &Jwt,
    holder: &HDOC,
    issuers: &[IDOC],
    options: &JwtPresentationValidationOptions,
    fail_fast: FailFast,
  ) -> Result<DecodedJwtPresentation<T, U>, CompoundJwtPresentationValidationError>
  where
    HDOC: AsRef<CoreDocument> + ?Sized,
    IDOC: AsRef<CoreDocument>,
    T: ToOwned<Owned = T> + serde::Serialize + serde::de::DeserializeOwned,
    U: ToOwned<Owned = U> + serde::Serialize + serde::de::DeserializeOwned,
  {
    // Verify that holder document matches holder in presentation.
    let holder_did: CoreDID = Self::extract_holder::<CoreDID, T>(presentation)
      .map_err(|err| CompoundJwtPresentationValidationError::one_prsentation_error(err))?;

    if &holder_did != <CoreDocument>::id(holder.as_ref()) {
      return Err(CompoundJwtPresentationValidationError::one_prsentation_error(
        ValidationError::DocumentMismatch(SignerContext::Holder),
      ));
    }

    // Verify JWS.
    let decoded_jws: DecodedJws<'_> = holder
      .as_ref()
      .verify_jws(
        presentation.as_str(),
        None,
        &self.0,
        &options.presentation_verifier_options,
      )
      .map_err(|err| {
        CompoundJwtPresentationValidationError::one_prsentation_error(ValidationError::PresentationJwsError(err))
      })?;

    let claims: PresentationJwtClaims<'_, T> =
      PresentationJwtClaims::from_json_slice(&decoded_jws.claims).map_err(|err| {
        CompoundJwtPresentationValidationError::one_prsentation_error(ValidationError::PresentationStructure(
          crate::Error::JwtClaimsSetDeserializationError(err.into()),
        ))
      })?;

    // Check the expiration date.
    let expiration_date: Option<Timestamp> = claims
      .exp
      .map(|exp| {
        Timestamp::from_unix(exp).map_err(|err| {
          CompoundJwtPresentationValidationError::one_prsentation_error(ValidationError::PresentationStructure(
            crate::Error::JwtClaimsSetDeserializationError(err.into()),
          ))
        })
      })
      .transpose()?;

    (expiration_date.is_none() || expiration_date >= Some(options.earliest_expiry_date.unwrap_or_default()))
      .then_some(())
      .ok_or(CompoundJwtPresentationValidationError::one_prsentation_error(
        ValidationError::ExpirationDate,
      ))?;

    // Check issuance date.
    let issuance_date: Option<Timestamp> = claims
      .issuance_date
      .map(|iss| {
        iss.to_issuance_date().map_err(|err| {
          CompoundJwtPresentationValidationError::one_prsentation_error(ValidationError::PresentationStructure(
            crate::Error::JwtClaimsSetDeserializationError(err.into()),
          ))
        })
      })
      .transpose()?;

    (issuance_date.is_none() || issuance_date <= Some(options.latest_issuance_date.unwrap_or_default()))
      .then_some(())
      .ok_or(CompoundJwtPresentationValidationError::one_prsentation_error(
        ValidationError::ExpirationDate,
      ))?;

    let aud = claims.aud.clone();

    let presentation: JwtPresentation<T> = claims.try_into_presentation().map_err(|err| {
      CompoundJwtPresentationValidationError::one_prsentation_error(ValidationError::PresentationStructure(err))
    })?;

    // Validate credentials.
    let credentials: Vec<DecodedJwtCredential<U>> = self
      .validate_credentials::<IDOC, T, U>(&presentation, issuers, options, fail_fast)
      .map_err(|err| CompoundJwtPresentationValidationError {
        credential_errors: err,
        presentation_validation_errors: vec![],
      })?;

    let decoded_jwt_presentation: DecodedJwtPresentation<T, U> = DecodedJwtPresentation {
      presentation,
      header: Box::new(decoded_jws.protected),
      expiration_date,
      aud,
      credentials,
    };

    Ok(decoded_jwt_presentation)
  }

  pub fn extract_holder<D: DID, T>(presentation: &Jwt) -> std::result::Result<D, ValidationError>
  where
    T: ToOwned<Owned = T> + serde::Serialize + serde::de::DeserializeOwned,
    <D as FromStr>::Err: std::error::Error + Send + Sync + 'static,
  {
    let validation_item = Decoder::new()
      .decode_compact_serialization(presentation.as_str().as_bytes(), None)
      .map_err(ValidationError::JwsDecodingError)?;

    let claims: PresentationJwtClaims<'_, T> = PresentationJwtClaims::from_json_slice(&validation_item.claims())
      .map_err(|err| {
        ValidationError::PresentationStructure(crate::Error::JwtClaimsSetDeserializationError(err.into()))
      })?;
    let iss: Url = claims.iss.into_owned();
    D::from_str(iss.as_str()).map_err(|err| ValidationError::SignerUrl {
      signer_ctx: SignerContext::Holder,
      source: err.into(),
    })
  }

  fn validate_credentials<DOC, T, U>(
    &self,
    presentation: &JwtPresentation<T>,
    issuers: &[DOC],
    options: &JwtPresentationValidationOptions,
    fail_fast: FailFast,
  ) -> Result<Vec<DecodedJwtCredential<U>>, BTreeMap<usize, CompoundCredentialValidationError>>
  where
    DOC: AsRef<CoreDocument>,
    T: ToOwned<Owned = T> + serde::Serialize + serde::de::DeserializeOwned,
    U: ToOwned<Owned = U> + serde::Serialize + serde::de::DeserializeOwned,
  {
    let number_of_credentials = presentation.verifiable_credential.len();
    let mut decoded_credentials: Vec<DecodedJwtCredential<U>> = vec![];
    let credential_errors_iter = presentation
      .verifiable_credential
      .iter()
      .map(|credential| {
        CredentialValidator::<V>::validate_extended::<DOC, V, U>(
          &self.0,
          credential,
          issuers,
          &options.shared_validation_options,
          presentation
            .holder
            .as_ref()
            .map(|holder_url| (holder_url, options.subject_holder_relationship)),
          fail_fast,
        )
      })
      .enumerate()
      .filter_map(|(position, result)| {
        if let Ok(decoded_credential) = result {
          decoded_credentials.push(decoded_credential);
          None
        } else {
          result.err().map(|error| (position, error))
        }
      });

    let credential_errors: BTreeMap<usize, CompoundCredentialValidationError> = credential_errors_iter
      .take(match fail_fast {
        FailFast::FirstError => 1,
        FailFast::AllErrors => number_of_credentials,
      })
      .collect();

    if credential_errors.is_empty() {
      Ok(decoded_credentials)
    } else {
      Err(credential_errors)
    }
  }
}