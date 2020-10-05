mod credential;
mod error;
mod presentation;
mod types;
mod validation;
mod verifiable_credential;
mod verifiable_presentation;

pub use credential::*;
pub use error::*;
pub use presentation::*;
pub use types::*;
pub use validation::*;
pub use verifiable_credential::*;
pub use verifiable_presentation::*;

pub const RESERVED_PROPERTIES: &[&str] = &["issued", "validFrom", "validUntil"];
