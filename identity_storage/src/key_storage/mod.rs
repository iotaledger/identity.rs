// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! A Key Storage is used to securely store private keys.
//!
//! This module provides the [`JwkStorage`] trait that
//! abstracts over storages that store JSON Web Keys.

#[cfg(feature = "memstore")]
mod ed25519;
mod jwk_gen_output;
mod jwk_storage;
mod key_id;
mod key_storage_error;
mod key_type;
#[cfg(feature = "memstore")]
mod memstore;

#[cfg(test)]
pub(crate) mod tests;

pub use jwk_gen_output::*;
pub use jwk_storage::*;
pub use key_id::*;
pub use key_storage_error::*;
pub use key_type::*;
#[cfg(feature = "memstore")]
pub use memstore::*;
