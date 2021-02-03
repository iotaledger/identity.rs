// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Identity Core

#![warn(
  rust_2018_idioms,
  unreachable_pub,
  missing_docs,
  missing_crate_level_docs,
  broken_intra_doc_links,
  private_intra_doc_links,
  private_doc_tests,
  clippy::missing_safety_doc,
  // clippy::missing_errors_doc
)]

#[macro_use]
extern crate serde;

#[doc(inline)]
pub use serde_json::json;

#[doc(inline)]
pub use identity_diff as diff;

pub mod common;
pub mod convert;
pub mod crypto;
pub mod error;
pub mod utils;

pub use self::error::Error;
pub use self::error::Result;
