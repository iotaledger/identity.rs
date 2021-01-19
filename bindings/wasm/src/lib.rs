// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use wasm_bindgen::prelude::*;

pub mod did;
pub mod doc;
pub mod iota;
pub mod key;
pub mod pubkey;
pub mod vc;
pub mod vp;

/// Initializes the console error panic hook for better error messages
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    Ok(())
}

/// Convert errors so they are readable in JS
pub fn js_err<T: ToString>(error: T) -> JsValue {
    error.to_string().into()
}
