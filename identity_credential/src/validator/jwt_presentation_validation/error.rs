// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::error::Error;
use std::fmt::Display;

use crate::validator::jwt_credential_validation::JwtValidationError;

/// Errors caused by a failure to validate a [`Presentation`](crate::presentation::Presentation).
#[derive(Debug)]
pub struct CompoundJwtPresentationValidationError {
  /// Errors that occurred during validation of the presentation.
  pub presentation_validation_errors: Vec<JwtValidationError>,
}

impl CompoundJwtPresentationValidationError {
  pub(crate) fn one_presentation_error(error: JwtValidationError) -> Self {
    Self {
      presentation_validation_errors: vec![error],
    }
  }
}

impl Display for CompoundJwtPresentationValidationError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let error_string_iter = self
      .presentation_validation_errors
      .iter()
      .map(|error| error.to_string());

    let detailed_information: String = itertools::intersperse(error_string_iter, "; ".to_string()).collect();
    write!(f, "[{detailed_information}]")
  }
}

impl Error for CompoundJwtPresentationValidationError {}
