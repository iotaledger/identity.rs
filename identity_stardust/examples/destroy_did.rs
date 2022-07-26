// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use bee_api_types::responses::OutputResponse;
use identity_stardust::StardustDocument;
use iota_client::block::address::Address;
use iota_client::block::output::unlock_condition::AddressUnlockCondition;
use iota_client::block::output::AliasId;
use iota_client::block::output::BasicOutputBuilder;
use iota_client::block::output::Output;
use iota_client::block::output::OutputId;
use iota_client::block::output::UnlockCondition;
use iota_client::secret::SecretManager;
use iota_client::Client;

mod create_did;

/// Demonstrate how to destroy an existing DID in an alias output, reclaiming the stored deposit.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // Create a new DID in an alias output for us to modify.
  let (client, address, secret_manager, document): (Client, Address, SecretManager, StardustDocument) =
    create_did::run().await?;

  // Get the latest output id for the alias.
  let alias_id: AliasId = AliasId::from_str(document.id().tag())?;
  let output_id: OutputId = client.alias_output_id(alias_id).await?;
  let output_response: OutputResponse = client.get_output(&output_id).await?;
  let output: Output = Output::try_from(&output_response.output)?;

  // Consume the Alias Output containing the DID Document, sending its tokens to a new Basic Output.
  // WARNING: this destroys the DID Document and renders it permanently unrecoverable.
  let basic_output = BasicOutputBuilder::new_with_amount(output.amount())?
    .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)))
    .finish_output()?;

  // Send the transaction that destroys the alias output.
  client
    .block()
    .with_secret_manager(&secret_manager)
    .with_input(output_id.into())?
    .with_outputs(vec![basic_output])?
    .finish()
    .await?;

  Ok(())
}
