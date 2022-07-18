// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]
#![allow(clippy::upper_case_acronyms)]

pub use self::error::Error;
pub use self::error::Result;

pub use stardust_document::StardustDocument;
pub(crate) use state_metadata_document::*;

mod error;
mod stardust_document;
mod state_metadata_document;
