// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! An example that demonstrates how to publish changes to the integration chain to update a
//! DID Document.
//!
//! cargo run --example update_did

use identity::core::json;
use identity::core::FromJson;
use identity::did::Service;
use identity::iota::Receipt;
use identity::iota::{ClientMap, DocumentDiff};
use identity::prelude::*;

mod create_did;

#[tokio::main]
async fn main() -> Result<()> {
  // Create a client instance to send messages to the Tangle.
  let client: ClientMap = ClientMap::new();

  // Create a signed DID Document and KeyPair (see create_did.rs).
  let (document, keypair, receipt): (IotaDocument, KeyPair, Receipt) = create_did::run().await?;

  // Update the document by adding a new service endpoint
  let updated_document: IotaDocument = {
    let mut doc: IotaDocument = document.clone();

    // Add a Service
    let service: Service = Service::from_json_value(json!({
      "id": doc.id().join("#linked-domain")?,
      "type": "LinkedDomains",
      "serviceEndpoint": "https://iota.org"
    }))?;
    assert!(doc.insert_service(service));
    doc
  };

  // Generate a signed diff object.
  let diff: DocumentDiff = document.diff(&updated_document, *receipt.message_id(), keypair.private())?;

  println!("Diff > {:#?}", diff);

  // Publish the diff object to the Tangle, starting a diff chain.
  let update_receipt: Receipt = client.publish_diff(receipt.message_id(), &diff).await?;

  println!("Diff Update Receipt > {:#?}", update_receipt);

  // Display the web explorer url that shows the published diff message.
  println!("Diff Transaction > {}", update_receipt.message_url()?);

  Ok(())
}
