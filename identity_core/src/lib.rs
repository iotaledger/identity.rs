#[macro_use]
pub mod common;

pub mod did;
pub mod error;
pub mod key;
pub mod resolver;
pub mod utils;
pub mod vc;

// Re-export the `identity_diff` crate as `diff`
pub use identity_diff as diff;

pub use error::{Error, Result};
