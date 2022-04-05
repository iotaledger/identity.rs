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

  let network: NetworkName = Network::Devnet.name();
  let (did, location): (IotaDID, KeyLocation) = storage
    .did_create(network.clone(), &fragment, None)
    .await
    .context("did_create returned an error")?;

  ensure_eq!(
    did.network_str(),
    network.as_ref(),
    "expected network `{network}` for the generated DID, was `{}`",
    did.network_str()
  );

  let exists: bool = storage
    .key_exists(&did, &location)
    .await
    .context("key_exists returned an error")?;

  ensure!(exists, "expected key at location `{location}` to exist");

  let public_key: PublicKey = storage
    .key_public(&did, &location)
    .await
    .context("key_public returned an error")?;

  let expected_did: IotaDID = IotaDID::new_with_network(public_key.as_ref(), network).unwrap();

  ensure_eq!(
    did,
    expected_did,
    "returned did `{did}` did not match did created from retrieved public key and network, expected: `{expected_did}`"
  );

  Ok(())
}

#[named]
pub async fn storage_key_generate_test(storage: Box<dyn Storage>) -> anyhow::Result<()> {
  let fragment: String = random_string();
  let network: NetworkName = Network::Mainnet.name();

  let (did, _): (IotaDID, _) = storage
    .did_create(network.clone(), &fragment, None)
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

#[named]
pub async fn storage_key_delete_test(storage: Box<dyn Storage>) -> anyhow::Result<()> {
  let fragment: String = random_string();
  let network: NetworkName = Network::Mainnet.name();

  let (did, _): (IotaDID, _) = storage
    .did_create(network.clone(), &fragment, None)
    .await
    .context("did_create returned an error")?;

  let mut locations: Vec<KeyLocation> = Vec::with_capacity(20);

  for _ in 0..20 {
    let key_fragment: String = random_string();
    let location: KeyLocation = storage
      .key_generate(&did, KeyType::Ed25519, &key_fragment)
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

    let deleted: bool = storage
      .key_delete(&did, &location)
      .await
      .context("key_delete returned an error")?;

    ensure!(deleted, "expected key at location `{location}` to be deleted");

    let deleted: bool = storage
      .key_delete(&did, &location)
      .await
      .context("key_delete returned an error")?;

    ensure!(!deleted, "expected key at location `{location}` to already be deleted");
  }

  Ok(())
}

#[named]
pub async fn storage_did_list_test(storage: Box<dyn Storage>) -> anyhow::Result<()> {
  const NUM_IDENTITIES: usize = 20;
  let fragment: String = random_string();
  let network: NetworkName = Network::Mainnet.name();

  let list: Vec<IotaDID> = storage.did_list().await.context("did_list returned an error")?;

  ensure!(
    list.is_empty(),
    "expected list to be empty, but found {} element(s)",
    list.len()
  );

  let mut dids: Vec<IotaDID> = Vec::with_capacity(NUM_IDENTITIES);
  for i in 0..NUM_IDENTITIES {
    let (did, _): (IotaDID, _) = storage
      .did_create(network.clone(), &fragment, None)
      .await
      .context("did_create returned an error")?;

    let exists: bool = storage.did_exists(&did).await.context("did_exists returned an error")?;
    ensure!(exists, "expected did `{did}` to exist");

    let list_len: usize = storage.did_list().await.context("did_list returned an error")?.len();
    let expected_len: usize = i + 1;

    ensure_eq!(
      list_len,
      expected_len,
      "expected did_list to return a list of len {expected_len}, got {list_len} elements instead"
    );

    dids.push(did);
  }

  for (i, did) in dids.into_iter().enumerate() {
    let purged: bool = storage.did_purge(&did).await.context("did_purge returned an error")?;

    ensure!(purged, "expected the did `{did}` to be purged");

    let exists: bool = storage.did_exists(&did).await.context("did_exists returned an error")?;
    ensure!(!exists, "expected did `{did}` to no longer exist");

    let list_len: usize = storage.did_list().await.context("did_list returned an error")?.len();
    let expected_len: usize = NUM_IDENTITIES - (i + 1);

    ensure_eq!(
      list_len,
      expected_len,
      "expected did_list to return a list of len {expected_len}, got {list_len} elements instead"
    );
  }

  Ok(())
}
