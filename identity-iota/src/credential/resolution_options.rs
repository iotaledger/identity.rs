// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::OneOrMany;

use crate::did::IotaDID;

use super::CredentialValidationOptions;

pub enum SubjectDocumentResolutionConfig {
    /// Indicates that all credential subjects should have their DID Documents resolved 
    Always, 
    /// Indicates that only credential subjects identifiable among the specified IotaDID's should have their DID Documents resolved  
    OnSpecified(OneOrMany<IotaDID>), 
    /// Indicates that the DID Documents of a credential subject should never be resolved 
    Never, 
}
pub struct CredentialResolutionOptions {
    validation_options: CredentialValidationOptions, 
    subject_document_resolution_config: SubjectDocumentResolutionConfig, 
}

pub struct PresentationResolutionOptions {
    common_credential_options: CredentialResolutionOptions, 
}

impl Default for SubjectDocumentResolutionConfig {
    fn default() -> Self {
        Self::Always
    }
}

impl Default for CredentialResolutionOptions {
    fn default() -> Self {
        Self {
            validation_options: CredentialValidationOptions::default(), 
            subject_document_resolution_config: SubjectDocumentResolutionConfig::default(),
        }
    }
}

impl Default for PresentationResolutionOptions {
    fn default() -> Self {
        Self {
            common_credential_options: CredentialResolutionOptions::default()
        }
    }
}

