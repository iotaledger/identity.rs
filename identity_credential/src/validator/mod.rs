// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Verifiable Credential and Presentation validators.

pub use self::credential_validator::CredentialValidator;
pub use self::errors::CompoundCredentialValidationError;
pub use self::errors::CompoundPresentationValidationError;
pub use self::errors::SignerContext;
pub use self::errors::ValidationError;
pub use self::presentation_validator::PresentationValidator;
pub use self::validation_options::CredentialValidationOptions;
pub use self::validation_options::FailFast;
pub use self::validation_options::PresentationValidationOptions;
pub use self::validation_options::StatusCheck;
pub use self::validation_options::SubjectHolderRelationship;
pub use vp_jwt_validation::*;

mod credential_validator;
mod errors;
mod presentation_validator;
#[cfg(test)]
pub(crate) mod test_utils;
mod validation_options;
mod vp_jwt_validation;

// Currently conflicting names with the old validator/validation options
// so we do not re-export the items in vc_jwt_validation for now.
pub mod vc_jwt_validation;
