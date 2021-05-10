// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(docsrs, feature(doc_cfg, extended_key_value_attributes))]
#![cfg_attr(docsrs, cfg_attr(docsrs, doc = include_str!("../README.md")))]
#![cfg_attr(not(docsrs), doc = "")]
#![allow(clippy::upper_case_acronyms)]
#![warn(
  rust_2018_idioms,
  // unreachable_pub,
  // missing_docs,
  missing_crate_level_docs,
  broken_intra_doc_links,
  private_intra_doc_links,
  private_doc_tests,
  clippy::missing_safety_doc,
  // clippy::missing_errors_doc,
)]

#[macro_use]
extern crate serde;

pub mod envelope;
pub mod error;
pub mod message;
pub mod types;
