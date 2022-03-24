// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]
#![allow(deprecated)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::unused_unit)]

#[macro_use]
extern crate serde;

use wasm_bindgen::prelude::*;

#[macro_use]
mod macros;

#[macro_use]
pub mod error;

pub mod account;
pub mod chain;
pub mod common;
pub mod core;
pub mod credential;
pub mod crypto;
pub mod did;
pub mod tangle;

/// Initializes the console error panic hook for better error messages
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
  console_error_panic_hook::set_once();

  Ok(())
}
