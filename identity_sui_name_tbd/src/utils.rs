// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use sui_sdk::SuiClient;
use sui_sdk::SuiClientBuilder;

use crate::Error;

pub async fn get_client(network: &str) -> Result<SuiClient, Error> {
  let client = SuiClientBuilder::default()
    .build(network)
    .await
    .map_err(|err| Error::Network(format!("failed to connect to {network}"), err))?;

  Ok(client)
}
