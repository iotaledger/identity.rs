#[macro_use]
extern crate anyhow;

#[macro_use]
extern crate serde;

pub mod common;
pub mod credential;
pub mod presentation;
pub mod verifiable;

pub const RESERVED_PROPERTIES: &[&str] = &["issued", "validFrom", "validUntil"];

pub mod prelude {
  pub use crate::{
    common::{Context, Issuer, Number, Object, OneOrMany, Value, URI},
    credential::{Credential, CredentialBuilder},
    presentation::Presentation,
    verifiable::{VerifiableCredential, VerifiablePresentation},
  };
}
