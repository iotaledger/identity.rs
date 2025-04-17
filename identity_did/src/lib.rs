// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/*
 * Modifications Copyright 2024 Fondazione LINKS.
 */

#![forbid(unsafe_code)]
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

#[allow(clippy::module_inception)]
mod did;
mod did_jwk;
mod did_url;
mod error;
mod did_compositejwk;

pub use crate::did_url::DIDUrl;
pub use crate::did_url::RelativeDIDUrl;
pub use ::did_url_parser::DID as BaseDIDUrl;
pub use did::CoreDID;
pub use did::DID;
pub use did_jwk::*;
pub use error::Error;
pub use did_compositejwk::*;