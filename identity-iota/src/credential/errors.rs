// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Display;

use identity_core::common::OneOrMany;

use crate::did::IotaDIDUrl;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
/// credential operations in this crate such as resolution and validation.  
pub enum Error {
/// Indicates that the expiration date of the credential is not considered valid.
  #[error("credential validation failed: the expiration date does not satisfy the validation criterea")]
  ExpirationDate,
  /// Indicates that the issuance date of the credential is not considered valid.
  #[error("credential validation failed: the issuance date does not satisfy the validation criterea")]
  IssuanceDate,
  /// The DID document corresponding to `did` has been deactivated.
  #[error("credential validation failed: encountered deactivated subject document")]
  //Todo: Should the did_url be included in the error message? Would it be better in terms of abstraction and flexibility to include more information 
  // in a simple String? 
  DeactivatedSubjectDocument { did_url: IotaDIDUrl },
  /// The proof verification failed.
  #[error("credential validation failed: could not verify the issuer's signature")]
  IssuerProof {
    source: Box<dyn std::error::Error + Send + Sync + 'static>, // Todo: Would it be better to use a specific type here? 
  },
  #[error("presentation validation failed: could not verify the holder's signature")]
  HolderProof {
      source: Box<dyn std::error::Error>, // Todo: Would it be better to use a specific type here? 
  },
  /// Indicates that the structure of the [identity_credential::credential::Credential] is not spec compliant
  #[error("credential validation failed: the credential's structure is not spec compliant")]
  CredentialStructure {
    source: identity_credential::Error, 
  },
  /// Indicates that the structure of the [identity_credential::presentation::Presentation] is not spec compliant
  #[error("presentation validation failed: the presentation's structure is not spec compliant")]
  PresentationStructure {
      source: identity_credential::Error, 
  },
  /// Indicates that the issuer's DID document could not be resolved, 
  #[error("credential validation failed: The issuer's DID Document could not be resolved")] 
  IssuerDocumentResolution {
      source: Box<dyn std::error::Error + Send + Sync + 'static >, // Todo: would it be better to use a specific type here? 
  },
  #[error("presentation validation failed: The holder's DID Document could not be resolved")]
  HolderDocumentResolution {
      source: Box<dyn std::error::Error + Send + Sync + 'static>, // Todo: would it be better to use a specific type here? 
  }, 
  #[error("credential validation failed: Could not resolve a subject's DID Document")]
  SubjectDocumentResolution {
      did_url: IotaDIDUrl, // Todo: Should did_url be included in the error message? Would it be better to include additional information in a String? 
      source: Box<dyn std::error::Error + Send + Sync + 'static>, // Todo: would it be better to use a specific type here? 
  }
}

// Todo: Should the DocumentResolution variants in Error be moved to their own enum? 
// If so would then AccumulatedError have an additional field? 

#[derive(Debug)]
/// An error that can occur during more complex credential operations such as presentiation_resolution and credential_resolution. 
pub struct AccumulatedError {
    pub description: String, 
    pub error_conditions: OneOrMany<Error>, 
}

impl Display for AccumulatedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Todo: Refactor the following imperative code once https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.intersperse
        // becomes stable.
        let mut detailed_information = String::new();
        let separator = ",";
        let separator_len = separator.len();
        for validation_error in self.error_conditions.iter() {
          let error_string = validation_error.to_string();
          detailed_information.reserve(error_string.len() + separator_len);
          detailed_information.push_str(&error_string);
          detailed_information.push_str(separator);
        }
        write!(
          f,
          "{} ",
          detailed_information
        )
    }
}

impl std::error::Error for AccumulatedError {}
