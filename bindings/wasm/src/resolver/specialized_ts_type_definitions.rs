// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "Promise<Array<CoreDocument | IAsCoreDocument>>")]
  pub type PromiseArrayIAsCoreDocument;

  #[wasm_bindgen(typescript_type = "Promise<CoreDocument | IAsCoreDocument>")]
  pub type PromiseIAsCoreDocument;

  #[wasm_bindgen(typescript_type = "CoreDocument | IAsCoreDocument | undefined")]
  pub type OptionIAsCoreDocument;

  #[wasm_bindgen(typescript_type = "Array<CoreDocument | IAsCoreDocument> | undefined")]
  pub type OptionArrayIAsCoreDocument;
}
