// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]
#![allow(clippy::upper_case_acronyms)]

pub use self::error::Error;
pub use self::error::Result;

pub use document::*;
pub use state_metadata::*;

pub use did::StardustDID;
// TODO: Uncomment once `document` has been refactored to use the types from the `did` module in this crate.
// pub use did::StardustDIDUrl;
pub use network::NetworkName;
mod did;
mod document;
mod error;
mod network;
mod state_metadata;
