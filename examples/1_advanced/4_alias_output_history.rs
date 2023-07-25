// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context;
use examples::create_did;
use examples::random_stronghold_path;
use examples::MemStorage;
use examples::API_ENDPOINT;
use identity_iota::core::json;
use identity_iota::core::FromJson;
use identity_iota::core::Timestamp;
use identity_iota::did::DID;
use identity_iota::document::Service;
use identity_iota::iota::block::address::Address;
use identity_iota::iota::block::output::RentStructure;
use identity_iota::iota::IotaClientExt;
use identity_iota::iota::IotaDID;
use identity_iota::iota::IotaDocument;
use identity_iota::iota::IotaIdentityClient;
use identity_iota::iota::IotaIdentityClientExt;
use identity_iota::storage::JwkMemStore;
use identity_iota::storage::KeyIdMemstore;
use identity_iota::verification::MethodRelationship;
use iota_sdk::client::secret::stronghold::StrongholdSecretManager;
use iota_sdk::client::secret::SecretManager;
use iota_sdk::client::Client;
use iota_sdk::client::Password;
use iota_sdk::types::block::input::Input;
use iota_sdk::types::block::output::AliasId;
use iota_sdk::types::block::output::AliasOutput;
use iota_sdk::types::block::output::AliasOutputBuilder;
use iota_sdk::types::block::output::Output;
use iota_sdk::types::block::output::OutputId;
use iota_sdk::types::block::output::OutputMetadata;
use iota_sdk::types::block::payload::transaction::TransactionEssence;
use iota_sdk::types::block::payload::Payload;
use iota_sdk::types::block::Block;

/// Demonstrates how to obtain the alias output history.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // Create a new client to interact with the IOTA ledger.
  // NOTE: a permanode is required to fetch older output histories.
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
  let (_, document, fragment): (Address, IotaDocument, String) =
    create_did(&client, &mut secret_manager, &storage).await?;
  let did: IotaDID = document.id().clone();

  // Resolve the latest state of the document.
  let mut document: IotaDocument = client.resolve_did(&did).await?;

  // Attach a new method relationship to the existing method.
  document.attach_method_relationship(
    &document.id().to_url().join(format!("#{fragment}"))?,
    MethodRelationship::Authentication,
  )?;

  // Adding multiple services.
  let services = [
    json!({"id": document.id().to_url().join("#my-service-0")?, "type": "MyService", "serviceEndpoint": "https://iota.org/"}),
  ];
  for service in services {
    let service: Service = Service::from_json_value(service)?;
    assert!(document.insert_service(service).is_ok());
    document.metadata.updated = Some(Timestamp::now_utc());

    // Increase the storage deposit and publish the update.
    let alias_output: AliasOutput = client.update_did_output(document.clone()).await?;
    let rent_structure: RentStructure = client.get_rent_structure().await?;
    let alias_output: AliasOutput = AliasOutputBuilder::from(&alias_output)
      .with_minimum_storage_deposit(rent_structure)
      .finish()?;
    client.publish_did_output(&secret_manager, alias_output).await?;
  }

  // ====================================
  // Retrieving the Alias Output History
  // ====================================
  let mut alias_history: Vec<AliasOutput> = Vec::new();

  // Step 0 - Get the latest Alias Output
  let alias_id: AliasId = AliasId::from(client.resolve_did(&did).await?.id());
  let (mut output_id, mut alias_output): (OutputId, AliasOutput) = client.get_alias_output(alias_id).await?;

  while alias_output.state_index() != 0 {
    // Step 1 - Get the current block
    let block: Block = current_block(&client, &output_id).await?;
    // Step 2 - Get the OutputId of the previous block
    output_id = previous_output_id(&block)?;
    // Step 3 - Get the Alias Output from the block
    alias_output = block_alias_output(&block, &alias_id)?;
    alias_history.push(alias_output.clone());
  }

  println!("Alias History: {alias_history:?}");

  Ok(())
}

async fn current_block(client: &Client, output_id: &OutputId) -> anyhow::Result<Block> {
  let output_metadata: OutputMetadata = client.get_output_metadata(output_id).await?;
  let block: Block = client.get_block(output_metadata.block_id()).await?;
  Ok(block)
}

fn previous_output_id(block: &Block) -> anyhow::Result<OutputId> {
  match block
    .payload()
    .context("expected a transaction payload, but no payload was found")?
  {
    Payload::Transaction(transaction_payload) => match transaction_payload.essence() {
      TransactionEssence::Regular(regular_transaction_essence) => {
        match regular_transaction_essence
          .inputs()
          .first()
          .context("expected an utxo for the block, but no input was found")?
        {
          Input::Utxo(utxo_input) => Ok(*utxo_input.output_id()),
          Input::Treasury(_) => {
            anyhow::bail!("expected an utxo input, found a treasury input");
          }
        }
      }
    },
    Payload::Milestone(_) | Payload::TreasuryTransaction(_) | Payload::TaggedData(_) => {
      anyhow::bail!("expected a transaction payload");
    }
  }
}

fn block_alias_output(block: &Block, alias_id: &AliasId) -> anyhow::Result<AliasOutput> {
  match block
    .payload()
    .context("expected a transaction payload, but no payload was found")?
  {
    Payload::Transaction(transaction_payload) => match transaction_payload.essence() {
      TransactionEssence::Regular(regular_transaction_essence) => {
        for (index, output) in regular_transaction_essence.outputs().iter().enumerate() {
          match output {
            Output::Alias(alias_output) => {
              if &alias_output.alias_id().or_from_output_id(
                &OutputId::new(
                  transaction_payload.id(),
                  index.try_into().context("output index must fit into a u16")?,
                )
                .context("failed to create OutputId")?,
              ) == alias_id
              {
                return Ok(alias_output.clone());
              }
            }
            Output::Basic(_) | Output::Foundry(_) | Output::Nft(_) | Output::Treasury(_) => continue,
          }
        }
      }
    },
    Payload::Milestone(_) | Payload::TreasuryTransaction(_) | Payload::TaggedData(_) => {
      anyhow::bail!("expected a transaction payload");
    }
  }
  anyhow::bail!("no alias output has been found");
}
