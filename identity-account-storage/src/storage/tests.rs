// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context;
use function_name::named;
use identity_core::crypto::KeyPair;
use identity_core::crypto::KeyType;
use identity_core::crypto::PublicKey;
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
  let fragment: String = random_string();
  let keypair: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
  let network: NetworkName = Network::Mainnet.name();

  let expected_did: IotaDID = IotaDID::new_with_network(keypair.public().as_ref(), network.clone()).unwrap();
  let expected_location: KeyLocation = KeyLocation::new(KeyType::Ed25519, fragment.clone(), keypair.public().as_ref());

  let (did, location): (IotaDID, KeyLocation) = storage
    .did_create(network.clone(), &fragment, Some(keypair.private().to_owned()))
    .await
    .context("did_create returned an error")?;

  ensure_eq!(
    did,
    expected_did,
    "expected returned did to be `{expected_did}`, was `{did}`"
  );

  ensure_eq!(
    location,
    expected_location,
    "expected returned location to be `{expected_location}`, was `{location}`"
  );

  let exists: bool = storage
    .key_exists(&did, &location)
    .await
    .context("key_exists returned an error")?;

  ensure!(exists, "expected key at location `{location}` to exist");

  let result: Result<_, crate::Error> = storage
    .did_create(network, &fragment, Some(keypair.private().to_owned()))
    .await;

  ensure!(
    result.is_err(),
    "expected did_create to return an error when attempting to create an identity from the same private key twice"
  );

  let public_key: PublicKey = storage
    .key_public(&did, &location)
    .await
    .context("key_public returned an error")?;

  ensure_eq!(
    public_key.as_ref(),
    keypair.public().as_ref(),
    "expected key_public to return `{:?}`, returned `{public_key:?}`",
    keypair.public()
  );

  Ok(())
}

#[named]
pub async fn storage_key_generate_test(storage: Box<dyn Storage>) -> anyhow::Result<()> {
  let fragment: String = random_string();
  let keypair: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
  let network: NetworkName = Network::Mainnet.name();

  let (did, _): (IotaDID, _) = storage
    .did_create(network.clone(), &fragment, Some(keypair.private().to_owned()))
    .await
    .context("did_create returned an error")?;

  let key_types: [KeyType; 2] = [KeyType::Ed25519, KeyType::X25519];

  let mut locations: Vec<KeyLocation> = Vec::with_capacity(key_types.len());

  for key_type in key_types {
    let key_fragment: String = random_string();
    let location: KeyLocation = storage
      .key_generate(&did, key_type, &key_fragment)
      .await
      .context("key_generate returned an error")?;
    locations.push(location);
  }

  for location in locations {
    let exists: bool = storage
      .key_exists(&did, &location)
      .await
      .context("key_exists returned an error")?;

    ensure!(exists, "expected key at location `{location}` to exist");

    // Ensure we can retrieve the public key without erroring.
    storage
      .key_public(&did, &location)
      .await
      .context("key_public returned an error")?;
  }

  Ok(())
}
