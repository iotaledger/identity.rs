mod authentication;
#[allow(clippy::module_inception)]
mod did;
mod document;
mod parser;
mod proof;
mod service;

pub use authentication::*;
pub use did::*;
pub use document::*;
pub use proof::*;
pub use service::*;
