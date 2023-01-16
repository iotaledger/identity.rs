// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The `verification` module contains code for verifying the correctness of core DID-related types.
//!
//! This crate DOES NOT verify IOTA-specific invariants, those are exposed by the
//! `identity_iota_core_legacy` crate.

mod builder;
mod material;
mod method;
mod method_ref;
mod method_relationship;
mod method_scope;
mod method_type;
#[deprecated(since = "0.5.0", note = "diff chain features are slated for removal")]
pub mod diff; 
mod traits;

pub use self::builder::MethodBuilder;
pub use self::material::MethodData;
pub use self::method::VerificationMethod;
pub use self::method_ref::MethodRef;
pub use self::method_relationship::MethodRelationship;
pub use self::method_scope::MethodScope;
pub use self::method_type::MethodType;
pub use self::traits::MethodUriType;
pub use self::traits::TryMethod;
