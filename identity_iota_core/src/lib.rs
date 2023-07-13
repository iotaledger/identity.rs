// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

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
#![allow(clippy::upper_case_acronyms)]

#[cfg(feature = "iota-client")]
pub use iota_sdk::types::block;

// Re-export the `iota_types::block` module for implementer convenience.
#[cfg(all(feature = "client", not(feature = "iota-client")))]
pub use iota_sdk::types::block;

#[cfg(feature = "client")]
pub use client::*;
pub use did::IotaDID;
pub use document::*;
pub use network::NetworkName;
pub use state_metadata::*;

pub use self::error::Error;
pub use self::error::Result;

#[cfg(feature = "client")]
mod client;
mod did;
mod document;
mod error;
mod network;
mod state_metadata;
