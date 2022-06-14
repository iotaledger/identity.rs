// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! An example that demonstrates how to publish changes to the integration chain to update a
//! DID Document.
//!
//! cargo run --example update_did

use identity_iota::core::json;
use identity_iota::core::FromJson;
use identity_iota::core::Timestamp;
use identity_iota::did::MethodScope;
use identity_iota::did::Service;
use identity_iota::did::DID;
use identity_iota::iota::ExplorerUrl;
use identity_iota::iota::Receipt;
use identity_iota::iota_core::IotaService;
use identity_iota::iota_core::IotaVerificationMethod;
use identity_iota::prelude::*;

mod create_did;

pub async fn run() -> Result<(IotaDocument, KeyPair, KeyPair, Receipt, Receipt)> {
  // Create a client instance to send messages to the Tangle.
  let client: Client = Client::new().await?;

  // Create a signed DID Document and KeyPair (see create_did.rs).
  let (mut document, keypair, receipt): (IotaDocument, KeyPair, Receipt) = create_did::run().await?;

  // Add a new VerificationMethod with a new keypair
  let new_key: KeyPair = KeyPair::new(KeyType::Ed25519)?;
  let method: IotaVerificationMethod =
    IotaVerificationMethod::new(document.id().clone(), new_key.type_(), new_key.public(), "newKey")?;
  assert!(document.insert_method(method, MethodScope::VerificationMethod).is_ok());

  // Add a new Service
  let service: IotaService = Service::from_json_value(json!({
    "id": document.id().to_url().join("#linked-domain")?,
    "type": "LinkedDomains",
    "serviceEndpoint": "https://iota.org"
  }))?;
  assert!(document.insert_service(service));

  // Add the messageId of the previous message in the chain.
  // This is REQUIRED in order for the messages to form a chain.
  // Skipping / forgetting this will render the publication useless.
  document.metadata.previous_message_id = *receipt.message_id();
  document.metadata.updated = Some(Timestamp::now_utc());

  // Sign the DID Document with the original private key.
  document.sign_self(keypair.private(), document.default_signing_method()?.id().clone())?;

  // Publish the updated DID Document to the Tangle.
  let update_receipt: Receipt = client.publish_document(&document).await?;

  println!("Publish Receipt > {:#?}", update_receipt);

  // Display the web explorer url that shows the published message.
  let explorer: &ExplorerUrl = ExplorerUrl::mainnet();
  println!(
    "DID Document Transaction > {}",
    explorer.message_url(update_receipt.message_id())?
  );
  println!("Explore the DID Document > {}", explorer.resolver_url(document.id())?);

  Ok((document, keypair, new_key, receipt, update_receipt))
}

#[tokio::main]
async fn main() -> Result<()> {
  let _ = run().await?;
  Ok(())
}
