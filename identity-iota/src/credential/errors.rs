// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;
use std::fmt::Display;

use identity_core::common::OneOrMany;

use crate::did::IotaDIDUrl;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
/// An error associated with validating credentials and presentations.
pub enum StandaloneValidationError {
  /// Indicates that the expiration date of the credential is not considered valid.
  #[error("credential validation failed: the expiration date does not satisfy the validation criterea")]
  ExpirationDate,
  /// Indicates that the issuance date of the credential is not considered valid.
  #[error("credential validation failed: the issuance date does not satisfy the validation criterea")]
  IssuanceDate,
  /// The DID document corresponding to `did` has been deactivated.
  #[error("credential validation failed: encountered deactivated subject document")]
  //Todo: Should the did_url be included in the error message? Would it be better in terms of abstraction and
  // flexibility to include more information in a simple String? Can the `did_url` be problematic in terms of GDPR if
  // it gets written to a log file?
  DeactivatedSubjectDocument { did_url: IotaDIDUrl },
  /// Indicates that the credential's signature could not be verified using the issuer's DID Document.
  #[error("credential validation failed: could not verify the issuer's signature")]
  IssuerProof {
    source: Box<dyn std::error::Error + Send + Sync + 'static>, /* Todo: Would it be better to use a specific type
                                                                 * here? */
  },
  /// Indicates an attempt to validate a credential signed by an untrusted issuer.
  #[error("credential validation failed: the credential is signed by an untrusted issuer")]
  UntrustedIssuer,

  /// Indicates that the credential's issuer could not be parsed as a valid DID.
  #[error("credential validation failed: The issuer property could not be parsed to a valid DID")]
  IssuerUrl,

  /// Indicates that the credential's issuer could not be parsed as a valid DID.
  #[error("presentation validation failed: The holder property could not be parsed to a valid DID")]
  HolderUrl,

  /// Indicates an attempt to validate a presentation using a resolved DID document not corresponding to the URL of the
  /// presentation's holder property.
  #[error("presentation validation failed: The provided holder document does not correspond to the presentation's holder property")]
  IncompatibleHolderDocument,

  /// Indicates that the presentation's signature could not be verified using the holder's DID Document.
  #[error("presentation validation failed: could not verify the holder's signature")]
  HolderProof {
    source: Box<dyn std::error::Error + Send + Sync + 'static>, /* Todo: Would it be better to use a specific type
                                                                 * here? */
  },
  /// Indicates that the structure of the [identity_credential::credential::Credential] is not semantically correct.
  #[error("credential validation failed: the credential's structure is not spec compliant")]
  CredentialStructure(#[source] identity_credential::Error),
  /// Indicates that the structure of the [identity_credential::presentation::Presentation] is not semantically
  /// correct.
  #[error("presentation validation failed: the presentation's structure is not spec compliant")]
  PresentationStructure(#[source] identity_credential::Error),
  /// Indicates that the presentation does not comply with the nonTransferable property of one of its credentials.
  #[error("presentation validation failed: The nonTransferable property of the credential at position {credential_position} is not met")]
  NonTransferableViolation { credential_position: usize },
  /// Indicates that the presentation does not have a holder.
  #[error("presentation validation failed: the presentation is required to have a non-empty holder property")]
  MissingPresentationHolder,
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
/// An error caused by an attempt to group credentials with unrelated resolved DID documents
pub enum CompoundError {
  #[error("could not associate the provided resolved DID document with the credential's issuer")]
  UnrelatedIssuer,
  #[error(
    "the subject data at {position} in the provided vector cannot be associated with any of the credential's subjects"
  )]
  UnrelatedSubjects { position: usize },
  #[error("could not associate the provided resolved DID document with the presentation's holder")]
  UnrelatedHolder,
  #[error("the credential at {position} in the provided resolved credentials cannot be associated with any of the presentation's credentials")]
  UnrelatedCredentials { position: usize },
}

#[derive(Debug)]
/// An error caused by a failure to validate a Credential.  
pub struct AccumulatedCredentialValidationError {
  pub validation_errors: OneOrMany<StandaloneValidationError>,
}

impl Display for AccumulatedCredentialValidationError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    // intersperse might become available in the standard library soon: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.intersperse
    let detailed_information: String = itertools::intersperse(
      self.validation_errors.iter().map(|err| err.to_string()),
      ", ".to_string(),
    )
    .collect();
    write!(
      f,
      "credential validation was unsuccessful. The following errors occurred: {}",
      detailed_information
    )
  }
}

impl std::error::Error for AccumulatedCredentialValidationError {}

#[derive(Debug)]
/// An error caused by a failure to validate a Presentation.
pub struct AccumulatedPresentationValidationError {
  pub credential_errors: BTreeMap<usize, AccumulatedCredentialValidationError>,
  pub presentation_validation_errors: Vec<StandaloneValidationError>,
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
    write!(
      f,
      "presentation validation was unsuccessful. The following errors occurred: {}",
      detailed_information
    )
  }
}

impl std::error::Error for AccumulatedPresentationValidationError {}
