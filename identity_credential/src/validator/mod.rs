// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Verifiable Credential and Presentation validators.

pub use self::jwt_credential_validation::*;
pub use self::jwt_presentation_validation::*;
pub use self::options::FailFast;
pub use self::options::StatusCheck;
pub use self::options::SubjectHolderRelationship;

mod jwt_credential_validation;
mod jwt_presentation_validation;
mod options;
#[cfg(test)]
pub(crate) mod test_utils;
