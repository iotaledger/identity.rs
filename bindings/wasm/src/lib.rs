// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]
#![allow(deprecated)]
#![allow(clippy::upper_case_acronyms)]
// wasm_bindgen calls drop on non-Drop types. When/If this is fixed, this can be removed (no issue to link here yet).
#![allow(clippy::drop_non_drop)]
#![allow(clippy::unused_unit)]
#![allow(clippy::await_holding_refcell_ref)] // complains about RefCell<Account> in future_to_promise, should only panic in multithreaded code/web workers

#[macro_use]
extern crate serde;

use wasm_bindgen::prelude::*;

#[macro_use]
mod macros;

// Deactivated legacy packages.
// pub mod account;

pub mod wasm_signable;
// pub mod wasm_method_suite;
pub mod common;
pub mod core_document_rc;
pub mod credential;
pub mod crypto;
pub mod did;
pub mod error;
pub mod iota;
pub mod key_alias;
pub mod key_storage;
pub mod resolver;
pub mod revocation;
pub mod storage;
pub mod util;

/// Initializes the console error panic hook for better error messages
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
  console_error_panic_hook::set_once();

  Ok(())
}
