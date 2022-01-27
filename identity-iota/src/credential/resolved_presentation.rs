// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Object;
use identity_core::common::OneOrMany;
use identity_credential::credential::Credential;
use identity_credential::presentation::Presentation;

use crate::document::ResolvedIotaDocument;

use super::CredentialValidationUnitError;
use super::ResolvedCredential;

/// A verifiable presentation whose associated DID documents have been resolved from the Tangle.
pub struct ResolvedPresentation<T = Object, U = Object> {
  pub presentation: Presentation<T, U>,
  pub holder: ResolvedIotaDocument,
  pub credentials: Vec<ResolvedCredential<U>>,
}

impl<T, U> ResolvedPresentation<T, U> {
  delegate::delegate! {
      to self.presentation {
          /// An iterator over the credentials that have the `nonTransferable` property set, but
          /// the credential subject id does not correspond to URL of the presentation's holder
          pub fn non_transferable_violations(&self) -> impl Iterator<Item = &Credential<U>> + '_ ;

          /// Validates the semantic structure of the `Presentation`.
          pub fn check_structure(&self) -> Result<(), identity_credential::Error>;
      }
  }
}

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum PresentationValidationUnitError {
  /// Indicates that the structure of the `[Presentation]` was found to be not spec compliant.
  #[error("presentation validation failed: the structure of the presentation is not spec compliant: {source}")]
  InvalidStructure {
    source: Box<dyn std::error::Error>, //Todo use a proper error here
  },
  #[error(
    "presentation validation failed: the presentation violates the nonTransferable property of one of its credentials"
  )]
  /// Indicates that the `Presentation` violates the `nonTransferable` property of one of its `[Credential]s`.
  NonTransferableViolation, // Todo: Should the error contain the corresponding credential(s) ?

  /// The proof verification failed.
  #[error("presentation validation failed: could not verify the holders's signature: {source}")]
  InvalidProof {
    source: Box<dyn std::error::Error>, // Todo: Put an actual error type here
  },
}

#[derive(Debug)]
pub struct PresentationResolutionError {
  /// Indicates that one or more credentials could not be resolved either because of validation errors or
  /// because of failing to resolve DID Documents from the tangle.  
    pub credential_validation_errors: Vec<CredentialValidationUnitError>,
  /// Caused by attempting to resolve a `Presentation` that fails to meet the specified validation rules.
    pub presentation_validation_errors: Vec<PresentationValidationUnitError>, 
  /// Caused by a failure to resolve a DID Document from the Tangle
    pub document_resolution_error: Option<Box<dyn std::error::Error>>, // Todo: Use an actual error here
}

//Todo: Implement the Error trait for PresentationResolutionError
