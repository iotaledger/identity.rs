// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::module_inception)]

mod credential;
mod credential_validator;
mod presentation;
mod presentation_validator;
mod types;
mod validation_options;

pub use self::credential::WasmCredential;
pub use self::credential_validator::WasmCredentialValidator;
pub use self::presentation::WasmPresentation;
pub use self::presentation_validator::WasmPresentationValidator;
pub use self::types::*;
pub use self::validation_options::WasmCredentialValidationOptions;
pub use self::validation_options::WasmFailFast;
pub use self::validation_options::WasmPresentationValidationOptions;
pub use self::validation_options::WasmSubjectHolderRelationship;
