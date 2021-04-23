// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The `verification` module contains code for verifying the correctness of core DID-related types.
//!
//! This crate DOES NOT verify IOTA-specific invariants, those are exposed by the `identity-iota`
//! crate.

mod builder;
mod method;
mod method_data;
mod method_query;
mod method_ref;
mod method_scope;
mod method_type;

pub use self::builder::MethodBuilder;
pub use self::method::Method;
pub use self::method_data::MethodData;
pub use self::method_query::MethodQuery;
pub use self::method_ref::MethodRef;
pub use self::method_scope::MethodScope;
pub use self::method_type::MethodType;
