// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
  /// Caused by one or more failures when validating a presentation.
  #[error("presentation validation failed")]
  PresentationValidationError(#[from] identity_credential::validator::CompoundPresentationValidationError),
  //TODO: IMPROVE ERROR
  #[error("{0}")]
  ResolutionProblem(String),
}
