// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Implementation of [Domain Linkage](https://identity.foundation/.well-known/resources/did-configuration/).

mod domain_linkage_configuration;
mod domain_linkage_credential_builder;
mod domain_linkage_validator;
mod error;

pub use self::domain_linkage_configuration::*;
pub use self::domain_linkage_credential_builder::*;
pub use self::domain_linkage_validator::*;
pub use error::*;
