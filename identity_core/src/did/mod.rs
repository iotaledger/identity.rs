mod authentication;
#[allow(clippy::module_inception)]
mod did;
mod did_diff;
mod document;
mod parser;
mod service;

pub use authentication::*;
pub use did::*;
pub use document::*;
pub use service::*;
