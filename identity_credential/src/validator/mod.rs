// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Verifiable Credential and Presentation validators.

pub use self::credential_validator::CredentialValidator;
pub use self::domain_linkage_validator::DomainLinkageValidator;
pub use self::errors::CompoundCredentialValidationError;
pub use self::errors::CompoundPresentationValidationError;
pub use self::errors::DomainLinkageValidationError;
pub use self::errors::SignerContext;
pub use self::errors::ValidationError;
pub use self::presentation_validator::PresentationValidator;
pub use self::validation_options::CredentialValidationOptions;
pub use self::validation_options::FailFast;
pub use self::validation_options::PresentationValidationOptions;
pub use self::validation_options::StatusCheck;
pub use self::validation_options::SubjectHolderRelationship;

mod credential_validator;
mod domain_linkage_validator;
mod errors;
mod presentation_validator;
#[cfg(test)]
mod test_utils;
mod validation_options;

// Currently conflicting names with the old validator/validation options
// so we do not re-export the items in vc_jwt_validation for now.
pub mod vc_jwt_validation;
