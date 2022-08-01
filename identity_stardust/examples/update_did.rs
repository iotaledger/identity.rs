// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::convert::FromJson;
use identity_core::json;
use identity_did::did::DID;
use identity_did::service::Service;
use identity_did::verification::MethodRelationship;
use identity_stardust::StardustClientExt;
use identity_stardust::StardustDocument;
use identity_stardust::StardustService;
use iota_client::block::output::AliasOutput;
use iota_client::block::output::AliasOutputBuilder;
use iota_client::secret::SecretManager;
use iota_client::Client;

mod create_did;

/// Demonstrate how to modify a DID document in an existing Alias Output.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // Create a new DID in an Alias Output for us to modify.
  let (client, _, secret_manager, document): (Client, _, SecretManager, StardustDocument) = create_did::run().await?;

  // Resolve the latest state of the document.
  // Technically this is equivalent to the document above.
  let mut document: StardustDocument = client.resolve(document.id()).await?;

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

  // Resolve the latest output and update it with the given document.
  let alias_output: AliasOutput = client.update_did(document).await?;

  // Obtain the current byte costs and increase the required storage deposit
  // since the amount of stored bytes increased.
  let rent_structure = client.get_rent_structure().await?;
  let alias_output = AliasOutputBuilder::from(&alias_output)
    .with_minimum_storage_deposit(rent_structure)
    .finish()?;

  // Publish the output.
  let document: StardustDocument = client.publish_did_output(&secret_manager, alias_output).await?;

  println!("Published updated DID Document: {:#?}", document);

  Ok(())
}
