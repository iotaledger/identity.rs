// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Timestamp;
use identity_core::convert::FromJson;
use identity_core::json;
use identity_did::did::DID;
use identity_did::service::Service;
use identity_did::verification::MethodRelationship;
use iota_client::block::output::AliasOutput;
use iota_client::block::output::AliasOutputBuilder;
use iota_client::secret::SecretManager;
use iota_client::Client;

use identity_stardust::StardustClientExt;
use identity_stardust::StardustDID;
use identity_stardust::StardustDocument;
use identity_stardust::StardustIdentityClientExt;
use identity_stardust::StardustService;

mod ex0_create_did;

/// Demonstrates how to update a DID document in an existing Alias Output.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // Create a new DID in an Alias Output for us to modify.
  let (client, _, secret_manager, did): (Client, _, SecretManager, StardustDID) = ex0_create_did::run().await?;

  // Resolve the latest state of the document.
  let mut document: StardustDocument = client.resolve_did(&did).await?;

  // Attach a new method relationship to the existing method.
  document.attach_method_relationship(
    &document.id().to_url().join("#key-1")?,
    MethodRelationship::Authentication,
  )?;

  // Add a new Service.
  let service: StardustService = Service::from_json_value(json!({
    "id": document.id().to_url().join("#linked-domain")?,
    "type": "LinkedDomains",
    "serviceEndpoint": "https://iota.org/"
  }))?;
  assert!(document.insert_service(service));
  document.metadata.updated = Some(Timestamp::now_utc());

  // Resolve the latest output and update it with the given document.
  let alias_output: AliasOutput = client.update_did_output(document.clone()).await?;

  // Because the size of the DID document increased, we have to increase the allocated storage deposit.
  // This increases the deposit amount to the new minimum.
  let rent_structure = client.get_rent_structure().await?;
  let alias_output = AliasOutputBuilder::from(&alias_output)
    .with_minimum_storage_deposit(rent_structure)
    .finish()?;

  // Publish the updated Alias Output.
  let resolved_document: StardustDocument = client.publish_did_output(&secret_manager, alias_output).await?;
  println!("Updated DID document: {:#}", resolved_document);
  assert_eq!(document, resolved_document);

  Ok(())
}
