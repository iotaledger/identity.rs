#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde;

pub mod chain;
pub mod client;
pub mod did;
pub mod error;
pub mod tangle;
pub mod utils;
pub mod vc;

/// Re-export `identity_core::crypto`; in the future this will be `crypto.rs`.
pub mod crypto {
    pub use identity_core::crypto::*;
}
