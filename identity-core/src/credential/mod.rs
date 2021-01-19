// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Types and traits for working with Verifiable Credentials.

#![allow(clippy::module_inception)]

mod credential;
mod credential_builder;
mod presentation;
mod presentation_builder;
mod types;
mod verifiable_credential;
mod verifiable_presentation;

pub use credential::*;
pub use credential_builder::*;
pub use presentation::*;
pub use presentation_builder::*;
pub use types::*;
pub use verifiable_credential::*;
pub use verifiable_presentation::*;
