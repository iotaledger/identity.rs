// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::iota::KinesisIdentityClientDummy;
use wasm_bindgen::prelude::*;

use super::WasmKinesisClient;

#[wasm_bindgen(js_name = KinesisIdentityClient)]
pub struct WasmKinesisIdentityClient(pub(crate) KinesisIdentityClientDummy<WasmKinesisClient>);

#[wasm_bindgen(js_class = KinesisIdentityClient)]
impl WasmKinesisIdentityClient {
  // create with (new) IotaClient instance
  #[wasm_bindgen(constructor)]
  pub fn new(kinesis_client: WasmKinesisClient) -> Self {
    // call actual new
    Self(KinesisIdentityClientDummy::<WasmKinesisClient>::new(kinesis_client))
  }

  // make test call
  #[wasm_bindgen(js_name = getBalance)]
  pub async fn get_balance(&self) -> Result<String, JsError> {
    KinesisIdentityClientDummy::get_chain_identifier(&self.0)
      .await
      .map_err(|err| JsError::new(&format!("could not get balance; {err}")))
  }
}
