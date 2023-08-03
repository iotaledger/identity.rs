// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Verifiable Credential and Presentation validators.

pub use self::options::FailFast;
pub use self::options::StatusCheck;
pub use self::options::SubjectHolderRelationship;
pub use self::vc_jwt_validation::*;
pub use self::vp_jwt_validation::*;

mod options;
#[cfg(test)]
pub(crate) mod test_utils;
mod vc_jwt_validation;
mod vp_jwt_validation;
