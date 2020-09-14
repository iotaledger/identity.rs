mod hashmap;
mod macros;
pub mod option;
mod string;
mod traits;
#[cfg(feature = "serde_value")]
mod value;
mod vec;

pub use traits::Diff;

#[cfg(feature = "diff_derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate identity_derive;

#[cfg(feature = "diff_derive")]
#[doc(hidden)]
pub use identity_derive::*;
