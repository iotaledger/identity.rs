// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context;
use examples::get_address;
use examples::random_stronghold_path;
use examples::request_faucet_funds;
use identity_iota::crypto::KeyPair;
use identity_iota::crypto::KeyType;
use identity_iota::did::MethodScope;
use identity_iota::iota::IotaClientExt;
use identity_iota::iota::IotaDocument;
use identity_iota::iota::IotaIdentityClientExt;
use identity_iota::iota::IotaVerificationMethod;
use identity_iota::iota::NetworkName;
use iota_client::block::address::Address;
use iota_client::block::output::AliasOutput;
use iota_client::secret::stronghold::StrongholdSecretManager;
use iota_client::secret::SecretManager;
use iota_client::Client;

/// Demonstrates how to create a DID Document and publish it in a new Alias Output.
///
/// In this example we connect to the Shimmer testnet, but it can be adapted
/// to run on a private network by setting the network and faucet endpoints.
///
/// See the following instructions on running your own private network
/// https://wiki.iota.org/hornet/develop/how_tos/private_tangle
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // The API endpoint of an IOTA node, e.g. Hornet.
  // In a locally running Hornet node, this would usually be `http://127.0.0.1:14265`.
  let network_endpoint: &str = "https://api.testnet.shimmer.network/";

  // The faucet endpoint from where we can request funds for testing purposes.
  // In a locally running hornet node, this would usually be `http://127.0.0.1:8091/api/enqueue`.
  let faucet_endpoint: &str = "https://faucet.testnet.shimmer.network/api/enqueue";

  // Create a new client to interact with the IOTA ledger.
  let client: Client = Client::builder().with_primary_node(network_endpoint, None)?.finish()?;

  // Create a new secret manager backed by a Stronghold.
  let mut secret_manager: SecretManager = SecretManager::Stronghold(
    StrongholdSecretManager::builder()
      .password("secure_password")
      .build(random_stronghold_path())?,
  );

  // Get an address from the secret manager.
  let address: Address = get_address(&client, &mut secret_manager)
    .await
    .context("failed to get address")?;

  // Get the Bech32 human-readable part (HRP) of the network.
  let network_name: NetworkName = client.network_name().await?;

  // Request funds from the private tangle faucet for the address.
  request_faucet_funds(&client, address, network_name.as_ref(), faucet_endpoint).await?;

  // Create a new DID document with a placeholder DID.
  // The DID will be derived from the Alias Id of the Alias Output after publishing.
  let mut document: IotaDocument = IotaDocument::new(&network_name);

  // Insert a new Ed25519 verification method in the DID document.
  let keypair: KeyPair = KeyPair::new(KeyType::Ed25519)?;
  let method: IotaVerificationMethod =
    IotaVerificationMethod::new(document.id().clone(), keypair.type_(), keypair.public(), "#key-1")?;
  document.insert_method(method, MethodScope::VerificationMethod)?;

  // Construct an Alias Output containing the DID document, with the wallet address
  // set as both the state controller and governor.
  let alias_output: AliasOutput = client.new_did_output(address, document, None).await?;

  // Publish the Alias Output and get the published DID document.
  let document: IotaDocument = client.publish_did_output(&secret_manager, alias_output).await?;
  println!("Published DID document: {:#}", document);

  Ok(())
}
