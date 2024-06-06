#[cfg(feature = "web")]
pub use did_web::*;
pub use self::errors::Error;
pub use self::errors::Result;

#[cfg(feature = "key")]
mod did_key;
#[cfg(feature = "web")]
mod did_web;
mod errors;