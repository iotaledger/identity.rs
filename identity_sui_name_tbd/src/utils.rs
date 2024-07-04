// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context;
use sui_sdk::types::base_types::SuiAddress;
use sui_sdk::SuiClient;
use sui_sdk::SuiClientBuilder;
use tokio::process::Command;

use crate::Error;

pub const LOCAL_NETWORK: &str = "http://127.0.0.1:9000";

pub async fn get_client(network: &str) -> Result<SuiClient, Error> {
  let client = SuiClientBuilder::default()
    .build(network)
    .await
    .map_err(|err| Error::Network(format!("failed to connect to {network}"), err))?;

  Ok(client)
}

pub async fn request_funds(address: &SuiAddress) -> anyhow::Result<()> {
  let output = Command::new("sui")
    .arg("client")
    .arg("faucet")
    .arg("--address")
    .arg(address.to_string())
    .arg("--json")
    .output()
    .await
    .context("Failed to execute command")?;

  if !output.status.success() {
    anyhow::bail!(
      "Failed to request funds from faucet: {}",
      std::str::from_utf8(&output.stderr).unwrap()
    );
  }

  Ok(())
}
