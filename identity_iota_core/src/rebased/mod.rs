// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// Module for handling assets.
pub mod assets;
/// Module for handling client operations.
pub mod client;
mod error;
mod iota;
/// Module for handling migration operations.
pub mod migration;
/// Contains the operations of proposals.
pub mod proposals;
/// Module for handling transactions.
pub mod transaction;
/// Contains utility functions.
pub mod utils;

/// Integration with IOTA's Keytool.
#[cfg(feature = "keytool")]
mod keytool_signer;

#[cfg(feature = "keytool")]
pub use keytool_signer::*;

pub use assets::*;
pub use error::*;
