// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::block::address::Address;
use iota_client::block::output::AliasOutput;
use iota_client::block::output::AliasOutputBuilder;
use iota_client::secret::SecretManager;
use iota_client::Client;

use identity_stardust::StardustClientExt;
use identity_stardust::StardustDID;
use identity_stardust::StardustDocument;
use identity_stardust::StardustIdentityClientExt;

mod ex0_create_did;

/// Demonstrates how to deactivate a DID in an Alias Output.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // Create a new DID in an Alias Output for us to modify.
  let (client, _, secret_manager, did): (Client, Address, SecretManager, StardustDID) = ex0_create_did::run().await?;

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
