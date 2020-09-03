#[macro_use]
extern crate serde;

#[macro_use]
mod macros;

pub mod common;
pub mod credential;
pub mod error;
pub mod presentation;
pub mod utils;
pub mod verifiable;

pub const RESERVED_PROPERTIES: &[&str] = &["issued", "validFrom", "validUntil"];

pub mod prelude {
    pub use crate::{
        common::{
            Context, CredentialSchema, CredentialStatus, CredentialSubject, Evidence, Issuer, RefreshService,
            TermsOfUse,
        },
        credential::{Credential, CredentialBuilder},
        error::{Error, Result},
        presentation::{Presentation, PresentationBuilder},
        verifiable::{VerifiableCredential, VerifiablePresentation},
    };
}
