// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! An example that demonstrates how to publish changes to the integration chain to update a
//! DID Document.
//!
//! cargo run --example update_did

use identity::core::FromJson;
use identity::core::{json, Timestamp};
use identity::did::MethodScope;
use identity::did::Service;
use identity::iota::IotaVerificationMethod;
use identity::iota::Receipt;
use identity::iota::{ClientMap, TangleRef};
use identity::prelude::*;

mod create_did;

pub async fn run() -> Result<(IotaDocument, KeyPair, KeyPair, Receipt, Receipt)> {
  // Create a client instance to send messages to the Tangle.
  let client: ClientMap = ClientMap::new();

  // Create a signed DID Document and KeyPair (see create_did.rs).
  let (mut document, keypair, receipt): (IotaDocument, KeyPair, Receipt) = create_did::run().await?;

  // Add a new VerificationMethod with a new keypair
  let new_key: KeyPair = KeyPair::new_ed25519()?;
  let method: IotaVerificationMethod = IotaVerificationMethod::from_did(document.did().clone(), &new_key, "newKey")?;
  assert!(document.insert_method(MethodScope::VerificationMethod, method));

  // Add a new Service
  let service: Service = Service::from_json_value(json!({
    "id": document.id().join("#linked-domain")?,
    "type": "LinkedDomains",
    "serviceEndpoint": "https://iota.org"
  }))?;
  assert!(document.insert_service(service));

  // Add the messageId of the previous message in the chain.
  // This is REQUIRED in order for the messages to form a chain.
  // Skipping / forgetting this will render the publication useless.
  document.set_previous_message_id(*receipt.message_id());
  document.set_updated(Timestamp::now_utc());

  // Sign the DID Document with the original private key.
  document.sign(keypair.private())?;

  // Publish the updated DID Document to the Tangle.
  let update_receipt: Receipt = client.publish_document(&document).await?;

  println!("Publish Receipt > {:#?}", update_receipt);

  // Display the web explorer url that shows the published message.
  println!("DID Document Transaction > {}", update_receipt.message_url()?);

  Ok((document, keypair, new_key, receipt, update_receipt))
}

#[tokio::main]
async fn main() -> Result<()> {
  let _ = run().await?;
  Ok(())
}
