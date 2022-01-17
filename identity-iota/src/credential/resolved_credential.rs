// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;

use identity_core::common::Timestamp;
use identity_credential::credential::Credential;
use identity_did::verifiable::VerifierOptions;

use crate::document::ResolvedIotaDocument;

use delegate::delegate;

/// A verifiable credential whose associated DID documents have been resolved from the Tangle. 
pub struct ResolvedCredential {
    pub credential: Credential, 
    pub issuer: ResolvedIotaDocument, 
    pub subjects: BTreeMap<String, ResolvedIotaDocument>, 
}

impl ResolvedCredential {
    /// Verify the signature using the issuer's DID document.
    pub fn verify_signature(&self, options: VerifierOptions) -> Result<(), impl std::error::Error> { // Todo: Return a specific error type here
        self.issuer.document.verify_data(&self.credential, options)
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
}

