// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::iota_sdk_abstraction::types::base_types::{ObjectID, IotaAddress};
use crate::iota_sdk_abstraction::types::TypeTag;

#[cfg(not(target_arch = "wasm32"))]
pub mod not_wasm32 {
  use super::*;

  use tokio::process::Command;
  use anyhow::Context as _;

  use iota_sdk::IotaClientBuilder;

  use crate::sui::iota_sdk_adapter::IotaClientAdapter;
  use crate::Error;

  pub async fn get_client(network: &str) -> Result<IotaClientAdapter, Error> {
    let client = IotaClientBuilder::default()
        .build(network)
        .await
        .map_err(|err| Error::Network(format!("failed to connect to {network}"), err))?;

    IotaClientAdapter::new(client)
  }

  pub async fn request_funds(address: &IotaAddress) -> anyhow::Result<()> {
    let output = Command::new("iota")
        .arg("client")
        .arg("faucet")
        .arg("--address")
        .arg(address.to_string())
        .arg("--url")
        .arg("http://127.0.0.1:9123/gas")
        .arg("--json")
        .output()
        .await
        .context("Failed to execute command")?;

    if !output.status.success() {
      anyhow::bail!(
      "Failed to request funds from faucet: {}",
      std::str::from_utf8(&output.stderr)?
    );
    }

    Ok(())
  }
}

#[cfg(not(target_arch = "wasm32"))]
pub use not_wasm32::*;

pub const LOCAL_NETWORK: &str = "http://127.0.0.1:9000";

pub trait MoveType {
  fn move_type(package: ObjectID) -> TypeTag;
}

impl MoveType for u8 {
  fn move_type(_package: ObjectID) -> TypeTag {
    TypeTag::U8
  }
}

impl MoveType for u16 {
  fn move_type(_package: ObjectID) -> TypeTag {
    TypeTag::U16
  }
}

impl MoveType for u32 {
  fn move_type(_package: ObjectID) -> TypeTag {
    TypeTag::U32
  }
}

impl MoveType for u64 {
  fn move_type(_package: ObjectID) -> TypeTag {
    TypeTag::U64
  }
}

impl MoveType for u128 {
  fn move_type(_package: ObjectID) -> TypeTag {
    TypeTag::U128
  }
}

impl MoveType for bool {
  fn move_type(_package: ObjectID) -> TypeTag {
    TypeTag::Bool
  }
}

impl MoveType for IotaAddress {
  fn move_type(_package: ObjectID) -> TypeTag {
    TypeTag::Address
  }
}

impl<T: MoveType> MoveType for Vec<T> {
  fn move_type(package: ObjectID) -> TypeTag {
    TypeTag::Vector(Box::new(T::move_type(package)))
  }
}
