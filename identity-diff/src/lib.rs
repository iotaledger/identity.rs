/// This module implements a `Diff` trait type.  The Diff trait gives data structures an ability to compare
/// themselves to another data structure of the same type over time.  The library pairs off with `identity_derive` which
/// implements a derive macro for the `Diff` Trait. Types supported include `HashMap`, `Option`, `String`,
/// `serde_json::Value`, `Vec` and primitives such as `i8`/`u8` up to `usize` and `isize` as well as the unit type `()`,
/// `bool`, and `char` types.  Structs and Enums are supported via `identity_derive` and can be composed of any number
/// of these types.
pub mod did_doc;
mod error;
pub mod hashmap;
pub mod hashset;
mod macros;
pub mod option;
pub mod string;
mod traits;
mod value;
pub mod vec;

pub use error::{Error, Result};
pub use traits::Diff;

#[doc(hidden)]
pub use identity_derive::*;
