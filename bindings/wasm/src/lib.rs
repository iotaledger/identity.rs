// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
/*
 * Modifications Copyright 2024 Fondazione LINKS.
 */

#![allow(deprecated)]
#![allow(clippy::upper_case_acronyms)]
// wasm_bindgen calls drop on non-Drop types. When/If this is fixed, this can be removed (no issue to link here yet).
#![allow(clippy::drop_non_drop)]
#![allow(clippy::unused_unit)]
// complains about RefCell<Account> in future_to_promise, should only panic in multithreaded code/web workers
#![allow(clippy::await_holding_refcell_ref)]

#[macro_use]
extern crate serde;

use wasm_bindgen::prelude::*;

#[macro_use]
mod macros;

pub mod common;
pub mod credential;
pub mod did;
pub mod error;
pub mod iota;
pub mod jose;
pub mod jpt;
pub mod resolver;
pub mod revocation;
pub mod sd_jwt;
pub mod sd_jwt_vc;
pub mod storage;
pub mod verification;

/// Initializes the console error panic hook for better error messages
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
  console_error_panic_hook::set_once();
  Ok(())
}

// This section should be used as the central place for imports from `./lib`
// used by TS definitions written on the Rust side.
// It appears the import path must be relative to `src`.
#[wasm_bindgen(typescript_custom_section)]
const CUSTOM_IMPORTS: &'static str = r#"
import { JwsAlgorithm, JwkOperation, JwkUse, JwkType, CompositeAlgId} from '../lib/jose/index';
"#;
