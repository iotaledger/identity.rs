// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]
#![allow(clippy::upper_case_acronyms)]

pub use self::error::Error;
pub use self::error::Result;

pub use did_or_placeholder::DidOrPlaceholder;
pub use stardust_document::StardustDocument;
pub use state_metadata_document::StateMetadataDocument;

mod did_or_placeholder;
mod error;
mod stardust_document;
mod state_metadata_document;
