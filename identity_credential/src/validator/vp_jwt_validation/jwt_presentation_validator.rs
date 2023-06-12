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

/// Struct for validating [`JwtPresentation`].
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct JwtPresentationValidator<V: JwsVerifier = EdDSAJwsVerifier>(V);

impl JwtPresentationValidator {
  /// Creates a new [`JwtPresentationValidator`].
  pub fn new() -> Self {
    Self(EdDSAJwsVerifier::default())
  }
}
impl Default for JwtPresentationValidator {
  fn default() -> Self {
    Self::new()
  }
}

impl<V> JwtPresentationValidator<V>
where
  V: JwsVerifier,
{
  /// Creates a new [`JwtPresentationValidator`] using a specific [`JwsVerifier`].
  pub fn with_signature_verifier(signature_verifier: V) -> Self {
    Self(signature_verifier)
  }

  /// Validates a [`JwtPresentation`].
  ///
  /// The following properties are validated according to `options`:
  /// - the JWT can be decoded into semantically valid presentation.
  /// - the expiration and issuance date contained in the JWT claims.
  /// - the holder's signature.
  /// - the relationship between the holder and the credential subjects.
  /// - the signatures and some properties of the constituent credentials (see [`CredentialValidator`]).
  ///
  /// Validation is done with respect to the properties set in `options`.
  ///
  /// # Warning
  /// The lack of an error returned from this method is in of itself not enough to conclude that the presentation can be
  /// trusted. This section contains more information on additional checks that should be carried out before and after
  /// calling this method.
  ///
  /// ## The state of the supplied DID Documents.
  /// The caller must ensure that the DID Documents in `holder` and `issuers` are up-to-date.
  ///
  /// ## Properties that are not validated
  ///  There are many properties defined in [The Verifiable Credentials Data Model](https://www.w3.org/TR/vc-data-model/) that are **not** validated, such as:
  /// `credentialStatus`, `type`, `credentialSchema`, `refreshService`, **and more**.
  /// These should be manually checked after validation, according to your requirements.
  ///
  /// # Errors
  /// An error is returned whenever a validated condition is not satisfied or when decoding fails.
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
        CompoundJwtPresentationValidationError::one_presentation_error(ValidationError::PresentationJwsError(err))
      })?;

    let claims: PresentationJwtClaims<'_, T> =
      PresentationJwtClaims::from_json_slice(&decoded_jws.claims).map_err(|err| {
        CompoundJwtPresentationValidationError::one_presentation_error(ValidationError::PresentationStructure(
          crate::Error::JwtClaimsSetDeserializationError(err.into()),
        ))
      })?;

    // Verify that holder document matches holder in presentation.
    let holder_did: CoreDID = CoreDID::from_str(claims.iss.as_str()).map_err(|err| {
      CompoundJwtPresentationValidationError::one_presentation_error(ValidationError::SignerUrl {
        signer_ctx: SignerContext::Holder,
        source: err.into(),
      })
    })?;

    if &holder_did != <CoreDocument>::id(holder.as_ref()) {
      return Err(CompoundJwtPresentationValidationError::one_presentation_error(
        ValidationError::DocumentMismatch(SignerContext::Holder),
      ));
    }

    // Check the expiration date.
    let expiration_date: Option<Timestamp> = claims
      .exp
      .map(|exp| {
        Timestamp::from_unix(exp).map_err(|err| {
          CompoundJwtPresentationValidationError::one_presentation_error(ValidationError::PresentationStructure(
            crate::Error::JwtClaimsSetDeserializationError(err.into()),
          ))
        })
      })
      .transpose()?;

    (expiration_date.is_none() || expiration_date >= Some(options.earliest_expiry_date.unwrap_or_default()))
      .then_some(())
      .ok_or(CompoundJwtPresentationValidationError::one_presentation_error(
        ValidationError::ExpirationDate,
      ))?;

    // Check issuance date.
    let issuance_date: Option<Timestamp> = match claims.issuance_date {
      Some(iss) => {
        if iss.iat.is_some() || iss.nbf.is_some() {
          Some(iss.to_issuance_date().map_err(|err| {
            CompoundJwtPresentationValidationError::one_presentation_error(ValidationError::PresentationStructure(
              crate::Error::JwtClaimsSetDeserializationError(err.into()),
            ))
          })?)
        } else {
          None
        }
      }
      None => None,
    };

    (issuance_date.is_none() || issuance_date <= Some(options.latest_issuance_date.unwrap_or_default()))
      .then_some(())
      .ok_or(CompoundJwtPresentationValidationError::one_presentation_error(
        ValidationError::IssuanceDate,
      ))?;

    let aud: Option<Url> = claims.aud.clone();

    let presentation: JwtPresentation<T> = claims.try_into_presentation().map_err(|err| {
      CompoundJwtPresentationValidationError::one_presentation_error(ValidationError::PresentationStructure(err))
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
      issuance_date,
      aud,
      credentials,
    };

    Ok(decoded_jwt_presentation)
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
          Some((&presentation.holder, options.subject_holder_relationship)),
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

impl JwtPresentationValidator {
  /// Attempt to extract the holder of the presentation and the issuers of the included
  /// credentials.
  ///
  /// # Errors:
  /// * If deserialization/decoding of the presentation or any of the credentials
  /// fails.
  /// * If the holder or any of the issuers can't be parsed as DIDs.
  ///
  /// Returned tuple: (presentation_holder, credentials_issuers).
  pub fn extract_dids<H: DID, I: DID, T, U>(presentation: &Jwt) -> std::result::Result<(H, Vec<I>), ValidationError>
  where
    T: ToOwned<Owned = T> + serde::Serialize + serde::de::DeserializeOwned,
    U: ToOwned<Owned = U> + serde::Serialize + serde::de::DeserializeOwned,
    <H as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    <I as FromStr>::Err: std::error::Error + Send + Sync + 'static,
  {
    let validation_item = Decoder::new()
      .decode_compact_serialization(presentation.as_str().as_bytes(), None)
      .map_err(ValidationError::JwsDecodingError)?;

    let claims: PresentationJwtClaims<'_, T> = PresentationJwtClaims::from_json_slice(&validation_item.claims())
      .map_err(|err| {
        ValidationError::PresentationStructure(crate::Error::JwtClaimsSetDeserializationError(err.into()))
      })?;

    let holder: H = H::from_str(claims.iss.as_str()).map_err(|err| ValidationError::SignerUrl {
      signer_ctx: SignerContext::Holder,
      source: err.into(),
    })?;

    let mut issuers: Vec<I> = vec![];
    for vc in claims.vp.verifiable_credential.iter() {
      issuers.push(CredentialValidator::extract_issuer_from_jwt::<I, U>(vc)?)
    }
    Ok((holder, issuers))
  }

  /// Validates the semantic structure of the `JwtPresentation`.
  pub fn check_structure<U>(presentation: &JwtPresentation<U>) -> Result<(), ValidationError> {
    presentation
      .check_structure()
      .map_err(ValidationError::PresentationStructure)
  }
}
