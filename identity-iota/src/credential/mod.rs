// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub mod errors;
pub(crate) mod resolution;
mod resolved_credential;
mod resolved_presentation;
mod validation_options;

pub use self::resolved_credential::ResolvedCredential;
pub use self::resolved_presentation::ResolvedPresentation;
pub use self::validation_options::CredentialValidationOptions;
pub use self::validation_options::PresentationValidationOptions;
