// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use examples::create_did;
use examples::random_stronghold_path;
use examples::API_ENDPOINT;
use identity_iota::core::json;
use identity_iota::core::FromJson;
use identity_iota::core::Timestamp;
use identity_iota::crypto::KeyPair;
use identity_iota::crypto::KeyType;
use identity_iota::did::DIDUrl;
use identity_iota::did::DID;
use identity_iota::document::Service;
use identity_iota::iota::block::address::Address;
use identity_iota::iota::block::output::RentStructure;
use identity_iota::iota::IotaClientExt;
use identity_iota::iota::IotaDID;
use identity_iota::iota::IotaDocument;
use identity_iota::iota::IotaIdentityClientExt;
use identity_iota::verification::MethodRelationship;
use identity_iota::verification::MethodScope;
use identity_iota::verification::VerificationMethod;
use iota_client::block::output::AliasOutput;
use iota_client::block::output::AliasOutputBuilder;
use iota_client::secret::stronghold::StrongholdSecretManager;
use iota_client::secret::SecretManager;
use iota_client::Client;

/// Demonstrates how to update a DID document in an existing Alias Output.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // Create a new client to interact with the IOTA ledger.
  let client: Client = Client::builder().with_primary_node(API_ENDPOINT, None)?.finish()?;

  // Create a new secret manager backed by a Stronghold.
  let mut secret_manager: SecretManager = SecretManager::Stronghold(
    StrongholdSecretManager::builder()
      .password("secure_password")
      .build(random_stronghold_path())?,
  );

  // Create a new DID in an Alias Output for us to modify.
  let (_, document, _): (Address, IotaDocument, KeyPair) = create_did(&client, &mut secret_manager).await?;
  let did: IotaDID = document.id().clone();

  // Resolve the latest state of the document.
  let mut document: IotaDocument = client.resolve_did(&did).await?;

  // Insert a new Ed25519 verification method in the DID document.
  let keypair: KeyPair = KeyPair::new(KeyType::Ed25519)?;
  let method: VerificationMethod =
    VerificationMethod::new(document.id().clone(), keypair.type_(), keypair.public(), "#key-2")?;
  document.insert_method(method, MethodScope::VerificationMethod)?;

  // Attach a new method relationship to the inserted method.
  document.attach_method_relationship(
    &document.id().to_url().join("#key-2")?,
    MethodRelationship::Authentication,
  )?;

  // Add a new Service.
  let service: Service = Service::from_json_value(json!({
    "id": document.id().to_url().join("#linked-domain")?,
    "type": "LinkedDomains",
    "serviceEndpoint": "https://iota.org/"
  }))?;
  assert!(document.insert_service(service).is_ok());
  document.metadata.updated = Some(Timestamp::now_utc());

  // Remove a verification method.
  let original_method: DIDUrl = document.resolve_method("key-1", None).unwrap().id().clone();
  document.remove_method(&original_method).unwrap();

  // Resolve the latest output and update it with the given document.
  let alias_output: AliasOutput = client.update_did_output(document.clone()).await?;

  // Because the size of the DID document increased, we have to increase the allocated storage deposit.
  // This increases the deposit amount to the new minimum.
  let rent_structure: RentStructure = client.get_rent_structure().await?;
  let alias_output: AliasOutput = AliasOutputBuilder::from(&alias_output)
    .with_minimum_storage_deposit(rent_structure)
    .finish(client.get_token_supply().await?)?;

  // Publish the updated Alias Output.
  let updated: IotaDocument = client.publish_did_output(&secret_manager, alias_output).await?;
  println!("Updated DID document: {updated:#}");

  Ok(())
}
