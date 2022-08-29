// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use examples::create_did;
use examples::random_stronghold_path;
use examples::NETWORK_ENDPOINT;
use identity_stardust::block::address::Address;
use identity_stardust::StardustDID;
use identity_stardust::StardustDocument;
use identity_stardust::StardustIdentityClientExt;
use iota_client::block::output::AliasOutput;
use iota_client::secret::stronghold::StrongholdSecretManager;
use iota_client::secret::SecretManager;
use iota_client::Client;

/// Demonstrates how to resolve an existing DID in an Alias Output.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // Create a new client to interact with the IOTA ledger.
  let client: Client = Client::builder()
    .with_primary_node(NETWORK_ENDPOINT, None)?
    .with_local_pow(false)
    .finish()?;

  // Create a new secret manager backed by a Stronghold.
  let mut secret_manager: SecretManager = SecretManager::Stronghold(
    StrongholdSecretManager::builder()
      .password("secure_password")
      .try_build(random_stronghold_path())?,
  );

  // Create a new DID in an Alias Output for us to modify.
  let (_, did): (Address, StardustDID) = create_did(&client, &mut secret_manager).await?;

  // Resolve the associated Alias Output and extract the DID document from it.
  let resolved: StardustDocument = client.resolve_did(&did).await?;
  println!("Resolved DID Document: {:#}", resolved);

  // We can also resolve the Alias Output directly.
  let alias_output: AliasOutput = client.resolve_did_output(&did).await?;

  println!("The Alias Output holds {} tokens", alias_output.amount());

  Ok(())
}
