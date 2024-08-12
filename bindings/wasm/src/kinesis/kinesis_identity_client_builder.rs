// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::iota::client_dummy::IdentityClientBuilder;
use identity_iota::iota::client_dummy::ObjectID;
use wasm_bindgen::prelude::*;

use super::WasmKinesisClient;
use super::WasmKinesisIdentityClient;
use crate::error::wasm_error;
use crate::error::Result;

#[derive(Default)]
#[wasm_bindgen(js_name = KinesisIdentityClientBuilder)]
pub struct WasmKinesisIdentityClientBuilder(pub(crate) IdentityClientBuilder<WasmKinesisClient>);

#[wasm_bindgen(js_class = KinesisIdentityClientBuilder)]
impl WasmKinesisIdentityClientBuilder {
  pub fn identity_iota_package_id(self, value: ObjectID) -> Self {
    Self(self.0.identity_iota_package_id(value))
  }

  pub fn sender_public_key(self, value: &[u8]) -> Self {
    Self(self.0.sender_public_key(value))
  }

  pub fn iota_client(self, value: WasmKinesisClient) -> Self {
    Self(self.0.iota_client(value))
  }

  pub fn network_name(self, value: &str) -> Self {
    Self(self.0.network_name(value))
  }

  pub fn build(self) -> Result<WasmKinesisIdentityClient> {
    Ok(WasmKinesisIdentityClient(self.0.build().map_err(wasm_error)?))
  }
}
