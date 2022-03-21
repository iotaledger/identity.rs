// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Demonstrates Elliptic-curve Diffie-Hellman (ECDH) cryptographic key exchange with DID Documents.
//!
//! cargo run --example key_exchange

use identity::crypto::KeyExchange;
use identity::crypto::KeyType;
use identity::crypto::X25519;
use identity::did::MethodScope;
use identity::iota::Receipt;
use identity::iota::ResolvedIotaDocument;
use identity::iota::TangleResolve;
use identity::iota_core::IotaDID;
use identity::iota_core::IotaVerificationMethod;
use identity::prelude::*;

mod create_did;

pub async fn run() -> Result<()> {
  // Alice and Bob want to communicate securely by encrypting their messages so only they
  // can read them. They both publish DID Documents with X25519 public keys and use them
  // to derive a shared secret key for encryption.
  let client: Client = Client::new().await?;

  // Alice creates and publishes their DID Document (see create_did and manipulate_did examples).
  let (alice_did, alice_x25519): (IotaDID, KeyPair) = {
    // Create a DID Document.
    let keypair: KeyPair = KeyPair::new(KeyType::Ed25519)?;
    let mut document: IotaDocument = IotaDocument::new(&keypair)?;

    // Insert a new X25519 KeyAgreement verification method.
    let x25519: KeyPair = KeyPair::new(KeyType::X25519)?;
    let method: IotaVerificationMethod =
      IotaVerificationMethod::new(document.id().clone(), KeyType::X25519, x25519.public(), "kex-0")?;
    document.insert_method(method, MethodScope::key_agreement())?;

    // Publish the DID Document.
    document.sign_self(keypair.private(), document.default_signing_method()?.id().clone())?;
    let _: Receipt = client.publish_document(&document).await?;
    (document.id().clone(), x25519)
  };

  // Bob creates and publishes their DID Document (see create_did and manipulate_did examples).
  let (bob_did, bob_x25519): (IotaDID, KeyPair) = {
    // Create a DID Document.
    let keypair: KeyPair = KeyPair::new(KeyType::Ed25519)?;
    let mut document: IotaDocument = IotaDocument::new(&keypair)?;

    // Insert a new X25519 KeyAgreement verification method.
    let x25519: KeyPair = KeyPair::new(KeyType::X25519)?;
    let method: IotaVerificationMethod =
      IotaVerificationMethod::new(document.id().clone(), KeyType::X25519, x25519.public(), "kex-0")?;
    document.insert_method(method, MethodScope::key_agreement())?;

    // Publish the DID Document.
    document.sign_self(keypair.private(), document.default_signing_method()?.id().clone())?;
    let _: Receipt = client.publish_document(&document).await?;
    (document.id().clone(), x25519)
  };

  // Alice and Bob tell each other their DIDs. They each resolve the DID Document of the other
  // to obtain their X25519 public key. Note that in practice, they would run this code completely
  // separately.

  let alice_shared_secret_key: [u8; 32] = {
    // Alice: resolves Bob's DID Document and extracts their public key.
    let bob_document: ResolvedIotaDocument = client.resolve(&bob_did).await?;
    let bob_method: &IotaVerificationMethod = bob_document
      .document
      .resolve_method_with_scope("kex-0", MethodScope::key_agreement())
      .unwrap();
    let bob_public_key: Vec<u8> = bob_method.data().try_decode()?;

    // Compute the shared secret.
    X25519::key_exchange(alice_x25519.private(), &bob_public_key)?
  };

  let bob_shared_secret_key: [u8; 32] = {
    // Bob: resolves Alice's DID Document and extracts their public key.
    let alice_document: ResolvedIotaDocument = client.resolve(&alice_did).await?;
    let alice_method: &IotaVerificationMethod = alice_document
      .document
      .resolve_method_with_scope("kex-0", MethodScope::key_agreement())
      .unwrap();
    let alice_public_key: Vec<u8> = alice_method.data().try_decode()?;

    // Compute the shared secret.
    X25519::key_exchange(bob_x25519.private(), &alice_public_key)?
  };

  // Both shared secret keys computed separately by Alice and Bob will match
  // and can then be used to establish encrypted communications.
  assert_eq!(alice_shared_secret_key, bob_shared_secret_key);
  println!("Diffie-Hellman key exchange successful!");

  Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
  let _ = run().await?;
  Ok(())
}
