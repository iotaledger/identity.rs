// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;
use std::fmt::Display;

use identity_core::common::OneOrMany;

use crate::did::IotaDIDUrl;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
/// credential operations in this crate such as resolution and validation.  
pub enum ValidationError {
  /// Indicates that the expiration date of the credential is not considered valid.
  #[error("credential validation failed: the expiration date does not satisfy the validation criterea")]
  ExpirationDate,
  /// Indicates that the issuance date of the credential is not considered valid.
  #[error("credential validation failed: the issuance date does not satisfy the validation criterea")]
  IssuanceDate,
  /// The DID document corresponding to `did` has been deactivated.
  #[error("credential validation failed: encountered deactivated subject document")]
  //Todo: Should the did_url be included in the error message? Would it be better in terms of abstraction and
  // flexibility to include more information in a simple String?
  DeactivatedSubjectDocument { did_url: IotaDIDUrl },
  /// The proof verification failed.
  #[error("credential validation failed: could not verify the issuer's signature")]
  IssuerProof {
    source: Box<dyn std::error::Error + Send + Sync + 'static>, /* Todo: Would it be better to use a specific type
                                                                 * here? */
  },
  #[error("presentation validation failed: could not verify the holder's signature")]
  HolderProof {
    source: Box<dyn std::error::Error + Send + Sync + 'static>, /* Todo: Would it be better to use a specific type
                                                                 * here? */
  },
  /// Indicates that the structure of the [identity_credential::credential::Credential] is not spec compliant
  #[error("credential validation failed: the credential's structure is not spec compliant")]
  CredentialStructure { source: identity_credential::Error },
  /// Indicates that the structure of the [identity_credential::presentation::Presentation] is not spec compliant
  #[error("presentation validation failed: the presentation's structure is not spec compliant")]
  PresentationStructure(#[source] identity_credential::Error),
  /// Indicates that the issuer's DID document could not be resolved,
  #[error("credential validation failed: The issuer's DID Document could not be resolved")]
  IssuerDocumentResolution {
    source: Box<dyn std::error::Error + Send + Sync + 'static>, /* Todo: would it be better to use a specific type
                                                                 * here? */
  },
  #[error("presentation validation failed: The holder's DID Document could not be resolved")]
  HolderDocumentResolution {
    source: Box<dyn std::error::Error + Send + Sync + 'static>, /* Todo: would it be better to use a specific type
                                                                 * here? */
  },
  #[error("credential validation failed: Could not resolve a subject's DID Document")]
  SubjectDocumentResolution {
    did_url: IotaDIDUrl, /* Todo: Should did_url be included in the error message? Would it be better to include
                          * additional information in a String? */
    source: Box<dyn std::error::Error + Send + Sync + 'static>, /* Todo: would it be better to use a specific type
                                                                 * here? */
  },
  /// Indicates that the presentation does not comply with the nonTransferable property of one of its credentials
  #[error("presentation validation failed: The nonTransferable property of the credential at position {credential_position} is not met")]
  NonTransferableViolation { credential_position: usize },
}

// Todo: Should the DocumentResolution variants in Error be moved to their own enum?
// If so would then AccumulatedError have an additional field?

#[derive(Debug)]
/// An error caused by a failure to resolve a Credential.  
pub struct CredentialResolutionError {
  pub encountered_errors: OneOrMany<ValidationError>,
}

impl Display for CredentialResolutionError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    // intersperse might become available in the standard library soon: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.intersperse
    let detailed_information: String = itertools::intersperse(
      self.encountered_errors.iter().map(|err| err.to_string()),
      ", ".to_string(),
    )
    .collect();
    write!(
      f,
      "credential resolution was unsuccessful. The following errors occurred: {}",
      detailed_information
    )
  }
}

impl std::error::Error for CredentialResolutionError {}

#[derive(Debug)]
pub struct PresentationResolutionError {
  pub credential_errors: BTreeMap<usize, CredentialResolutionError>,
  pub presentation_validation_errors: Vec<ValidationError>,
}

impl Display for PresentationResolutionError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let credential_error_formatter = |(position, reason): (&usize, &CredentialResolutionError)| -> String {
      format!(
        "could not resolve credential at position {}. The following errors occurred {}",
        position,
        reason.to_string().as_str()
      )
    };

    let error_string_iter = self
      .presentation_validation_errors
      .iter()
      .map(|error| error.to_string())
      .chain(self.credential_errors.iter().map(credential_error_formatter));
    let detailed_information: String = itertools::intersperse(error_string_iter, ", ".to_string()).collect();
    write!(
      f,
      "presentation resolution was unsuccessful. The following errors occurred: {}",
      detailed_information
    )
  }
}

// Todo: should we insert a PhantomData field declared as pub (crate) in CredentialResolutionError and/or
// PresentationResolutionError for future proofing?
