// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Timestamp;
use identity_did::verifiable::VerifierOptions;
pub struct CredentialValidationOptions {
    pub(crate) expires_after: Timestamp, 
    pub(crate) issued_before: Timestamp, 
    pub(crate) allow_deactivated_subject_documents: bool, 
    pub(crate) verifier_options: VerifierOptions, 
}

impl Default for CredentialValidationOptions {
    fn default() -> Self {
        Self {
            expires_after: Timestamp::now_utc(), 
            issued_before: Timestamp::now_utc(), 
            allow_deactivated_subject_documents: false, 
            verifier_options: VerifierOptions::default(), 
        }
    }
}

impl CredentialValidationOptions {
    pub fn with_latest_expiration_date(mut self, timestamp: Timestamp) -> Self {
        self.expires_after = timestamp; 
        self 
    }

    pub fn with_earliest_issuance_date(mut self, timestamp: Timestamp) -> Self {
        self.issued_before = timestamp; 
        self
    }
    
    pub fn allow_deactivated_subject_documents(mut self, value: bool) -> Self {
        self.allow_deactivated_subject_documents = value; 
        self 
    }

    pub fn with_verifier_options(mut self, verifier_options: VerifierOptions) -> Self {
        self.verifier_options = verifier_options;
        self
    }
}

pub struct PresentationValidationOptions {
    pub(crate) common_validation_options: CredentialValidationOptions
}

impl Default for PresentationValidationOptions {
    fn default() -> Self {
        Self {
            common_validation_options: CredentialValidationOptions::default(),
        }
    }
}

impl PresentationValidationOptions {
    pub fn with_common_validation_options(mut self, options: CredentialValidationOptions) -> Self {
        self.common_validation_options = options; 
        self 
    }
}