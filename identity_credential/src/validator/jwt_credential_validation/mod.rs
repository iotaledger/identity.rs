// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
/*
 * Modifications Copyright 2024 Fondazione LINKS.
 */

//! Contains functionality for validating credentials issued as JWTs.
mod decoded_jwt_credential;
mod error;
mod jwt_credential_validation_options;
mod jwt_credential_validator;
#[cfg(feature = "hybrid")]
mod jwt_credential_validator_hybrid;
mod jwt_credential_validator_utils;

pub use decoded_jwt_credential::*;
pub use error::*;
pub use jwt_credential_validation_options::*;
pub use jwt_credential_validator::*;
#[cfg(feature = "hybrid")]
pub use jwt_credential_validator_hybrid::*;
pub use jwt_credential_validator_utils::*;
