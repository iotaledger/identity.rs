// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Object;
use identity_core::common::OneOrMany;
use identity_credential::credential::Credential;
use identity_credential::presentation::Presentation;
use identity_did::verifiable::VerifierOptions;
use serde::Serialize;

use super::errors::DocumentAssociationError;
use super::ResolvedCredential;
use crate::did::IotaDID;
use crate::document::ResolvedIotaDocument;
use crate::Error;
use crate::Result;

/// A verifiable presentation whose associated DID documents have been resolved from the Tangle.
///
/// This struct enables low-level control over how a [`Presentation`] gets validated by offering the following
/// validation units
/// - [`Self::verify_signature()`]
/// - [`Self::check_non_transferable()`]
/// - [`Self::check_structure()`]
pub struct ResolvedPresentation<T = Object, U = Object> {
  pub(crate) presentation: Presentation<T, U>,
  pub(crate) holder: ResolvedIotaDocument,
  pub(crate) resolved_credentials: OneOrMany<ResolvedCredential<U>>,
}

impl<T: Serialize, U: Serialize + PartialEq> ResolvedPresentation<T, U> {
  /// Combines a [`Presentation`] with the [`ResolvedIotaDocument`] belonging to the holder and the
  /// [`ResolvedCredential`]s corresponding to the presentation's credentials.
  ///
  /// # Errors
  /// Fails if the presentation's holder property does not have a url corresponding to the `holder`,
  /// or `resolved_credentials` contains credentials that cannot be found in the presentations `verifiable_credential`
  /// property.
  pub fn try_new(
    &self,
    presentation: Presentation<T, U>,
    holder: ResolvedIotaDocument,
    resolved_credentials: OneOrMany<ResolvedCredential<U>>,
  ) -> Result<Self> {
    // check that the holder corresponds with the holder stated in the presentation.
    //  need to parse a valid IotaDID from the presentation's holder and check that the DID matches with the provided
    // resolved DID document

    let presentation_holder_did: Result<IotaDID> = presentation
      .holder
      .clone()
      .ok_or(Error::InvalidPresentationPairing(
        DocumentAssociationError::UnrelatedHolder,
      ))?
      .as_str()
      .parse();
    if let Ok(did) = presentation_holder_did {
      if &did != holder.document.id() {
        return Err(Error::InvalidPresentationPairing(
          DocumentAssociationError::UnrelatedHolder,
        ));
      }
    } else {
      return Err(Error::InvalidPresentationPairing(
        DocumentAssociationError::UnrelatedHolder,
      ));
    }

    // check that the resolved credentials correspond to the presentation's credentials
    for (position, resolved_credential) in resolved_credentials.iter().enumerate() {
      if !presentation
        .verifiable_credential
        .contains(&resolved_credential.credential)
      {
        return Err(Error::InvalidPresentationPairing(
          DocumentAssociationError::UnrelatedCredentials { position },
        ));
      }
    }

    Ok(Self {
      presentation,
      holder,
      resolved_credentials,
    })
  }

  /// Verify the signature using the holders's DID document.
  ///
  /// # Terminology
  /// This method is a *validation unit*
  pub fn verify_signature(&self, options: &VerifierOptions) -> Result<()> {
    self
      .holder
      .document
      .verify_data(&self.presentation, options)
      .map_err(|err| super::errors::StandaloneValidationError::HolderProof { source: err.into() })
      .map_err(Into::into)
  }
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

  /// Unpack [`Self`] into a triple consisting of the presentation, the holder's DID Document and a collection of
  /// resolved credentials respectively.
  pub fn de_assemble(
    self,
  ) -> (
    Presentation<T, U>,
    ResolvedIotaDocument,
    OneOrMany<ResolvedCredential<U>>,
  ) {
    (self.presentation, self.holder, self.resolved_credentials)
  }
}
