// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use anyhow::Context;
use iota_sdk::types::base_types::IotaAddress;
use iota_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use iota_sdk::types::transaction::Argument;
use iota_sdk::types::transaction::ObjectArg;
use iota_sdk::types::Identifier;
use iota_sdk::IotaClient;
use iota_sdk::IotaClientBuilder;
use serde::Serialize;
use tokio::process::Command;

use crate::Error;

pub const LOCAL_NETWORK: &str = "http://127.0.0.1:9000";

pub async fn get_client(network: &str) -> Result<IotaClient, Error> {
  let client = IotaClientBuilder::default()
    .build(network)
    .await
    .map_err(|err| Error::Network(format!("failed to connect to {network}"), err))?;

  Ok(client)
}

pub async fn request_funds(address: &IotaAddress) -> anyhow::Result<()> {
  let output = Command::new("iota")
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

pub(crate) fn parse_identifier(name: &str) -> Result<Identifier, Error> {
  Identifier::from_str(name).map_err(|err| Error::ParsingFailed(format!(r#""{name}" to identifier; {err}"#)))
}

pub(crate) fn ptb_pure<T>(ptb: &mut ProgrammableTransactionBuilder, name: &str, value: T) -> Result<Argument, Error>
where
  T: Serialize + core::fmt::Debug,
{
  ptb.pure(&value).map_err(|err| {
    Error::InvalidArgument(format!(
      r"could not serialize pure value {name} with value {value:?}; {err}"
    ))
  })
}

pub(crate) fn ptb_obj(
  ptb: &mut ProgrammableTransactionBuilder,
  name: &str,
  value: ObjectArg,
) -> Result<Argument, Error> {
  ptb
    .obj(value)
    .map_err(|err| Error::InvalidArgument(format!("could not serialize object {name} {value:?}; {err}")))
}
