// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Object;
use identity_credential::credential::Credential;
use identity_credential::presentation::Presentation;

use super::ResolvedCredential;
use crate::document::ResolvedIotaDocument;
use crate::Result;

/// A verifiable presentation whose associated DID documents have been resolved from the Tangle.
pub struct ResolvedPresentation<T = Object, U = Object> {
  pub presentation: Presentation<T, U>,
  pub holder: ResolvedIotaDocument,
  pub credentials: Vec<ResolvedCredential<U>>,
}

impl<T, U> ResolvedPresentation<T, U> {
  delegate::delegate! {
      to self.presentation {
        /// An iterator over the credentials (with their corresponding position in the presentation) that have the
  /// `nonTransferable` property set, but the credential subject id does not correspond to URL of the presentation's
  /// holder
          pub fn non_transferable_violations(&self) -> impl Iterator<Item = (usize, &Credential<U>)> + '_ ;

      }
  }

  /// Validates the semantic structure of the `Presentation`.
  ///
  /// # Terminology
  /// This is a *validation unit*.
  pub fn check_structure(&self) -> Result<()> {
    self
      .presentation
      .check_structure()
      .map_err(super::errors::StandaloneValidationError::PresentationStructure)
      .map_err(Into::into)
  }

  /// Validates that the nonTransferable property is met.
  ///
  /// # Errors
  /// Returns at the first credential requiring a nonTransferable property that is not met.
  ///
  /// If one needs to find *all* the nonTransferable violations of this presentation, then see
  /// [Self::non_transferable_violations] .
  ///
  /// # Terminology
  ///
  /// This is a *validation unit*
  pub fn check_non_transferable(&self) -> Result<()> {
    if let Some((position, _)) = self.non_transferable_violations().next() {
      let err = super::errors::StandaloneValidationError::NonTransferableViolation {
        credential_position: position,
      };
      Err(err.into())
    } else {
      Ok(())
    }
  }
}
