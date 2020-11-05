pub mod client;
pub mod did;
pub mod error;
pub mod helpers;
pub mod network;
pub mod utils;
pub mod vc;

// Re-export the `identity_core` crate as `core`
pub use identity_core as core;
