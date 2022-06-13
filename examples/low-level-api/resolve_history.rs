// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Advanced example that performs multiple updates and demonstrates how to resolve the
//! DID Document history to view them.
//!
//! cargo run --example did_history

use identity::core::json;
use identity::core::FromJson;
use identity::core::Timestamp;
use identity::crypto::KeyPair;
use identity::did::MethodScope;
use identity::did::Service;
use identity::did::DID;
use identity::iota::Client;
use identity::iota::DocumentHistory;
use identity::iota::Receipt;
use identity::iota::Result;
use identity::iota_core::IotaDocument;
use identity::iota_core::IotaService;
use identity::iota_core::IotaVerificationMethod;
use identity::prelude::*;

mod create_did;

#[rustfmt::skip]
#[tokio::main]
async fn main() -> Result<()> {
  // Create a client instance to send messages to the Tangle.
  let client: Client = Client::new().await?;

  // ===========================================================================
  // DID Creation
  // ===========================================================================

  // Create a signed DID Document and KeyPair (see "create_did.rs" example).
  let (document, keypair, original_receipt): (IotaDocument, KeyPair, Receipt) = create_did::run().await?;

  // ===========================================================================
  // Integration Chain Spam
  // ===========================================================================

  // Publish several spam messages to the same index as the integration chain on the Tangle.
  // These are not valid DID messages and are simply to demonstrate that invalid messages
  // can be included in the history for debugging invalid DID documents.
  let int_index: &str = document.integration_index();
  client.publish_json(int_index, &json!({ "intSpam:1": true })).await?;
  client.publish_json(int_index, &json!({ "intSpam:2": true })).await?;
  client.publish_json(int_index, &json!({ "intSpam:3": true })).await?;
  client.publish_json(int_index, &json!({ "intSpam:4": true })).await?;
  client.publish_json(int_index, &json!({ "intSpam:5": true })).await?;

  // ===========================================================================
  // Integration Chain Update 1
  // ===========================================================================

  // Prepare an integration chain update, which writes the full updated DID document to the Tangle.
  let int_doc_1 = {
    let mut int_doc_1 = document.clone();

    // Add a new Service with the tag "linked-domain-1".
    let service: IotaService = Service::from_json_value(json!({
      "id": int_doc_1.id().to_url().join("#linked-domain-1")?,
      "type": "LinkedDomains",
      "serviceEndpoint": "https://iota.org/"
    }))?;
    assert!(int_doc_1.insert_service(service));

    // Add a second Service with the tag "linked-domain-2".
    let service: IotaService = Service::from_json_value(json!({
      "id": int_doc_1.id().to_url().join("#linked-domain-2")?,
      "type": "LinkedDomains",
      "serviceEndpoint": {
        "origins": ["https://iota.org/", "https://example.com/"]
      }
    }))?;
    assert!(int_doc_1.insert_service(service));

    // Add a new VerificationMethod with a new KeyPair, with the tag "keys-1"
    let keys_1: KeyPair = KeyPair::new(KeyType::Ed25519)?;
    let method_1: IotaVerificationMethod = IotaVerificationMethod::new(int_doc_1.id().clone(), keys_1.type_(), keys_1.public(), "keys-1")?;
    assert!(int_doc_1.insert_method(method_1, MethodScope::VerificationMethod).is_ok());

    // Add the `message_id` of the previous message in the chain.
    // This is REQUIRED in order for the messages to form a chain.
    // Skipping / forgetting this will render the publication useless.
    int_doc_1.metadata.previous_message_id = *original_receipt.message_id();
    int_doc_1.metadata.updated = Some(Timestamp::now_utc());

    // Sign the DID Document with the original private key.
    int_doc_1.sign_self(keypair.private(), int_doc_1.default_signing_method()?.id().clone())?;

    int_doc_1
  };

  // Publish the updated DID Document to the Tangle, updating the integration chain.
  // This may take a few seconds to complete proof-of-work.
  let int_receipt_1: Receipt = client.publish_document(&int_doc_1).await?;

  // ===========================================================================
  // DID History 1
  // ===========================================================================

  // Retrieve the message history of the DID.
  let history_1: DocumentHistory = client.resolve_history(document.id()).await?;

  // The history shows two documents in the integration chain.
  println!("History (1) = {:#?}", history_1);

  // ===========================================================================
  // Integration Chain Update 2
  // ===========================================================================

  // Publish a second integration chain update
  let int_doc_2 = {
    let mut int_doc_2 = int_doc_1.clone();

    // Remove the #keys-1 VerificationMethod
    int_doc_2.remove_method(&int_doc_2.id().to_url().join("#keys-1")?)?;

    // Remove the #linked-domain-1 Service
    int_doc_2.remove_service(&int_doc_2.id().to_url().join("#linked-domain-1")?);

    // Add a VerificationMethod with a new KeyPair, called "keys-2"
    let keys_2: KeyPair = KeyPair::new(KeyType::Ed25519)?;
    let method_2: IotaVerificationMethod = IotaVerificationMethod::new(int_doc_2.id().clone(), keys_2.type_(), keys_2.public(), "keys-2")?;
    assert!(int_doc_2.insert_method(method_2, MethodScope::VerificationMethod).is_ok());

    // Note: the `previous_message_id` points to the `message_id` of the last integration chain
    //       update.
    int_doc_2.metadata.previous_message_id = *int_receipt_1.message_id();
    int_doc_2.metadata.updated = Some(Timestamp::now_utc());

    int_doc_2.sign_self(keypair.private(), int_doc_2.default_signing_method()?.id().clone())?;
    int_doc_2
  };
  let _int_receipt_2: Receipt = client.publish_document(&int_doc_2).await?;

  // ===========================================================================
  // DID History 2
  // ===========================================================================

  // Retrieve the updated message history of the DID.
  let history_2: DocumentHistory = client.resolve_history(document.id()).await?;

  // The history now shows three documents in the integration chain.
  println!("History (2) = {:#?}", history_2);

  Ok(())
}
