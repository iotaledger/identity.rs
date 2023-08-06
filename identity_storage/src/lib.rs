// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]
#![doc = include_str!("./../README.md")]
#![warn(
  rust_2018_idioms,
  unreachable_pub,
  missing_docs,
  rustdoc::missing_crate_level_docs,
  rustdoc::broken_intra_doc_links,
  rustdoc::private_intra_doc_links,
  rustdoc::private_doc_tests,
  clippy::missing_safety_doc
)]

pub mod key_id_storage;
pub mod key_storage;
/// Wrapper around `SecretManager` that implements the storage interfaces.
pub mod secret_manager_wrapper;
pub mod storage;
#[cfg(test)]
mod test_utils;

pub use key_id_storage::*;
pub use key_storage::*;
pub use secret_manager_wrapper::*;
pub use storage::*;
