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

pub use did::IotaDID;
#[cfg(feature = "iota-client")]
pub use did_resolution::DidResolutionHandler;
pub use document::*;
pub use network::NetworkName;
pub use state_metadata::*;

pub use self::error::Error;
pub use self::error::Result;

mod did;
mod document;
mod error;
mod network;
mod state_metadata;

#[cfg(feature = "iota-client")]
mod did_resolution;
#[cfg(feature = "iota-client")]
mod iota_interaction_adapter;
#[cfg(all(feature = "iota-client", not(target_arch = "wasm32")))]
/// IOTA Rust SDK based implementation of the identity_iota_interaction interface for non wasm targets.
mod iota_interaction_rust;
#[cfg(feature = "iota-client")]
/// Contains the rebased Identity and the interaction with the IOTA Client.
pub mod rebased;
