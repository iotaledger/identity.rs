// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::upper_case_acronyms)]

#[macro_use]
extern crate serde;

use wasm_bindgen::prelude::*;

#[macro_use]
mod macros;

#[macro_use]
pub mod error;

pub mod chain;
pub mod common;
pub mod credential;
pub mod crypto;
pub mod did;
pub mod message;
pub mod service;
pub mod tangle;

/// Initializes the console error panic hook for better error messages
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
  console_error_panic_hook::set_once();

  Ok(())
}
