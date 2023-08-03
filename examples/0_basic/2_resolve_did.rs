// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use examples::create_did;
use examples::random_stronghold_path;
use examples::MemStorage;
use examples::API_ENDPOINT;
use identity_iota::iota::block::address::Address;

use identity_iota::iota::IotaDocument;
use identity_iota::iota::IotaIdentityClientExt;
use identity_iota::prelude::Resolver;
use identity_iota::storage::JwkMemStore;
use identity_iota::storage::KeyIdMemstore;
use iota_sdk::client::secret::stronghold::StrongholdSecretManager;
use iota_sdk::client::secret::SecretManager;
use iota_sdk::client::Client;
use iota_sdk::client::Password;
use iota_sdk::types::block::output::AliasOutput;

/// Demonstrates how to resolve an existing DID in an Alias Output.
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

  // Create a new DID in an Alias Output for us to resolve.
  let storage: MemStorage = MemStorage::new(JwkMemStore::new(), KeyIdMemstore::new());
  let (_, document, _): (Address, IotaDocument, String) = create_did(&client, &mut secret_manager, &storage).await?;

  let did = document.id().clone();

  // We can resolve a `IotaDID` with the client itself.
  // Resolve the associated Alias Output and extract the DID document from it.
  let client_document: IotaDocument = client.resolve_did(&did).await?;
  println!("Client resolved DID Document: {client_document:#}");

  // We can also create a `Resolver` that has additional convenience methods,
  // for example to resolve presentation issuers or to verify presentations.
  let mut resolver = Resolver::<IotaDocument>::new();

  // We need to register a handler that can resolve IOTA DIDs.
  // This convenience method only requires us to provide a client.
  resolver.attach_iota_handler(client.clone());

  let resolver_document: IotaDocument = resolver.resolve(&did).await.unwrap();

  // Client and Resolver resolve to the same document in this case.
  assert_eq!(client_document, resolver_document);

  // We can also resolve the Alias Output directly.
  let alias_output: AliasOutput = client.resolve_did_output(&did).await?;

  println!("The Alias Output holds {} tokens", alias_output.amount());

  Ok(())
}
