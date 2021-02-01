// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Types and traits for working with Verifiable Credentials/Presentations.

#![warn(
  missing_docs,
  missing_crate_level_docs,
  broken_intra_doc_links,
  private_intra_doc_links,
  private_doc_tests,
  clippy::missing_safety_doc,
  // clippy::missing_errors_doc
)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde;

pub mod credential;
pub mod error;
pub mod presentation;

pub use self::error::Error;
pub use self::error::Result;
