// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! An example that utilizes a diff and integration chain to publish updates to a
//! DID Document.
//!
//! cargo run --example manipulate_did

mod create_did;

use identity::core::json;
use identity::core::FromJson;
use identity::did::MethodScope;
use identity::did::Service;
use identity::iota::ClientMap;
use identity::iota::IotaVerificationMethod;
use identity::iota::Receipt;
use identity::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
  // Create a client instance to send messages to the Tangle.
  let client: ClientMap = ClientMap::new();

  // Create a signed DID Document/KeyPair for the credential issuer (see create_did.rs).
  let (mut document, keypair, receipt): (IotaDocument, KeyPair, Receipt) = create_did::run().await?;

  // Add a new VerificationMethod with a new keypair
  let new_key: KeyPair = KeyPair::new_ed25519()?;
  let method: IotaVerificationMethod = IotaVerificationMethod::from_keypair(&new_key, "newKey")?;

  assert!(document.insert_method(MethodScope::VerificationMethod, method));

  // Add a new ServiceEndpoint
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

  // Sign the DID Document with the appropriate key.
  document.sign(keypair.secret())?;

  // Publish the updated DID Document to the Tangle.
  let receipt: Receipt = client.publish_document(&document).await?;

  println!("Publish Receipt > {:#?}", receipt);

  // Display the web explorer url that shows the published message.
  println!("DID Document Transaction > {}", receipt.message_url()?);

  Ok(())
}
