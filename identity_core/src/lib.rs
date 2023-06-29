// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]
#![allow(deprecated)]
#![doc = include_str!("./../README.md")]
#![allow(clippy::upper_case_acronyms)]
#![warn(
  rust_2018_idioms,
  unreachable_pub,
  missing_docs,
  rustdoc::missing_crate_level_docs,
  rustdoc::broken_intra_doc_links,
  rustdoc::private_intra_doc_links,
  rustdoc::private_doc_tests,
  clippy::missing_safety_doc,
  // clippy::missing_errors_doc
)]

#[doc(inline)]
pub use serde_json::json;

#[cfg(feature = "diff")]
#[deprecated(since = "0.5.0", note = "diff chain features are slated for removal")]
#[doc(inline)]
pub use identity_diff as diff;

pub mod common;
pub mod convert;
pub mod error;
pub mod utils;

pub use self::error::Error;
pub use self::error::Result;
