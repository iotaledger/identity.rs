use identity_core::common::Timestamp;

// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub struct CredentialValidationOptions {
    expires_after: Timestamp, 
    issued_before: Timestamp, 
    allow_deactivated_subject_documents: bool, 
}

impl Default for CredentialValidationOptions {
    fn default() -> Self {
        Self {
            expires_after: Timestamp::now_utc(), 
            issued_before: Timestamp::now_utc(), 
            allow_deactivated_subject_documents: false, 
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

}