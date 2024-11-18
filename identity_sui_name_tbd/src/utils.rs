// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Serialize;

use crate::iota_sdk_abstraction::types::base_types::IotaAddress;
use crate::iota_sdk_abstraction::types::base_types::ObjectID;
use crate::iota_sdk_abstraction::types::TypeTag;
use crate::IotaVerifiableCredential;

#[cfg(not(target_arch = "wasm32"))]
pub mod not_wasm32 {
  use super::*;

  use anyhow::Context as _;
  use tokio::process::Command;

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

pub enum TypedValue<'a, T: MoveType> {
  IotaVerifiableCredential(&'a IotaVerifiableCredential),
  Other(&'a T),
}

pub trait MoveType<T: Serialize = Self>: Serialize {
  fn move_type(package: ObjectID) -> TypeTag;

  fn get_typed_value(&self, _package: ObjectID) -> TypedValue<Self> where Self: MoveType, Self: Sized {
    TypedValue::Other(self)
  }
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

// Alias name for the Send trait controlled by the "send-sync-transaction" feature
cfg_if::cfg_if! {
  if #[cfg(feature = "send-sync-transaction")] {
    pub trait OptionalSend: Send {}
    impl<T> OptionalSend for T where T: Send {}
  } else {
    pub trait OptionalSend: {}
    impl<T> OptionalSend for T {}
  }
}