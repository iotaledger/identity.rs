// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Advanced example that performs multiple diff chain and integration chain updates and
//! demonstrates how to resolve the DID Document history to view these chains.
//!
//! cargo run --example did_history

use identity::core::json;
use identity::core::Timestamp;
use identity::crypto::KeyPair;
use identity::iota::Client;
use identity::iota::DiffSet;
use identity::iota::DocumentDiff;
use identity::iota::IotaDocument;
use identity::iota::IotaVerificationMethod;
use identity::iota::MessageHistory;
use identity::iota::MessageId;
use identity::iota::Receipt;
use identity::iota::Result;

mod create_did;

#[tokio::main]
async fn main() -> Result<()> {
  // Create a client instance to send messages to the Tangle.
  let client: Client = Client::new().await?;

  // ===========================================================================
  // DID Creation
  // ===========================================================================

  // Create a signed DID Document and KeyPair (see create_did.rs).
  let (mut document, keypair, original_receipt): (IotaDocument, KeyPair, Receipt) = create_did::run().await?;

  // Publish some unrelated spam messages to the same index as the DID on the Tangle.
  // These are not valid DID documents and are simply to demonstrate that invalid messages are
  // included in the history, potentially for debugging invalid DID documents.
  let index: &str = document.id().tag();
  client.publish_json(index, &json!({ "spam:1": true })).await?;
  client.publish_json(index, &json!({ "spam:2": true })).await?;
  client.publish_json(index, &json!({ "spam:3": true })).await?;
  client.publish_json(index, &json!({ "spam:4": true })).await?;
  client.publish_json(index, &json!({ "spam:5": true })).await?;

  // ===========================================================================
  // Integration Chain Update 1
  // ===========================================================================

  // Update the DID Document with new values and sign once again.
  document.properties_mut().insert("foo".into(), 123.into());
  // Setting the previous message id is REQUIRED in order for the messages to form a chain.
  document.set_previous_message_id(*original_receipt.message_id());
  document.set_updated(Timestamp::now_utc());
  document.sign(keypair.secret())?;

  // Publish the updated DID Document to the Tangle, updating the integration chain.
  let int1_update_receipt: Receipt = client.publish_document(&document).await?;

  // ===========================================================================
  // Diff Chain Update 1
  // ===========================================================================

  // Prepare a diff chain DID Document update.
  let update1: IotaDocument = {
    let mut new: IotaDocument = document.clone();
    new.properties_mut().insert("foo".into(), 0.into()); // change `foo`
    new.properties_mut().insert("bar".into(), 456.into()); // insert `bar`
    new.set_updated(Timestamp::now_utc());
    new
  };

  // Generate a signed diff object.
  //
  // This is the first diff therefore the `previous_message_id` property is
  // set to the last DID document published.
  let diff1: DocumentDiff = document.diff(&update1, *int1_update_receipt.message_id(), keypair.secret())?;
  // Publish the diff object to the Tangle, starting a diff chain.
  let diff1_update_receipt: Receipt = client.publish_diff(int1_update_receipt.message_id(), &diff1).await?;

  // ===========================================================================
  // Diff Chain Update 2
  // ===========================================================================

  // Prepare another diff chain update.
  let update2: IotaDocument = {
    let mut new: IotaDocument = document.clone();
    new.properties_mut().insert("baz".into(), 789.into()); // insert `baz`
    new.set_updated(Timestamp::now_utc());
    new
  };

  // Generate a signed diff object.
  //
  // This is the second diff therefore the `previous_message_id` property is
  // set to the first published diff object to keep the diff chain intact.
  let diff2: DocumentDiff = document.diff(&update2, *diff1_update_receipt.message_id(), keypair.secret())?;

  // Publish the diff object to the Tangle.
  // Note that we still use the message_id from the last integration chain message here.
  let _diff2_update_receipt: Receipt = client.publish_diff(int1_update_receipt.message_id(), &diff2).await?;

  // ===========================================================================
  // DID History 1
  // ===========================================================================

  // Retrieve the message history of the DID.
  let history1: MessageHistory = client.resolve_history(document.id()).await?;

  // The history shows one document in the integration chain (plus the current document), and two
  // diffs in the diff chain.
  println!("History (1) = {:#?}", history1);

  // ===========================================================================
  // Integration Chain Update 2
  // ===========================================================================

  // Publish an integration chain update, which writes the full updated DID document to the Tangle.
  document = update2;
  document.properties_mut().insert("foo".into(), 123456789.into());
  document.properties_mut().remove("bar");
  document.properties_mut().remove("baz");
  // Note: the previous_message_id points to the message_id of the last integration chain update,
  //       not the last diff chain message.
  document.set_previous_message_id(*int1_update_receipt.message_id());
  document.set_updated(Timestamp::now_utc());
  document.sign(keypair.secret())?;
  let _int2_update_receipt: Receipt = client.publish_document(&document).await?;

  // ===========================================================================
  // DID History 2
  // ===========================================================================

  // Retrieve the updated message history of the DID.
  let history2: MessageHistory = client.resolve_history(document.id()).await?;

  // The history shows two documents in the integration chain (plus the current document), and no
  // diffs in the diff chain. This is because the previous document published included those updates
  // and we have not added any diffs pointing to the latest document.
  println!("History (2) = {:#?}", history2);

  // Fetch the diff chain of the previous integration chain message.
  // We can still retrieve old diff chains, but they do not affect DID resolution.
  let method: &IotaVerificationMethod = document.authentication();
  let previous_integration_chain_message_id: &MessageId = int1_update_receipt.message_id();
  let previous_diffs: DiffSet = client.resolve_diffs(document.id(), method, previous_integration_chain_message_id).await?;

  println!("Diffs = {:#?}", previous_diffs);

  Ok(())
}
