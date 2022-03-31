// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The `verification` module contains code for verifying the correctness of core DID-related types.
//!
//! This crate DOES NOT verify IOTA-specific invariants, those are exposed by the `identity-iota`
//! crate.

mod builder;
mod method_data;
mod method_ref;
mod method_relationship;
mod method_scope;
mod method_type;
mod traits;
mod verification_method;

pub use self::builder::MethodBuilder;
pub use self::method_data::MethodData;
pub use self::method_ref::MethodRef;
pub use self::method_relationship::MethodRelationship;
pub use self::method_scope::MethodScope;
pub use self::method_type::MethodType;
pub use self::traits::MethodUriType;
pub use self::traits::TryMethod;
pub use self::verification_method::VerificationMethod;
