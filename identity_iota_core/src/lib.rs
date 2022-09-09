// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]
#![allow(clippy::upper_case_acronyms)]

// Re-export the `bee_block` crate for implementer convenience.
#[cfg(all(feature = "client", not(feature = "iota-client")))]
pub use bee_block as block;
#[cfg(feature = "iota-client")]
pub use iota_client::block;

#[cfg(feature = "client")]
pub use client::*;
pub use did::IotaDID;
pub use did::IotaDIDUrl;
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
