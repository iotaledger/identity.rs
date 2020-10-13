pub mod did;
pub mod error;
pub mod helpers;
pub mod io;
pub mod network;
pub mod resolver;
pub mod types;
pub mod utils;

// Re-export the `identity_core` crate as `core`
pub use identity_core as core;
