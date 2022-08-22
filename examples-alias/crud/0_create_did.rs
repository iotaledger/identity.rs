// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context;
use identity_core::convert::ToJson;
use identity_core::crypto::KeyPair;
use identity_core::crypto::KeyType;
use identity_did::verification::MethodScope;
use iota_client::block::address::Address;
use iota_client::block::output::AliasOutput;
use iota_client::secret::SecretManager;
use iota_client::Client;
use utils::get_address_with_funds;
use utils::NETWORK_ENDPOINT;

use identity_stardust::NetworkName;
use identity_stardust::StardustClientExt;
use identity_stardust::StardustDocument;
use identity_stardust::StardustIdentityClientExt;
use identity_stardust::StardustVerificationMethod;

/// Demonstrates how to create a DID Document and publish it in a new Alias Output.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // Create a client and a wallet address with funds from the testnet faucet.
  let client: Client = Client::builder().with_primary_node(NETWORK_ENDPOINT, None)?.finish()?;

  // Get an address and a secret manager with funds for testing.
  let (address, secret_manager): (Address, SecretManager) = get_address_with_funds(&client)
    .await
    .context("failed to get address with funds")?;

  // Get the Bech32 human-readable part (HRP) of the network.
  let network_name: NetworkName = client.network_name().await?;

  // Create a new DID document with a placeholder DID.
  // The DID will be derived from the Alias Id of the Alias Output after publishing.
  let mut document: StardustDocument = StardustDocument::new(&network_name);

  // Insert a new Ed25519 verification method in the DID document.
  let keypair: KeyPair = KeyPair::new(KeyType::Ed25519)?;
  let method: StardustVerificationMethod =
    StardustVerificationMethod::new(document.id().clone(), keypair.type_(), keypair.public(), "#key-1")?;
  document.insert_method(method, MethodScope::VerificationMethod)?;

  // Construct an Alias Output containing the DID document, with the wallet address
  // set as both the state controller and governor.
  let alias_output: AliasOutput = client.new_did_output(address, document, None).await?;
  println!("Alias Output: {}", alias_output.to_json()?);

  // Publish the Alias Output and get the published DID document.
  let document: StardustDocument = client.publish_did_output(&secret_manager, alias_output).await?;
  println!("Published DID document: {:#}", document);

  Ok(())
}
