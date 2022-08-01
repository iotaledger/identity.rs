// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_stardust::StardustClientExt;
use identity_stardust::StardustDocument;
use iota_client::block::address::Address;
use iota_client::secret::SecretManager;
use iota_client::Client;

mod create_did;

/// Demonstrate how to destroy an existing DID in an Alias Output, reclaiming the stored deposit.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // Create a new DID in an Alias Output for us to modify.
  let (client, _, secret_manager, document): (Client, Address, SecretManager, StardustDocument) =
    create_did::run().await?;

  // Deactivate the DID by publishing an empty document.
  // This process can be reversed since the Alias Output is not destroyed.
  client.deactivate_did_output(&secret_manager, document.id()).await?;

  // Wait for the node to index the new state.
  tokio::time::sleep(std::time::Duration::from_secs(5)).await;

  // Attempting to resolve a deactivated DID results in a `DeactivatedDID` error.
  let error = client.resolve_did(document.id()).await.unwrap_err();

  assert!(matches!(error, identity_stardust::Error::DeactivatedDID(_)));

  Ok(())
}
