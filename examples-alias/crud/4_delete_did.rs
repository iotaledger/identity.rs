// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::block::address::Address;
use iota_client::secret::SecretManager;
use iota_client::Client;

use identity_stardust::Error;
use identity_stardust::StardustClientExt;
use identity_stardust::StardustDID;
use identity_stardust::StardustIdentityClientExt;
use utils::create_did;

/// Demonstrates how to delete a DID in an Alias Output, reclaiming the storage deposit.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // Create a new DID in an Alias Output for us to modify.
  let (client, address, secret_manager, did): (Client, Address, SecretManager, StardustDID) = create_did().await?;

  // Deletes the Alias Output and its contained DID Document, rendering the DID permanently destroyed.
  // This operation is *not* reversible.
  // Deletion can only be done by the governor of the Alias Output.
  client.delete_did_output(&secret_manager, address, &did).await?;

  // Attempting to resolve a deleted DID results in a `NotFound` error.
  let error: Error = client.resolve_did(&did).await.unwrap_err();
  assert!(matches!(
    error,
    identity_stardust::Error::DIDResolutionError(iota_client::Error::NotFound)
  ));

  Ok(())
}
