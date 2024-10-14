// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use wasm_bindgen::prelude::*;

use identity_iota::iota::iota_sdk_abstraction::IdentityClientBuilder;
use identity_iota::iota::iota_sdk_abstraction::types::base_types::ObjectID;

use crate::error::wasm_error;
use crate::error::Result;

use super::ts_client_sdk::{IotaClientTsSdk, WasmIotaClient};
use super::WasmKinesisIdentityClient;
use super::types::WasmObjectID;

#[derive(Default)]
#[wasm_bindgen(js_name = KinesisIdentityClientBuilder)]
pub struct WasmKinesisIdentityClientBuilder(pub(crate) IdentityClientBuilder<IotaClientTsSdk>);

#[wasm_bindgen(js_class = KinesisIdentityClientBuilder)]
impl WasmKinesisIdentityClientBuilder {
  #[wasm_bindgen(js_name = identityIotaPackageId)]
  pub fn identity_iota_package_id(self, value: WasmObjectID) -> Self {
    Self(self.0.identity_iota_package_id(value.parse().expect("failed to parse value into ObjectID")))
  }

  #[wasm_bindgen(js_name = senderPublicKey)]
  pub fn sender_public_key(self, value: &[u8]) -> Self {
    Self(self.0.sender_public_key(value))
  }

  #[wasm_bindgen(js_name = iotaClient)]
  pub fn iota_client(self, value: WasmIotaClient) -> Self {
    Self(self.0.iota_client(IotaClientTsSdk::new(value).expect("IotaClientTsSdk could not be initialized")))
  }

  #[wasm_bindgen(js_name = networkName)]
  pub fn network_name(self, value: &str) -> Self {
    Self(self.0.network_name(value))
  }

  pub fn build(self) -> Result<WasmKinesisIdentityClient> {
    Ok(WasmKinesisIdentityClient(self.0.build().map_err(wasm_error)?))
  }
}
