// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::convert::FromJson;
use identity_document::document::CoreDocument;
use identity_verification::jws::DecodedJws;
use identity_verification::jws::EdDSAJwsSignatureVerifier;
use identity_verification::jws::JwsSignatureVerifier;
use identity_verification::jws::JwsSignatureVerifierFn;

use crate::credential::Jwt;
use crate::presentation::PresentationJwtClaims;
use crate::validator::vc_jwt_validation::ValidationError;
use crate::validator::FailFast;

use super::CompoundPresentationValidationError;
use super::JwtPresentationValidationOptions;

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct PresentationJwtValidator {}
type PresentationValidationResult = std::result::Result<(), CompoundPresentationValidationError>;

impl PresentationJwtValidator {
  pub fn validate<HDOC: AsRef<CoreDocument> + ?Sized, IDOC: AsRef<CoreDocument>>(
    presentation: &Jwt,
    holder: &HDOC,
    issuers: &[IDOC],
    options: &JwtPresentationValidationOptions,
    fail_fast: FailFast,
  ) -> PresentationValidationResult {
    let decoded_jws = holder
      .as_ref()
      .verify_jws(
        presentation.as_str(),
        None,
        &EdDSAJwsSignatureVerifier::default(),
        &options.presentation_verifier_options,
      )
      .unwrap();

    let claims: PresentationJwtClaims = PresentationJwtClaims::from_json_slice(&decoded_jws.claims).map_err(|err| {
      CompoundPresentationValidationError::one_prsentation_error(ValidationError::PresentationStructure(
        crate::Error::JwtClaimsSetDeserializationError(err.into()),
      ))
    })?;

    Ok(())
  }
}
