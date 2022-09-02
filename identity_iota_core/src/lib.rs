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
pub use did::StardustDID;
pub use did::StardustDIDUrl;
pub use document::*;
pub use network::NetworkName;
pub(crate) use state_metadata::*;

pub use self::error::Error;
pub use self::error::Result;

#[cfg(feature = "client")]
mod client;
mod did;
mod document;
mod error;
mod network;
mod state_metadata;
