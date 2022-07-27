// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::block::output::Output;
use iota_client::node_api::indexer::query_parameters::QueryParameter;
use iota_client::Client;

static ENDPOINT: &str = "https://api.testnet.shimmer.network/";

/// Demonstrate how to resolve an existing DID in an alias output.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let client: Client = Client::builder().with_primary_node(ENDPOINT, None)?.finish()?;

  println!(
    "{}",
    get_address_balance(
      &client,
      "rms1qrjwn7gc9fu88amxssxnkzqkxn80sfc29lwxnm0l2690z8kgd6yeqqatfd8"
    )
    .await
  );

  Ok(())
}

async fn get_address_balance(client: &Client, address: &str) -> u64 {
  let output_ids = client
    .basic_output_ids(vec![
      QueryParameter::Address(address.to_owned()),
      QueryParameter::HasExpirationCondition(false),
      QueryParameter::HasTimelockCondition(false),
      QueryParameter::HasStorageReturnCondition(false),
    ])
    .await
    .unwrap();

  let outputs_responses = client.get_outputs(output_ids).await.unwrap();

  let mut total_amount = 0;
  for output_response in outputs_responses {
    let output = Output::try_from(&output_response.output).unwrap();
    total_amount += output.amount();
  }

  total_amount
}
