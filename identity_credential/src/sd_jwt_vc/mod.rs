// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod claims;
mod error;
mod status;
mod token;
mod presentation;
/// Additional metadata defined by the SD-JWT VC specification
/// such as issuer's metadata and credential type metadata.
pub mod metadata;
mod resolver;

pub use claims::*;
pub use resolver::*;
pub use presentation::*;
pub use error::Error;
pub use error::Result;
pub use status::*;
pub use token::*;
