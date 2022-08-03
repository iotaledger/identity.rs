// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::block::address::Address;
use iota_client::block::output::AliasOutput;
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
  client.deactivate_did_output(&secret_manager, document.id()).await?;

  // Wait for the node to index the new state.
  tokio::time::sleep(std::time::Duration::from_secs(5)).await;

  // Attempting to resolve a deactivated DID results in an document
  // where the metadata's `deactivated` field is `true`.
  let deactivated_document: StardustDocument = client.resolve_did(document.id()).await?;

  assert!(matches!(deactivated_document.metadata.deactivated, Some(true)));

  // Re-activate the DID by publishing a valid document.
  let alias_output: AliasOutput = client.update_did_output(document.clone()).await?;
  client.publish_did_output(&secret_manager, alias_output).await?;

  // Resolve the republished document.
  let resolved_document: StardustDocument = client.resolve_did(document.id()).await?;

  assert_eq!(document, resolved_document);

  Ok(())
}
