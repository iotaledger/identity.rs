// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::iota::client_dummy::IdentityClient;
use wasm_bindgen::prelude::*;

use super::kinesis_identity_client_builder::WasmKinesisIdentityClientBuilder;
use super::WasmKinesisClient;

#[wasm_bindgen(js_name = KinesisIdentityClient)]
pub struct WasmKinesisIdentityClient(pub(crate) IdentityClient<WasmKinesisClient>);

#[wasm_bindgen(js_class = KinesisIdentityClient)]
impl WasmKinesisIdentityClient {
  #[wasm_bindgen]
  pub fn builder() -> WasmKinesisIdentityClientBuilder {
    WasmKinesisIdentityClientBuilder::default()
  }

  // integration test functions

  // create with (new) IotaClient instance
  #[wasm_bindgen(constructor)]
  pub fn new(kinesis_client: WasmKinesisClient) -> Self {
    // call actual new
    Self(IdentityClient::<WasmKinesisClient>::new(kinesis_client))
  }

  // make test call
  #[wasm_bindgen(js_name = getBalance)]
  pub async fn get_balance(&self) -> Result<String, JsError> {
    IdentityClient::get_chain_identifier(&self.0)
      .await
      .map_err(|err| JsError::new(&format!("could not get balance; {err}")))
  }
}
