#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde;

pub use did_doc;
pub use did_url;
pub use identity_diff;
pub use serde_json::json;

#[macro_use]
pub mod common;
pub mod convert;
pub mod crypto;
pub mod error;
pub mod proof;
pub mod resolver;
pub mod utils;
pub mod vc;

pub use error::{Error, Result};
