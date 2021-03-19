// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! This module implements a `Diff` trait type.  The Diff trait gives data structures an ability to compare
//! themselves to another data structure of the same type over time.  The library pairs off with `identity_derive` which
//! implements a derive macro for the `Diff` Trait. Types supported include `HashMap`, `Option`, `String`,
//! `serde_json::Value`, `Vec` and primitives such as `i8`/`u8` up to `usize` and `isize` as well as the unit type `()`,
//! `bool`, and `char` types.  Structs and Enums are supported via `identity_derive` and can be composed of any number
//! of these types.

#![allow(renamed_and_removed_lints)]
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
