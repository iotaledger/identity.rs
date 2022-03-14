// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! An example that demonstrates how to publish changes to the integration chain to update a
//! DID Document.
//!
//! cargo run --example update_did

use identity::core::json;
use identity::core::FromJson;
use identity::core::Timestamp;
use identity::core::ToJson;
use identity::did::Service;
use identity::did::DID;
use identity::iota::ExplorerUrl;
use identity::iota::Receipt;
use identity::iota_core::DiffMessage;
use identity::iota_core::IotaService;
use identity::prelude::*;

mod create_did;

#[tokio::main]
async fn main() -> Result<()> {
  // Create a client instance to send messages to the Tangle.
  let client: Client = Client::new().await?;

  // Create a signed DID Document and KeyPair (see create_did.rs).
  let (document, keypair, receipt): (IotaDocument, KeyPair, Receipt) = create_did::run().await?;

  // Update the document by adding a new service endpoint
  let updated_document: IotaDocument = {
    let mut doc: IotaDocument = document.clone();

    // Add a Service
    let service: IotaService = Service::from_json_value(json!({
      "id": doc.id().to_url().join("#linked-domain-1")?,
      "type": "LinkedDomains",
      "serviceEndpoint": "https://example.com/"
    }))?;
    assert!(doc.insert_service(service));
    doc.metadata.updated = Timestamp::now_utc();
    doc
  };

  // Generate a signed diff object.
  let diff: DiffMessage = document.diff(
    &updated_document,
    *receipt.message_id(),
    keypair.private(),
    document.default_signing_method()?.id(),
  )?;

  println!("Diff > {}", diff.to_json_pretty().unwrap());

  // Publish the diff object to the Tangle, starting a diff chain.
  let update_receipt: Receipt = client.publish_diff(receipt.message_id(), &diff).await?;

  println!("Diff Update Receipt > {:#?}", update_receipt);

  // Display the web explorer url that shows the published diff message.
  let explorer: &ExplorerUrl = ExplorerUrl::mainnet();
  println!(
    "Diff Update Transaction > {}",
    explorer.message_url(update_receipt.message_id())?
  );
  println!("Explore the DID Document > {}", explorer.resolver_url(document.id())?);

  Ok(())
}
