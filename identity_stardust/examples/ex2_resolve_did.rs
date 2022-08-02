// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_stardust::StardustClientExt;
use identity_stardust::StardustDocument;
use iota_client::Client;

mod ex0_create_did;

/// Demonstrate how to resolve an existing DID in an Alias Output.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let (client, _, _, document): (Client, _, _, StardustDocument) = ex0_create_did::run().await?;

  // Obtains the Alias Output and extracts the contained document.
  let resolved_doc: StardustDocument = client.resolve_did(document.id()).await?;

  assert_eq!(resolved_doc, document);

  println!("Resolved DID Document: {resolved_doc:#?}");

  Ok(())
}
