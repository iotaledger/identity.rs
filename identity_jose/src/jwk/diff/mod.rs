// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
//! Provides a [`Diff`](::identity_diff::Diff) implementation for [`Jwk`](crate::jwk::Jwk).
//!
//! # Warning: This module has not been tested.  
#![allow(deprecated)]
mod key;
mod key_operations;
mod key_params;
mod key_type;
mod key_use;
pub use key::*;
pub use key_operations::*;
pub use key_params::*;
pub use key_type::*;
pub use key_use::*;
