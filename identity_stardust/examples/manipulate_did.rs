// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_did::did::DID;
use identity_did::verification::MethodRelationship;
use identity_stardust::StardustClientExt;
use identity_stardust::StardustDocument;
use iota_client::block::output::AliasOutput;
use iota_client::block::output::AliasOutputBuilder;
use iota_client::secret::SecretManager;
use iota_client::Client;

mod create_did;

/// Demonstrate how to modify a DID document in an existing alias output.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // Create a new DID in an alias output for us to modify.
  let (client, _, secret_manager, mut document): (Client, _, SecretManager, StardustDocument) =
    create_did::run().await?;

  // Modify the document.
  // Here we modify the document we just published.
  // Alternatively, we can resolve the latest document and modify that.
  document.attach_method_relationship(
    &document.id().to_url().join("#key-1")?,
    MethodRelationship::CapabilityDelegation,
  )?;

  // Resolve the latest output and update it with the given document.
  let alias_output: AliasOutput = client.update_did(document).await?;

  let rent_structure = client.get_rent_structure().await?;

  let alias_output = AliasOutputBuilder::from(&alias_output)
    .with_minimum_storage_deposit(rent_structure)
    .finish()?;

  // Publish the output.
  let document: StardustDocument = client.publish_did_output(&secret_manager, alias_output).await?;

  println!("Published updated DID Document: {:#?}", document);

  Ok(())
}
