#[macro_use]
pub mod common;
pub mod did;
pub mod did_parser;
pub mod document;
mod error;
pub mod utils;
pub use iota::client::builder::Network as iota_network;

pub use error::{Error, Result};
