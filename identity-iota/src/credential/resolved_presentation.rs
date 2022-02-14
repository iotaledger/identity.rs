// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Object;
use identity_core::common::OneOrMany;
use identity_credential::credential::Credential;
use identity_credential::presentation::Presentation;
use identity_did::verifiable::VerifierOptions;
use serde::Serialize;

use super::errors::ValidationError;
use super::presentation_validator::PresentationValidator;
use super::CredentialValidator;
use super::PresentationValidationOptions;
use super::ResolvedCredential;
use crate::did::IotaDID;
use crate::document::ResolvedIotaDocument;
use crate::tangle::TangleResolve;
use crate::Error;
use crate::Result;

/// A verifiable presentation whose associated DID documents have been resolved from the Tangle.
///
/// This struct enables low-level control over how a [`Presentation`] gets validated by offering the following
/// validation units
/// - [`Self::verify_signature()`]
/// - [`Self::check_non_transferable()`]
/// - [`Self::check_structure()`]
///
/// # Security
/// This struct uses resolved DID Documents received upon construction. These associated documents may become outdated
/// at any point in time and will then no longer be fit for purpose. We encourage disposing these objects as soon as
/// possible.
#[non_exhaustive]
pub struct ResolvedPresentation<T = Object, U = Object> {
  pub presentation: Presentation<T, U>,
  pub holder: ResolvedIotaDocument,
  pub credential_issuers: Vec<ResolvedIotaDocument>,
}

//Todo: Provide a method to construct this object on the new resolver.

impl<T: Serialize, U: Serialize + PartialEq> ResolvedPresentation<T, U> {}

impl<T: Serialize, U: Serialize + PartialEq + Clone> ResolvedPresentation<T, U> {
  /// Validate the presentation using the Resolved DID documents of the holders and credential issuers received upon
  /// creation.
  ///
  /// # Errors
  /// Fails if any of the following conditions occur
  /// - The structure of the presentation is not semantically valid
  /// - The nonTransferable property is set in one of the credentials, but the credential's subject is not the holder of
  ///   the presentation.
  /// - Validation of any of the presentation's credentials fails.
  // Todo: This method does currently not fail on deactivated subject documents in the ResolvedCredentials. Should it?
  pub fn validate(&self, options: &PresentationValidationOptions) -> Result<()> {
    PresentationValidator::new(&self.presentation).full_validation(
      options,
      &self.holder,
      &self.credential_issuers.as_slice(),
    )
  }
}
