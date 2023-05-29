// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::Display;

use crate::validator::vc_jwt_validation::CompoundCredentialValidationError;
use crate::validator::vc_jwt_validation::ValidationError;

#[derive(Debug)]
/// An error caused by a failure to validate a `JwtPresentation`.
pub struct CompoundJwtPresentationValidationError {
  /// Errors that occurred during validation of individual credentials, mapped by index of their
  /// order in the presentation.
  pub credential_errors: BTreeMap<usize, CompoundCredentialValidationError>,
  /// Errors that occurred during validation of the presentation.
  pub presentation_validation_errors: Vec<ValidationError>,
}

impl CompoundJwtPresentationValidationError {
  pub(crate) fn one_prsentation_error(error: ValidationError) -> Self {
    Self {
      credential_errors: BTreeMap::new(),
      presentation_validation_errors: vec![error],
    }
  }
}

impl Display for CompoundJwtPresentationValidationError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let credential_error_formatter = |(position, reason): (&usize, &CompoundCredentialValidationError)| -> String {
      format!("credential num. {} errors: {}", position, reason.to_string().as_str())
    };

    let error_string_iter = self
      .presentation_validation_errors
      .iter()
      .map(|error| error.to_string())
      .chain(self.credential_errors.iter().map(credential_error_formatter));
    let detailed_information: String = itertools::intersperse(error_string_iter, "; ".to_string()).collect();
    write!(f, "[{detailed_information}]")
  }
}

impl Error for CompoundJwtPresentationValidationError {}
