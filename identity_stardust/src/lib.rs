// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]
#![allow(clippy::upper_case_acronyms)]

#[cfg(feature = "iota-client")]
pub use client::*;
pub use did::StardustDID;
pub use did::StardustDIDUrl;
pub use document::*;
pub use network::NetworkName;
pub use state_metadata::*;

pub use self::error::Error;
pub use self::error::Result;
pub use resolution::*;

#[cfg(feature = "iota-client")]
mod client;
mod did;
mod document;
mod error;
mod network;
mod resolution;
mod state_metadata;
