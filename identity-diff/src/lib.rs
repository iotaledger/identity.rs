// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![deprecated(since = "0.5.0", note = "diff chain features are slated for removal")]
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
  // clippy::missing_errors_doc,
)]

#[cfg(feature = "derive")]
#[doc(hidden)]
pub use identity_diff_derive::Diff;

pub use self::error::Error;
pub use self::error::Result;
pub use self::hashmap::DiffHashMap;
pub use self::hashset::DiffHashSet;
pub use self::object::DiffObject;
pub use self::option::DiffOption;
pub use self::string::DiffString;
pub use self::traits::Diff;
pub use self::vec::DiffVec;

mod error;
mod hashmap;
mod hashset;
mod macros;
mod object;
mod option;
mod string;
mod traits;
mod value;
mod vec;
