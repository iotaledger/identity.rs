// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;

use identity_stardust::block::address::Address;
use iota_client::block::output::AliasOutput;
use iota_client::block::output::AliasOutputBuilder;
use iota_client::secret::SecretManager;
use iota_client::Client;

use examples::create_did;
use examples::NETWORK_ENDPOINT;
use identity_stardust::StardustClientExt;
use identity_stardust::StardustDID;
use identity_stardust::StardustDocument;
use identity_stardust::StardustIdentityClientExt;
use iota_client::secret::stronghold::StrongholdSecretManager;

/// Demonstrates how to deactivate a DID in an Alias Output.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // Create a new client to interact with the IOTA ledger.
  let client: Client = Client::builder().with_primary_node(NETWORK_ENDPOINT, None)?.finish()?;

  // Create a new secret manager backed by a Stronghold.
  let mut secret_manager: SecretManager = SecretManager::Stronghold(
    StrongholdSecretManager::builder()
      .password("secure_password")
      .try_build(PathBuf::from("./example-strong.hodl"))?,
  );

  // Create a new DID in an Alias Output for us to modify.
  let (_, did): (Address, StardustDID) = create_did(&client, &mut secret_manager).await?;

  // Resolve the latest state of the DID document, so we can reactivate it later.
  let document: StardustDocument = client.resolve_did(&did).await?;

  // Deactivate the DID by publishing an empty document.
  // This process can be reversed since the Alias Output is not destroyed.
  // Deactivation may only be performed by the state controller of the Alias Output.
  let deactivated_output: AliasOutput = client.deactivate_did_output(&did).await?;

  // Optional: reduce and reclaim the storage deposit, sending the tokens to the state controller.
  let rent_structure = client.get_rent_structure().await?;
  let deactivated_output = AliasOutputBuilder::from(&deactivated_output)
    .with_minimum_storage_deposit(rent_structure)
    .finish()?;

  // Publish the deactivated DID document.
  let _ = client.publish_did_output(&secret_manager, deactivated_output).await?;

  // Resolving a deactivated DID returns an empty DID document
  // with its `deactivated` metadata field set to `true`.
  let deactivated: StardustDocument = client.resolve_did(&did).await?;
  println!("Deactivated DID document: {:#}", deactivated);
  assert_eq!(deactivated.metadata.deactivated, Some(true));

  // Re-activate the DID by publishing a valid DID document.
  let reactivated_output: AliasOutput = client.update_did_output(document.clone()).await?;

  // Increase the storage deposit to the minimum again, if it was reclaimed during deactivation.
  let rent_structure = client.get_rent_structure().await?;
  let reactivated_output = AliasOutputBuilder::from(&reactivated_output)
    .with_minimum_storage_deposit(rent_structure)
    .finish()?;
  client.publish_did_output(&secret_manager, reactivated_output).await?;

  // Resolve the reactivated DID document.
  let reactivated: StardustDocument = client.resolve_did(&did).await?;
  assert_eq!(document, reactivated);
  assert!(!reactivated.metadata.deactivated.unwrap_or_default());

  Ok(())
}
