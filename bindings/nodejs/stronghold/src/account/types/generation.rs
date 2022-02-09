// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account::Generation;

#[napi(js_name = Generation)]
pub struct JsGeneration(pub(crate) Generation);

impl From<Generation> for JsGeneration {
  fn from(generation: Generation) -> Self {
    JsGeneration(generation)
  }
}
