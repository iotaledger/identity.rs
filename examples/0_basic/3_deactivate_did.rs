// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use examples::create_did;
use examples::random_stronghold_path;
use examples::MemStorage;
use examples::API_ENDPOINT;
use identity_iota::iota::block::address::Address;
use identity_iota::iota::IotaClientExt;
use identity_iota::iota::IotaDID;
use identity_iota::iota::IotaDocument;
use identity_iota::iota::IotaIdentityClientExt;
use identity_iota::storage::JwkMemStore;
use identity_iota::storage::KeyIdMemstore;
use iota_sdk::client::secret::stronghold::StrongholdSecretManager;
use iota_sdk::client::secret::SecretManager;
use iota_sdk::client::Client;
use iota_sdk::client::Password;
use iota_sdk::types::block::output::AliasOutput;
use iota_sdk::types::block::output::AliasOutputBuilder;

/// Demonstrates how to deactivate a DID in an Alias Output.
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
  let (_, document, _): (Address, IotaDocument, String) = create_did(&client, &mut secret_manager, &storage).await?;
  let did: IotaDID = document.id().clone();

  // Resolve the latest state of the DID document.
  let document: IotaDocument = client.resolve_did(&did).await?;

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
  let deactivated: IotaDocument = client.resolve_did(&did).await?;
  println!("Deactivated DID document: {deactivated:#}");
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
  let reactivated: IotaDocument = client.resolve_did(&did).await?;
  assert_eq!(document, reactivated);
  assert!(!reactivated.metadata.deactivated.unwrap_or_default());

  Ok(())
}
