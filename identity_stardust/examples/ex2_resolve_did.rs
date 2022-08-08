// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::block::output::AliasOutput;
use iota_client::Client;

use identity_stardust::StardustDID;
use identity_stardust::StardustDocument;
use identity_stardust::StardustIdentityClientExt;

mod ex0_create_did;

/// Demonstrates how to resolve an existing DID in an Alias Output.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let (client, _, _, did): (Client, _, _, StardustDID) = ex0_create_did::run().await?;

  // Resolve the associated Alias Output and extract the DID document from it.
  let resolved: StardustDocument = client.resolve_did(&did).await?;
  println!("Resolved DID Document: {:#}", resolved);

  // We can also resolve the Alias Output directly.
  let alias_output: AliasOutput = client.resolve_did_output(&did).await?;
  println!("The Alias Output holds {} tokens", alias_output.amount());

  Ok(())
}
