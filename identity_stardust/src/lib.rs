// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]
#![allow(clippy::upper_case_acronyms)]

pub use self::error::Error;
pub use self::error::Result;

pub use stardust_document::StardustDocument;
pub use state_metadata::*;

pub use did::StardustDID;
pub use did::StardustDIDUrl;

mod did;
mod error;
mod stardust_document;
mod state_metadata;
