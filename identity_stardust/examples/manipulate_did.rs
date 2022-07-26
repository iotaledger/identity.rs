// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use bee_api_types::responses::OutputResponse;
use identity_did::did::DID;
use identity_stardust::StardustClientExt;
use identity_stardust::StardustDocument;
use iota_client::block::output::AliasId;
use iota_client::block::output::AliasOutput;
use iota_client::block::output::AliasOutputBuilder;
use iota_client::block::output::Output;
use iota_client::block::output::OutputId;
use iota_client::block::output::RentStructure;
use iota_client::secret::SecretManager;
use iota_client::Client;

mod create_did2;

/// Demonstrate how to modify a DID document in an existing alias output.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // Create a new DID in an alias output for us to modify.
  let (client, _, secret_manager, mut document): (Client, _, SecretManager, StardustDocument) =
    create_did2::run().await?;

  // Modify the document.
  // Here we modify the document we just published.
  // Alternatively, we can resolve the latest document and modify that.
  document.attach_method_relationship(
    &document.id().to_url().join("#key-1")?,
    identity_did::verification::MethodRelationship::CapabilityDelegation,
  )?;

  // Convert the DID into an AliasId to retrieve the output.
  let alias_id: AliasId = AliasId::from_str(document.id().tag())?;
  let output_id: OutputId = client.alias_output_id(alias_id).await?;
  let output_response: OutputResponse = client.get_output(&output_id).await?;
  let output: Output = Output::try_from(&output_response.output)?;

  let alias_output: AliasOutput = if let Output::Alias(alias_output) = output {
    alias_output
  } else {
    anyhow::bail!("not an alias output");
  };

  // Obtain the current byte costs.
  let rent_structure: RentStructure = client.get_rent_structure().await?;

  // Create a new builder based on the previous output.
  let mut alias_output_builder: AliasOutputBuilder = AliasOutputBuilder::from(&alias_output)
    // Recalculates the required storage deposit if necessary.
    .with_minimum_storage_deposit(rent_structure)
    // State updates require us to increment the state index.
    .with_state_index(alias_output.state_index() + 1)
    // Set the updated document which overwrites the previous one.
    .with_state_metadata(document.pack()?);

  // Set the alias id if it's not yet set.
  // This is only necessary on the first update to an existing alias output.
  if alias_output.alias_id().is_null() {
    alias_output_builder = alias_output_builder.with_alias_id(alias_id);
  }

  let updated_alias_output: Output = alias_output_builder.finish_output()?;

  // Publish the updated output, which consumes the previous one.
  let block = client
    .publish_outputs(&secret_manager, vec![updated_alias_output])
    .await?;

  let documents: Vec<StardustDocument> = client.documents_from_block(&block).await?;

  println!("Published updated DID Document: {:#?}", documents[0]);

  Ok(())
}
