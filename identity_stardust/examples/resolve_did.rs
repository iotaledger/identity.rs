// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_stardust::StardustClientExt;
use identity_stardust::StardustDocument;
use iota_client::Client;

mod create_did;

/// Demonstrate how to resolve an existing DID in an Alias Output.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let (client, _, _, document): (Client, _, _, StardustDocument) = create_did::run().await?;

  let resolved_doc: StardustDocument = client.resolve(document.id()).await?;

  assert_eq!(resolved_doc, document);

  println!("Resolved DID Document: {resolved_doc:#?}");

  Ok(())
}
