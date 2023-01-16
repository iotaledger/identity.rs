// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]
#![allow(deprecated)]
#![doc = include_str!("./../README.md")]
#![allow(clippy::upper_case_acronyms)]
#![warn(
  rust_2018_idioms,
  unreachable_pub,
  // missing_docs,
  rustdoc::missing_crate_level_docs,
  rustdoc::broken_intra_doc_links,
  rustdoc::private_intra_doc_links,
  rustdoc::private_doc_tests,
  clippy::missing_safety_doc,
  // clippy::missing_errors_doc
)]

#[macro_use]
extern crate serde;

mod error;
pub mod verification_method;
pub use error::Error;
pub use error::Result;
