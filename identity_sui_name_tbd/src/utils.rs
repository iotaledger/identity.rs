// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::IotaClient;
use iota_sdk::IotaClientBuilder;

use crate::Error;

pub const LOCAL_NETWORK: &str = "http://127.0.0.1:9000";

pub async fn get_client(network: &str) -> Result<IotaClient, Error> {
  let client = IotaClientBuilder::default()
    .build(network)
    .await
    .map_err(|err| Error::Network(format!("failed to connect to {network}"), err))?;

  Ok(client)
}
