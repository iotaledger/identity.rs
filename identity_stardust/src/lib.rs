// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]
#![allow(clippy::upper_case_acronyms)]

pub use did::StardustDID;
pub use did::StardustDIDUrl;
pub use document::*;
pub use network::NetworkName;
pub use state_metadata::*;

pub use self::error::Error;
pub use self::error::Result;

mod client;
mod did;
mod document;
mod error;
mod network;
mod state_metadata;
