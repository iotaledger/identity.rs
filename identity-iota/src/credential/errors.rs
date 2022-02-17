// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;
use std::fmt::Display;

use identity_core::common::OneOrMany;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
/// An error associated with validating credentials and presentations.
pub enum ValidationError {
  /// Indicates that the expiration date of the credential is not considered valid.
  #[error("the expiration date is in the past or earlier than required.")]
  ExpirationDate,
  /// Indicates that the issuance date of the credential is not considered valid.
  #[error("the credential is yet to be active or has not been active for the required period.")]
  IssuanceDate,
  /// Indicates that the credential's signature could not be verified using the issuer's DID Document.
  #[error("could not verify the issuer's signature")]
  IssuerProof {
    source: Box<dyn std::error::Error + Send + Sync + 'static>, /* Todo: Would it be better to use a specific type
                                                                 * * here? */
  },
  /// Indicates an attempt to verify a credential's signature with a resolved DID document not corresponding to the
  /// URL of the credential's issuer.
  #[error("the provided issuer document does not correspond to the credential's issuer")]
  IncompatibleIssuerDocument,

  /// Indicates that the credential's issuer could not be parsed as a valid DID.
  #[error("the credential's issuer property could not be parsed to a valid DID")]
  IssuerUrl {
    source: Box<dyn std::error::Error + Send + Sync + 'static>, /* Todo: Would it be better to use a specific type
                                                                 * here? */
  },

  /// Indicates that the credential's issuer could not be parsed as a valid DID.
  #[error("the presentation's holder property could not be parsed to a valid DID")]
  HolderUrl {
    source: Box<dyn std::error::Error + Send + Sync + 'static>, /* Todo: Would it be better to use a specific type
                                                                 * here? */
  },

  /// Indicates an attempt to verify a presentation's signature with a resolved DID document not corresponding to the
  /// URL of the presentation's holder property.
  #[error("the provided holder document does not correspond to the presentation's holder property")]
  IncompatibleHolderDocument,

  /// Indicates that the presentation's signature could not be verified using the holder's DID Document.
  #[error("could not verify the holder's signature")]
  HolderProof {
    source: Box<dyn std::error::Error + Send + Sync + 'static>, /* Todo: Would it be better to use a specific type
                                                                 * here? */
  },
  /// Indicates that the structure of the [identity_credential::credential::Credential] is not semantically correct.
  #[error("the credential's structure is not semantically correct")]
  CredentialStructure(#[source] identity_credential::Error),
  /// Indicates that the structure of the [identity_credential::presentation::Presentation] is not semantically
  /// correct.
  #[error("the presentation's structure is not semantically correct")]
  PresentationStructure(#[source] identity_credential::Error),
  /// Indicates that the presentation does not comply with the nonTransferable property of one of its credentials.
  #[error("expected holder = subject of the credential at position {credential_position}")]
  NonTransferableViolation { credential_position: usize },
  /// Indicates that the presentation does not have a holder.
  #[error("the presentation has an empty holder property")]
  MissingPresentationHolder,
}

// Todo: Consider implementing Error for OneOrMany<E: std::error::Error> to avoid wrapping it in
// AccumulatedCredentialValidationError
#[derive(Debug)]
/// An error caused by a failure to validate a Credential.  
pub struct AccumulatedCredentialValidationError {
  pub validation_errors: OneOrMany<ValidationError>,
}

impl Display for AccumulatedCredentialValidationError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    // intersperse might become available in the standard library soon: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.intersperse
    let detailed_information: String = itertools::intersperse(
      self.validation_errors.iter().map(|err| err.to_string()),
      ", ".to_string(),
    )
    .collect();
    write!(f, "The following errors occurred: {}", detailed_information)
  }
}

impl std::error::Error for AccumulatedCredentialValidationError {}

#[derive(Debug)]
/// An error caused by a failure to validate a Presentation.
pub struct AccumulatedPresentationValidationError {
  pub credential_errors: BTreeMap<usize, AccumulatedCredentialValidationError>,
  pub presentation_validation_errors: Vec<ValidationError>,
}

impl Display for AccumulatedPresentationValidationError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let credential_error_formatter = |(position, reason): (&usize, &AccumulatedCredentialValidationError)| -> String {
      format!(
        "could not validate credential at position {}. The following errors occurred {}",
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
    write!(f, "the following errors occurred: {}", detailed_information)
  }
}

impl std::error::Error for AccumulatedPresentationValidationError {}
