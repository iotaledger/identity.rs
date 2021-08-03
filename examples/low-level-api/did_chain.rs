// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example did_chain

mod create_did;

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

#[tokio::main]
async fn main() -> Result<()> {
  // Create a client instance to send messages to the Tangle.
  let client: Client = Client::new().await?;
  let keypair: KeyPair = KeyPair::new_ed25519()?;

  // Create a new DID Document from the generated key pair.
  let mut document: IotaDocument = IotaDocument::from_keypair(&keypair)?;

  // Sign the DID Document
  document.sign(keypair.secret())?;

  // Publish the DID Document to the Tangle
  let receipt: Receipt = client.publish_document(&document).await?;

  // Publish some spam
  let index: &str = document.id().tag();
  client.publish_json(index, &json!({ "spam:1": true })).await?;
  client.publish_json(index, &json!({ "spam:2": true })).await?;
  client.publish_json(index, &json!({ "spam:3": true })).await?;
  client.publish_json(index, &json!({ "spam:4": true })).await?;
  client.publish_json(index, &json!({ "spam:5": true })).await?;

  // Update the DID Document with newly added values and sign once again
  document.properties_mut().insert("foo".into(), 123.into());
  // document.set_message_id(*receipt.message_id());
  document.set_previous_message_id(*receipt.message_id());
  document.set_updated(Timestamp::now_utc());
  document.sign(keypair.secret())?;

  // Publish the updated DID Document to the integration chain address
  let receipt: Receipt = client.publish_document(&document).await?;

  // Prepare a diff chain DID Document update
  let update: IotaDocument = {
    let mut new: IotaDocument = document.clone();
    new.properties_mut().insert("foo".into(), 0.into()); // change `foo`
    new.properties_mut().insert("bar".into(), 456.into()); // insert `bar`
    new.set_updated(Timestamp::now_utc());
    new
  };

  // Generate a signed diff object.
  //
  // This is the first diff therefore the `previous_message_id` property is
  // set to the authentication message
  let diff: DocumentDiff = document.diff(&update, *receipt.message_id(), keypair.secret())?;

  // Publish the diff object to the Tangle
  let diff_receipt: Receipt = client.publish_diff(receipt.message_id(), &diff).await?;

  // Prepare another diff chain update
  let update: IotaDocument = {
    let mut new: IotaDocument = document.clone();
    new.properties_mut().insert("baz".into(), 789.into()); // insert `baz`
    new.set_updated(Timestamp::now_utc());
    new
  };

  // Generate a signed diff object.
  //
  // This is the second diff therefore the `previous_message_id` property is
  // set to the previously published diff object
  let diff: DocumentDiff = document.diff(&update, *diff_receipt.message_id(), keypair.secret())?;

  // Publish the diff object to the Tangle
  let _diff_receipt: Receipt = client.publish_diff(receipt.message_id(), &diff).await?;

  // Retrieve the message history of the DID
  let history: MessageHistory = client.resolve_history(document.id()).await?;

  println!("History (1) = {:#?}", history);

  // Publish another integration chain update
  document = update;
  document.properties_mut().insert("foo".into(), 123456789.into());
  document.properties_mut().remove("bar");
  document.properties_mut().remove("baz");
  document.set_previous_message_id(*receipt.message_id());
  document.set_updated(Timestamp::now_utc());
  document.sign(keypair.secret())?;

  // Send the updated DID Document to the Tangle
  let _receipt: Receipt = client.publish_document(&document).await?;

  // Retrieve the updated message history of the DID
  let history: MessageHistory = client.resolve_history(document.id()).await?;

  println!("History (2) = {:#?}", history);

  // Retrieve the diff chain of the previous integration message
  let method: &IotaVerificationMethod = document.authentication();
  let target: &MessageId = receipt.message_id();
  let diffs: DiffSet = client.resolve_diffs(document.id(), method, target).await?;

  println!("Diffs = {:#?}", diffs);

  Ok(())
}
