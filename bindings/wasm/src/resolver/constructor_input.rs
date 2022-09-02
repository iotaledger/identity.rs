// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "ResolutionHandlers | undefined")]
  pub type OptionMapResolutionHandler;

  #[wasm_bindgen(typescript_type = "StardustIdentityClientExt | undefined")]
  pub type OptionWasmStardustIdentityClientExt;
}

// Workaround because JSDocs does not support arrows (=>) while TS does not support the "function" word in type
// definitions (which would be accepted by JSDocs).
#[wasm_bindgen(typescript_custom_section)]
const HANDLERS: &'static str =
  "export type ResolutionHandlers = Map<string, (did: string) => Promise<StardustDocument | CoreDocument>>;";
