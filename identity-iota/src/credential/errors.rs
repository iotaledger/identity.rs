// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;
use std::fmt::Display;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
/// An error associated with validating credentials and presentations.
pub enum ValidationError {
  /// Indicates that the expiration date of the credential is not considered valid.
  #[error("the expiration date is in the past or earlier than required")]
  ExpirationDate,
  /// Indicates that the issuance date of the credential is not considered valid.
  #[error("issuance date is in the future or later than required")]
  IssuanceDate,
  /// Indicates that the credential's signature could not be verified using the issuer's DID Document.
  #[error("could not verify the issuer's signature")]
  #[non_exhaustive]
  IssuerSignature {
    source: Box<dyn std::error::Error + Send + Sync + 'static>, /* Todo: Would it be better to use a specific type
                                                                 * * here? */
  },

  /// Indicates that the credential's issuer could not be parsed as a valid DID.
  #[error("issuer URL is not a valid DID")]
  #[non_exhaustive]
  IssuerUrl {
    source: Box<dyn std::error::Error + Send + Sync + 'static>, /* Todo: Would it be better to use a specific type
                                                                 * here? */
  },

  /// Indicates that the presentation's holder could not be parsed as a valid DID.
  #[error("the presentation's holder property could not be parsed to a valid DID")]
  #[non_exhaustive]
  HolderUrl {
    source: Box<dyn std::error::Error + Send + Sync + 'static>, /* Todo: Would it be better to use a specific type
                                                                 * here? */
  },

  /// Indicates an attempt to verify a signature of a credential or presentation using a DID Document not matching the
  /// signer's id.
  #[error("the signer's id does not match any of the provided DID Documents")]
  #[non_exhaustive]
  DocumentMismatch,

  /// Indicates that the presentation's signature could not be verified using the holder's DID Document.
  #[error("could not verify the holder's signature")]
  #[non_exhaustive]
  HolderSignature {
    source: Box<dyn std::error::Error + Send + Sync + 'static>, /* Todo: Would it be better to use a specific type
                                                                 * here? */
  },
  /// Indicates that the structure of the [Credential](identity_credential::credential::Credential) is not semantically
  /// correct.
  #[error("the credential's structure is not semantically correct")]
  CredentialStructure(#[source] identity_credential::Error),
  /// Indicates that the structure of the [Presentation](identity_credential::presentation::Presentation) is not
  /// semantically correct.
  #[error("the presentation's structure is not semantically correct")]
  PresentationStructure(#[source] identity_credential::Error),
  /// Indicates that the relationship between the presentation holder and one of the credential subjects is not valid.
  #[error("expected holder = subject of the credential at position {credential_position}")]
  #[non_exhaustive]
  InvalidHolderSubjectRelationship { credential_position: usize },
  /// Indicates that the presentation does not have a holder.
  #[error("the presentation has an empty holder property")]
  MissingPresentationHolder,
}

// AccumulatedCredentialValidationError
#[derive(Debug)]
/// An error caused by a failure to validate a Credential.  
pub struct AccumulatedCredentialValidationError {
  pub validation_errors: Vec<ValidationError>,
}

impl Display for AccumulatedCredentialValidationError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    // intersperse might become available in the standard library soon: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.intersperse
    let detailed_information: String = itertools::intersperse(
      self.validation_errors.iter().map(|err| err.to_string()),
      "; ".to_string(),
    )
    .collect();
    write!(f, "[{}]", detailed_information)
  }
}

impl std::error::Error for AccumulatedCredentialValidationError {}

#[derive(Debug)]
/// An error caused by a failure to validate a Presentation.
pub struct CompoundPresentationValidationError {
  pub credential_errors: BTreeMap<usize, AccumulatedCredentialValidationError>,
  pub presentation_validation_errors: Vec<ValidationError>,
}

impl Display for CompoundPresentationValidationError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let credential_error_formatter = |(position, reason): (&usize, &AccumulatedCredentialValidationError)| -> String {
      format!("credential num. {} errors: {}", position, reason.to_string().as_str())
    };

    let error_string_iter = self
      .presentation_validation_errors
      .iter()
      .map(|error| error.to_string())
      .chain(self.credential_errors.iter().map(credential_error_formatter));
    let detailed_information: String = itertools::intersperse(error_string_iter, "; ".to_string()).collect();
    write!(f, "[{}]", detailed_information)
  }
}

impl std::error::Error for CompoundPresentationValidationError {}
