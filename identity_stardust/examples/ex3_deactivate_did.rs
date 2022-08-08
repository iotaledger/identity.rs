// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::block::address::Address;
use iota_client::block::output::AliasOutput;
use iota_client::block::output::AliasOutputBuilder;
use iota_client::secret::SecretManager;
use iota_client::Client;

use identity_stardust::StardustClientExt;
use identity_stardust::StardustDocument;
use identity_stardust::StardustIdentityClientExt;

mod ex0_create_did;

/// Demonstrate how to deactivate a DID in an Alias Output.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // Create a new DID in an Alias Output for us to modify.
  let (client, _, secret_manager, document): (Client, Address, SecretManager, StardustDocument) =
    ex0_create_did::run().await?;

  // Deactivate the DID by publishing an empty document.
  // This process can be reversed since the Alias Output is not destroyed.
  // Deactivation can only be done by the state controller of the Alias Output.
  let deactivated_output: AliasOutput = client.deactivate_did_output(document.id()).await?;

  // Optional: reduce and reclaim the storage deposit, sending the tokens to the state controller.
  let rent_structure = client.get_rent_structure().await?;
  let deactivated_output = AliasOutputBuilder::from(&deactivated_output)
    .with_minimum_storage_deposit(rent_structure)
    .finish()?;
  let _ = client.publish_did_output(&secret_manager, deactivated_output).await?;

  // Wait for the node to index the new state.
  tokio::time::sleep(std::time::Duration::from_secs(5)).await;

  // Attempting to resolve a deactivated DID returns an empty DID document
  // with its `deactivated` metadata field set to `true`.
  let deactivated_document: StardustDocument = client.resolve_did(document.id()).await?;
  println!("Deactivated DID document: {:#}", deactivated_document);
  assert_eq!(deactivated_document.metadata.deactivated, Some(true));

  // Re-activate the DID by publishing a valid DID document.
  let alias_output: AliasOutput = client.update_did_output(document.clone()).await?;

  // Increase the storage deposit to the minimum again, if it was reclaimed during deactivation.
  let rent_structure = client.get_rent_structure().await?;
  let reactivated_output = AliasOutputBuilder::from(&alias_output)
    .with_minimum_storage_deposit(rent_structure)
    .finish()?;
  client.publish_did_output(&secret_manager, reactivated_output).await?;

  // Resolve the reactivated DID document.
  let reactivated_document: StardustDocument = client.resolve_did(document.id()).await?;
  assert_eq!(document, reactivated_document);
  assert!(!reactivated_document.metadata.deactivated.unwrap_or_default());

  Ok(())
}
