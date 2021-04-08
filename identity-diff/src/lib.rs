// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(docsrs, feature(doc_cfg, extended_key_value_attributes))]
#![cfg_attr(docsrs, cfg_attr(docsrs, doc = include_str!("../README.md")))]
#![cfg_attr(not(docsrs), doc = "")]
#![allow(clippy::upper_case_acronyms)]
#![warn(
  rust_2018_idioms,
  unreachable_pub,
  // missing_docs,
  missing_crate_level_docs,
  broken_intra_doc_links,
  private_intra_doc_links,
  private_doc_tests,
  clippy::missing_safety_doc,
  // clippy::missing_errors_doc,
)]

#[doc(hidden)]
pub use identity_derive::*;

mod error;
mod hashmap;
mod hashset;
mod macros;
mod object;
mod option;
mod string;
mod traits;
mod url;
mod value;
mod vec;

pub use self::error::Error;
pub use self::error::Result;
pub use self::hashmap::DiffHashMap;
pub use self::hashset::DiffHashSet;
pub use self::object::DiffObject;
pub use self::option::DiffOption;
pub use self::string::DiffString;
pub use self::traits::Diff;
pub use self::vec::DiffVec;
