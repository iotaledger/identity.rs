// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Advanced example that performs multiple diff chain and integration chain updates and
//! demonstrates how to resolve the DID Document history to view these chains.
//!
//! cargo run --example did_history

use identity::core::json;
use identity::core::FromJson;
use identity::core::Timestamp;
use identity::crypto::KeyPair;
use identity::did::MethodScope;
use identity::did::Service;
use identity::did::DID;
use identity::iota::ChainHistory;
use identity::iota::Client;
use identity::iota::DocumentHistory;
use identity::iota::Receipt;
use identity::iota::Result;
use identity::iota_core::DiffMessage;
use identity::iota_core::IotaDocument;
use identity::iota_core::IotaService;
use identity::iota_core::IotaVerificationMethod;

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

    // Add a new VerificationMethod with a new KeyPair, with the tag "keys-1"
    let keys_1: KeyPair = KeyPair::new_ed25519()?;
    let method_1: IotaVerificationMethod = IotaVerificationMethod::new(int_doc_1.id().clone(), keys_1.type_(), keys_1.public(), "keys-1")?;
    assert!(int_doc_1.insert_method(method_1, MethodScope::VerificationMethod).is_ok());

    // Add the `message_id` of the previous message in the chain.
    // This is REQUIRED in order for the messages to form a chain.
    // Skipping / forgetting this will render the publication useless.
    int_doc_1.metadata.previous_message_id = *original_receipt.message_id();
    int_doc_1.metadata.updated = Timestamp::now_utc();

    // Sign the DID Document with the original private key.
    int_doc_1.sign_self(keypair.private(), int_doc_1.default_signing_method()?.id().clone())?;

    int_doc_1
  };

  // Publish the updated DID Document to the Tangle, updating the integration chain.
  // This may take a few seconds to complete proof-of-work.
  let int_receipt_1: Receipt = client.publish_document(&int_doc_1).await?;

  // ===========================================================================
  // Diff Chain Update 1
  // ===========================================================================

  // Prepare a diff chain DID Document update, which writes only the changes to the Tangle.
  let diff_doc_1: IotaDocument = {
    let mut diff_doc_1: IotaDocument = int_doc_1.clone();

    // Add a new Service with the tag "linked-domain-1"
    let service: IotaService = Service::from_json_value(json!({
      "id": diff_doc_1.id().to_url().join("#linked-domain-1")?,
      "type": "LinkedDomains",
      "serviceEndpoint": "https://iota.org/"
    }))?;
    assert!(diff_doc_1.insert_service(service));
    diff_doc_1.metadata.updated = Timestamp::now_utc();
    diff_doc_1
  };

  // Create a signed diff update.
  //
  // This is the first diff therefore the `previous_message_id` property is
  // set to the last DID document published.
  let diff_1: DiffMessage = int_doc_1.diff(&diff_doc_1, *int_receipt_1.message_id(), keypair.private(), int_doc_1.default_signing_method()?.id())?;

  // Publish the diff to the Tangle, starting a diff chain.
  let diff_receipt_1: Receipt = client.publish_diff(int_receipt_1.message_id(), &diff_1).await?;

  // ===========================================================================
  // Diff Chain Update 2
  // ===========================================================================

  // Prepare another diff chain update.
  let diff_doc_2: IotaDocument = {
    let mut diff_doc_2: IotaDocument = diff_doc_1.clone();

    // Add a second Service with the tag "linked-domain-2"
    let service: IotaService = Service::from_json_value(json!({
      "id": diff_doc_2.id().to_url().join("#linked-domain-2")?,
      "type": "LinkedDomains",
      "serviceEndpoint": {
        "origins": ["https://iota.org/", "https://example.com/"]
      }
    }))?;
    diff_doc_2.insert_service(service);
    diff_doc_2.metadata.updated = Timestamp::now_utc();
    diff_doc_2
  };

  // This is the second diff therefore its `previous_message_id` property is
  // set to the first published diff to extend the diff chain.

  let diff_2: DiffMessage = diff_doc_1.diff(&diff_doc_2, *diff_receipt_1.message_id(), keypair.private(), diff_doc_1.default_signing_method()?.id())?;
  // Publish the diff to the Tangle.
  // Note that we still use the `message_id` from the last integration chain message here to link
  // the current diff chain to that point on the integration chain.
  let _diff_receipt_2: Receipt = client.publish_diff(int_receipt_1.message_id(), &diff_2).await?;

  // ===========================================================================
  // Diff Chain Spam
  // ===========================================================================

  // Publish several spam messages to the same index as the new diff chain on the Tangle.
  // These are not valid DID diffs and are simply to demonstrate that invalid messages
  // can be included in the history for debugging invalid DID diffs.
  let diff_index: &str = &IotaDocument::diff_index(int_receipt_1.message_id())?;
  client.publish_json(diff_index, &json!({ "diffSpam:1": true })).await?;
  client.publish_json(diff_index, &json!({ "diffSpam:2": true })).await?;
  client.publish_json(diff_index, &json!({ "diffSpam:3": true })).await?;

  // ===========================================================================
  // DID History 1
  // ===========================================================================

  // Retrieve the message history of the DID.
  let history_1: DocumentHistory = client.resolve_history(document.id()).await?;

  // The history shows two documents in the integration chain, and two diffs in the diff chain.
  println!("History (1) = {:#?}", history_1);

  // ===========================================================================
  // Integration Chain Update 2
  // ===========================================================================

  // Publish a second integration chain update
  let int_doc_2 = {
    let mut int_doc_2 = diff_doc_2.clone();

    // Remove the #keys-1 VerificationMethod
    int_doc_2.remove_method(&int_doc_2.id().to_url().join("#keys-1")?)?;

    // Remove the #linked-domain-1 Service
    int_doc_2.remove_service(&int_doc_2.id().to_url().join("#linked-domain-1")?)?;

    // Add a VerificationMethod with a new KeyPair, called "keys-2"
    let keys_2: KeyPair = KeyPair::new_ed25519()?;
    let method_2: IotaVerificationMethod = IotaVerificationMethod::new(int_doc_2.id().clone(), keys_2.type_(), keys_2.public(), "keys-2")?;
    assert!(int_doc_2.insert_method(method_2, MethodScope::VerificationMethod).is_ok());

    // Note: the `previous_message_id` points to the `message_id` of the last integration chain
    //       update, NOT the last diff chain message.
    int_doc_2.metadata.previous_message_id = *int_receipt_1.message_id();
    int_doc_2.metadata.updated = Timestamp::now_utc();

    int_doc_2.sign_self(keypair.private(), int_doc_2.default_signing_method()?.id().clone())?;
    int_doc_2
  };
  let _int_receipt_2: Receipt = client.publish_document(&int_doc_2).await?;

  // ===========================================================================
  // DID History 2
  // ===========================================================================

  // Retrieve the updated message history of the DID.
  let history_2: DocumentHistory = client.resolve_history(document.id()).await?;

  // The history now shows three documents in the integration chain, and no diffs in the diff chain.
  // This is because each integration chain document has its own diff chain but only the last one
  // is used during resolution.
  println!("History (2) = {:#?}", history_2);

  // ===========================================================================
  // Diff Chain History
  // ===========================================================================

  // Fetch the diff chain history of the previous integration chain document.
  // Old diff chains can be retrieved but they no longer affect DID resolution.
  let previous_integration_document = &history_2.integration_chain_data[1];
  let previous_diff_history: ChainHistory<DiffMessage> = client
    .resolve_diff_history(previous_integration_document)
    .await?;

  println!("Previous Diff History = {:#?}", previous_diff_history);

  Ok(())
}
