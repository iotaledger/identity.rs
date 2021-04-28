// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! An example that utilizes a diff and int chain to publish updates to a
//! DID Document.
//!
//! cargo run --example diff_chain

use identity::core::Timestamp;
use identity::did::MethodBuilder;
use identity::did::MethodData;
use identity::did::MethodRef;
use identity::did::MethodType;
use identity::iota::DocumentChain;
use identity::iota::DocumentDiff;
use identity::iota::IntChain;
use identity::prelude::*;
use std::thread::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
  // Create a new client connected to the Testnet (Chrysalis).
  let client: Client = Client::new().await?;
  // Keep track of the chain state locally, for reference
  let mut chain: DocumentChain;
  let mut keys: Vec<KeyPair> = Vec::new();

  // =========================================================================
  // Publish Initial Document
  // =========================================================================

  {
    let keypair: KeyPair = KeyPair::new_ed25519()?;
    let mut document: Document = Document::from_keypair(&keypair)?;
    document.sign(keypair.secret())?;
    document.publish(&client).await?;

    chain = DocumentChain::new(IntChain::new(document)?);
    keys.push(keypair);

    println!("Chain (1) > {:#}", chain);
    println!();
  }

  // =========================================================================
  // Publish Int Chain Update
  // =========================================================================

  sleep(Duration::from_secs(1));

  {
    let mut new: Document = chain.current().clone();
    let keypair: KeyPair = KeyPair::new_ed25519().unwrap();

    let authentication: MethodRef = MethodBuilder::default()
      .id(chain.id().join("#key-2")?.into())
      .controller(chain.id().clone().into())
      .key_type(MethodType::Ed25519VerificationKey2018)
      .key_data(MethodData::new_b58(keypair.public()))
      .build()
      .map(Into::into)?;

    unsafe {
      new.as_document_mut().authentication_mut().clear();
      new.as_document_mut().authentication_mut().append(authentication.into());
    }

    new.set_updated(Timestamp::now());
    new.set_previous_message_id(*chain.auth_message_id());

    chain.current().sign_data(&mut new, keys[0].secret())?;
    new.publish(&client).await?;

    keys.push(keypair);
    chain.try_push_auth(new)?;

    println!("Chain (2) > {:#}", chain);
    println!();
  }

  // =========================================================================
  // Publish Diff Chain Update
  // =========================================================================

  sleep(Duration::from_secs(1));

  {
    let new: Document = {
      let mut this: Document = chain.current().clone();
      this.properties_mut().insert("foo".into(), 123.into());
      this.properties_mut().insert("bar".into(), 456.into());
      this.set_updated(Timestamp::now());
      this
    };

    let message_id = *chain.diff_message_id();
    let mut diff: DocumentDiff = chain.current().diff(&new, message_id, keys[1].secret())?;

    diff.publish(chain.auth_message_id(), &client).await?;
    chain.try_push_diff(diff)?;
    let message_id2 = *chain.diff_message_id();

    println!("Chain (3) > {:#}", chain);
    let text: String = format!("Diff Message: {}", message_id2);
    println!("Diff Document Tx: {}", text);
  }

  // =========================================================================
  // Publish Phony Auth Update
  // =========================================================================

  sleep(Duration::from_secs(1));

  {
    let mut new: Document = chain.current().clone();
    let keypair: KeyPair = KeyPair::new_ed25519().unwrap();

    let authentication: MethodRef = MethodBuilder::default()
      .id(new.id().join("#bad-key")?.into())
      .controller(new.id().clone().into())
      .key_type(MethodType::Ed25519VerificationKey2018)
      .key_data(MethodData::new_b58(keypair.public()))
      .build()
      .map(Into::into)?;

    unsafe {
      new.as_document_mut().authentication_mut().clear();
      new.as_document_mut().authentication_mut().append(authentication.into());
    }

    new.set_updated(Timestamp::now());
    new.set_previous_message_id(*chain.auth_message_id());

    new.sign(keypair.secret())?;
    new.publish(&client).await?;

    println!("Chain Err > {:?}", chain.try_push_auth(new).unwrap_err());
  }

  // =========================================================================
  // Publish Second Diff Chain Update
  // =========================================================================

  sleep(Duration::from_secs(1));

  {
    let new: Document = {
      let mut this: Document = chain.current().clone();
      this.properties_mut().insert("baz".into(), 789.into());
      this.properties_mut().remove("bar");
      this.set_updated(Timestamp::now());
      this
    };

    let message_id = *chain.diff_message_id();
    let mut diff: DocumentDiff = chain.current().diff(&new, message_id, keys[1].secret())?;

    diff.publish(chain.auth_message_id(), &client).await?;
    chain.try_push_diff(diff)?;

    println!("Chain (4) > {:#}", chain);
    println!();
  }

  // =========================================================================
  // Read Document Chain with no query parameter given
  // No query parameter => Read out Int- and Diff Chain
  // =========================================================================

  let remote: DocumentChain = client.read_document_chain(chain.id()).await?;

  println!("Chain (R) {:#}", remote);
  println!();

  let a: &Document = chain.current();
  let b: &Document = remote.current();

  // The current document in the resolved chain should be identical to the
  // current document in our local chain.
  assert_eq!(a, b);

  // =========================================================================
  // Test Read Document Chain with diff true
  // query parameter diff=true => Read out Int- and Diff Chain
  // =========================================================================

  let mut did = chain.id().clone();
  let opt: Option<&str> = Some("diff=true");
  did.set_query(opt);

  // The flag diff=true and the did query parameter should be identical
  assert_eq!("diff=true", did.query().unwrap());

  let remote: DocumentChain = client.read_document_chain(&did).await?;

  println!("Chain (R) {:#}", remote);
  println!();

  let a: &Document = chain.current();
  let b: &Document = remote.current();

  // The current document in the resolved chain should be identical to the
  // current document in our local chain.
  assert_eq!(a, b);

  // =========================================================================
  // Test Read Document Chain with query parameter diff false
  // query parameter diff=false => Read Int Chain, Skip Diff Chain
  // Warning: this leads to an outdated version & is therefore not recommended
  // =========================================================================

  let opt: Option<&str> = Some("diff=false");
  did.set_query(opt);

  // The flag diff=false and the did query parameter should be identical
  assert_eq!("diff=false", did.query().unwrap());

  let remote: DocumentChain = client.read_document_chain(&did).await?;

  println!("Chain (R) {:#}", remote);
  println!();

  let a: &Document = chain.current();
  let b: &Document = remote.current();

  // The current document in the resolved chain should not be identical to
  // the current document in our local chain because you read an outdated
  // version of the document
  assert_ne!(a, b);

  Ok(())
}
