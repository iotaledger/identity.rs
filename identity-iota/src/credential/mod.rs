// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod credential_validator;
pub mod errors;
mod presentation_validator;
mod resolved_credential;
mod resolved_presentation;
#[cfg(test)]
mod test_utils;
mod validation_options;

pub use self::credential_validator::CredentialValidator;
pub use self::presentation_validator::PresentationValidator;
pub use self::resolved_credential::ResolvedCredential;
pub use self::resolved_presentation::ResolvedPresentation;
pub use self::validation_options::CredentialValidationOptions;
pub use self::validation_options::PresentationValidationOptions;
