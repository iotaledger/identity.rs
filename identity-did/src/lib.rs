// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[macro_use]
extern crate serde;

pub mod did {
  #[doc(import)]
  pub use did_url::*;
}

pub mod document;
pub mod error;
pub mod service;
pub mod signature;
pub mod utils;
pub mod verifiable;
pub mod verification;

pub use self::error::Error;
pub use self::error::Result;
