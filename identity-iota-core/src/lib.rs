// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]
#![allow(deprecated)]
#![cfg_attr(docsrs, feature(doc_cfg, extended_key_value_attributes))]
#![cfg_attr(docsrs, cfg_attr(docsrs, doc = include_str!("../README.md")))]
#![cfg_attr(not(docsrs), doc = "")]
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
  // clippy::missing_errors_doc,
)]

pub use self::error::Error;
pub use self::error::Result;

#[deprecated(since = "0.5.0", note = "diff chain features are slated for removal")]
pub mod diff;

pub mod did;
pub mod document;
pub mod error;
pub mod revocation;
pub mod service;
pub mod tangle;
