// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context;
use function_name::named;
use identity_core::crypto::KeyPair;
use identity_core::crypto::KeyType;
use identity_iota_core::did::IotaDID;
use identity_iota_core::tangle::Network;
use identity_iota_core::tangle::NetworkName;

use crate::types::KeyLocation;

use super::test_util::random_string;
use super::Storage;

macro_rules! ensure {
  ($cond:expr, $($msg:expr),*) => {{
    if !$cond {
      let message: String = format!($( $msg, )*);
      let fn_name: &'static str = function_name!();
      return Err(anyhow::Error::msg(format!("[{}]: {}", fn_name, message)));
    }
  };};
}

macro_rules! ensure_eq {
  ($left:expr, $right:expr, $($msg:expr),*) => {{
    if $left != $right {
      let message: String = format!($( $msg, )*);
      let fn_name: &'static str = function_name!();
      return Err(anyhow::Error::msg(format!("[{}]: {}", fn_name, message)));
    }
  };};
}

#[named]
pub async fn storage_did_create_test(storage: Box<dyn Storage>) -> anyhow::Result<()> {
  let fragment: String = random_string(20);
  let keypair: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
  let network: NetworkName = Network::Mainnet.name();

  let expected_did: IotaDID = IotaDID::new_with_network(keypair.public().as_ref(), network.clone()).unwrap();

  let (did, location): (IotaDID, KeyLocation) = storage
    .did_create(network, &fragment, Some(keypair.private().to_owned()))
    .await
    .context("did_create returned an error")?;

  ensure_eq!(
    did,
    expected_did,
    "expected returned did to be `{}`, was `{}`",
    expected_did,
    did
  );

  let exists: bool = storage
    .key_exists(&did, &location)
    .await
    .context("key_exists returned an error")?;

  ensure!(exists, "expected key at location {} to exist", location);

  Ok(())
}
