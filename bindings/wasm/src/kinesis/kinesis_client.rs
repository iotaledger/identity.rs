// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::common::PromiseString;
use crate::error::JsValueResult;
use identity_iota::iota::KinesisClientTrait;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

// here we describe what the TYPESCRIPT client can do
// but is this the correct place to do so?
// (hopefully) yes, as we also convert TYPESCRIPT types to the expected RUST types

// make `IotaClient` type available in ts
// TODO: check why this isn't done by `module` macro attribute for `WasmKinesisClient`
#[wasm_bindgen(typescript_custom_section)]
const IOTA_CLIENT_TYPE: &'static str = r#"
  import { IotaClient } from "@iota/iota.js/client";
"#;

#[wasm_bindgen(module = "@iota/iota.js/client")]
extern "C" {
  #[wasm_bindgen(typescript_type = "IotaClient")]
  pub type WasmKinesisClient;

  #[wasm_bindgen(method, js_name = getChainIdentifier)]
  pub fn get_chain_identifier(this: &WasmKinesisClient) -> PromiseString;
}

// convert TYPESCRIPT types to RUST types
#[async_trait::async_trait(?Send)]
impl KinesisClientTrait for WasmKinesisClient {
  type Error = anyhow::Error;

  async fn get_chain_identifier(&self) -> Result<String, Self::Error> {
    let promise: Promise = Promise::resolve(&WasmKinesisClient::get_chain_identifier(self));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }
}
