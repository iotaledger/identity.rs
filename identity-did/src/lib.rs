// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Types and traits for working with Decentralized Identifiers.

#![warn(
  rust_2018_idioms,
  unreachable_pub,
  // missing_docs,
  missing_crate_level_docs,
  broken_intra_doc_links,
  private_intra_doc_links,
  private_doc_tests,
  clippy::missing_safety_doc,
  // clippy::missing_errors_doc
)]

#[macro_use]
extern crate serde;

pub mod did {
  #[doc(import)]
  pub use did_url::*;
}

pub mod document;
pub mod error;
pub mod service;
pub mod signature;
pub mod utils;
pub mod verifiable;
pub mod verification;

pub use self::error::Error;
pub use self::error::Result;
