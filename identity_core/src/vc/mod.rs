mod credential;
mod error;
mod presentation;
mod types;
mod validation;
mod verifiable_credential;
mod verifiable_presentation;

pub use self::{
    credential::*, error::*, presentation::*, types::*, validation::*, verifiable_credential::*,
    verifiable_presentation::*,
};

pub const RESERVED_PROPERTIES: &[&str] = &["issued", "validFrom", "validUntil"];
