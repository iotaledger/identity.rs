// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::upper_case_acronyms)]

#[macro_use]
extern crate serde;

use wasm_bindgen::prelude::*;

#[macro_use]
mod macros;
mod utils;

pub mod credential;
pub mod crypto;
pub mod iota;
pub mod service;
pub mod wasm_did;
pub mod wasm_document;
pub mod wasm_verification_method;

//this will currently not build https://github.com/uuid-rs/uuid/pull/512
//pub mod message;

/// Initializes the console error panic hook for better error messages
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
  console_error_panic_hook::set_once();

  Ok(())
}
