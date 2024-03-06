// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The core types used to create Verifiable Credentials.

#![allow(clippy::module_inception)]

pub mod common;
mod jws;
mod jwt;
mod jwt_serialization;
mod any_credential;
#[cfg(feature = "sd-jwt")]
pub mod sd_jwt;
mod traits;
pub mod vc1_1;
pub mod vc2_0;

pub use self::jws::Jws;
pub use common::Evidence;
pub use common::Issuer;
pub use common::LinkedDomainService;
pub use common::Policy;
pub use common::Proof;
pub use common::RefreshService;
pub use common::Schema;
pub use common::Subject;
pub use jwt::*;
pub use traits::*;
pub use vc1_1::Credential;
pub use vc1_1::CredentialBuilder;
pub use vc1_1::Status;
pub use any_credential::*;

#[cfg(feature = "validator")]
pub(crate) use self::jwt_serialization::CredentialJwtClaims;
#[cfg(feature = "presentation")]
pub(crate) use self::jwt_serialization::IssuanceDateClaims;