// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;

use identity_core::common::{Timestamp, OneOrMany};
use identity_credential::credential::Credential;
use identity_did::{verifiable::VerifierOptions, did::DID};

use crate::{document::ResolvedIotaDocument, did::IotaDIDUrl, tangle::TangleRef};

use delegate::delegate;

/// A verifiable credential whose associated DID documents have been resolved from the Tangle. 
pub struct ResolvedCredential {
    pub credential: Credential, 
    pub issuer: ResolvedIotaDocument, 
    pub subjects: BTreeMap<String, ResolvedIotaDocument>, 
}

impl ResolvedCredential {
    /// Verify the signature using the issuer's DID document.
    pub fn verify_signature(&self, options: VerifierOptions) -> Result<(), ValidationUnitError> { 
        self.issuer.document.verify_data(&self.credential, options).map_err(|err| 
            ValidationUnitError::InvalidProof {source: Box::new(err)})
    }

    /// Returns an iterator over the resolved documents that have been deactivated
    pub fn deactivated_subject_documents(&self) -> impl Iterator<Item= &ResolvedIotaDocument> + '_ {
        self.subjects.iter().map(|(_, doc)| doc)
        .filter(|resolved_doc| resolved_doc.document.methods().next().is_none())
    } 
    delegate! {
        to self.credential {
            /// Checks whether this Credential expires after the given `Timestamp`.
            /// True is returned in the case of no expiration date.  
            pub fn expires_after(&self, timestamp: Timestamp) -> bool;

            /// Checks whether the issuance date of this Credential is before the given `Timestamp`.
            pub fn issued_before(&self, timestamp: Timestamp) -> bool;

            /// Checks whether this Credential's types match the input. 
            pub fn matches_types(&self, other: &[&str]) -> bool;

            /// Returns an iterator of the `types` of this Credential that are not in `input_types`. 
            pub fn types_difference_left<'a>(&'a self, input_types: &'a [&str]) -> impl Iterator<Item = &String> + 'a; 

            /// Returns an iterator of `types` that are in `input_types`, but not in this Credential. 
            pub fn types_difference_right<'a>(&'a self, input_types: &'a [&str]) -> impl Iterator<Item= &str> + 'a;
        }
    }

    pub fn try_expires_after(&self, timestamp: Timestamp) -> Result<(), ValidationUnitError> {
        self.expires_after(timestamp).then(||()).ok_or(ValidationUnitError::InvalidExpirationDate)
    }

    pub fn try_issued_before(&self, timestamp: Timestamp) -> Result<(), ValidationUnitError> {
        self.issued_before(timestamp).then(||()).ok_or(ValidationUnitError::InvalidIssuanceDate)
    }

    pub fn try_without_deactivated_subject_documents(&self, fail_fast: bool) -> Result<(), OneOrMany<ValidationUnitError>> {
        let mut iter = self.deactivated_subject_documents().peekable(); 

        if iter.peek().is_none() {
            Ok(())
        } else if fail_fast {
            let error: OneOrMany<ValidationUnitError> = iter.take(1)
            .map(|deactivated_doc| deactivated_doc.did().to_url())
            .map(|url| ValidationUnitError::Deactivated{did_url: url}).collect(); 
            Err(error)
        } else {
            let errors: OneOrMany<ValidationUnitError> = iter
            .map(|deactivated_doc| deactivated_doc.did().to_url())
            .map(|url| ValidationUnitError::Deactivated{did_url: url}).collect(); 
            Err(errors)
        }
    }

}

#[non_exhaustive]
#[derive(Debug)]
pub enum ValidationUnitError {
    /// Indicates that the expiration date of the credential is not considered valid.
    InvalidExpirationDate,
    /// Indicates that the issuance date of the credential is not considered valid.
    InvalidIssuanceDate,
    /// The DID document corresponding to `did` has been deactivated.
    Deactivated {
        did_url: IotaDIDUrl, 
    },
    /// The proof verification failed. 
    InvalidProof {
        source: Box<dyn std::error::Error> // Todo: Put an actual error type here 
    }
}

// Todo: Create tests for verify_signature and deactivated_subject_documents