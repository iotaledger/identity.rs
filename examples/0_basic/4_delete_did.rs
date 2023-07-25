// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use examples::create_did;
use examples::random_stronghold_path;
use examples::MemStorage;
use examples::API_ENDPOINT;
use identity_iota::iota::Error;
use identity_iota::iota::IotaClientExt;

use identity_iota::iota::IotaDocument;
use identity_iota::iota::IotaIdentityClientExt;
use identity_iota::storage::JwkMemStore;
use identity_iota::storage::KeyIdMemstore;
use iota_sdk::client::secret::stronghold::StrongholdSecretManager;
use iota_sdk::client::secret::SecretManager;
use iota_sdk::client::Client;
use iota_sdk::client::Password;
use iota_sdk::types::block::address::Address;

/// Demonstrates how to delete a DID in an Alias Output, reclaiming the storage deposit.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // Create a new client to interact with the IOTA ledger.
  let client: Client = Client::builder()
    .with_primary_node(API_ENDPOINT, None)?
    .finish()
    .await?;

  // Create a new secret manager backed by a Stronghold.
  let mut secret_manager: SecretManager = SecretManager::Stronghold(
    StrongholdSecretManager::builder()
      .password(Password::from("secure_password".to_owned()))
      .build(random_stronghold_path())?,
  );

  // Create a new DID in an Alias Output for us to modify.
  let storage: MemStorage = MemStorage::new(JwkMemStore::new(), KeyIdMemstore::new());
  let (address, document, _): (Address, IotaDocument, String) =
    create_did(&client, &mut secret_manager, &storage).await?;
  let did = document.id().clone();

  // Deletes the Alias Output and its contained DID Document, rendering the DID permanently destroyed.
  // This operation is *not* reversible.
  // Deletion can only be done by the governor of the Alias Output.
  client.delete_did_output(&secret_manager, address, &did).await?;

  // Attempting to resolve a deleted DID results in a `NoOutput` error.
  let error: Error = client.resolve_did(&did).await.unwrap_err();

  assert!(matches!(
    error,
    identity_iota::iota::Error::DIDResolutionError(iota_sdk::client::Error::Node(
      iota_sdk::client::node_api::error::Error::NotFound(..)
    ))
  ));

  Ok(())
}
