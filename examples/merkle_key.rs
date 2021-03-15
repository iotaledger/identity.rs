// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example merkle_key

mod common;

use identity::core::BitSet;
use identity::core::FromJson;
use identity::core::ToJson;
use identity::core::Url;
use identity::credential::CredentialBuilder;
use identity::credential::Subject;
use identity::credential::VerifiableCredential;
use identity::crypto::merkle_key::Sha256;
use identity::crypto::merkle_tree::Proof;
use identity::crypto::KeyCollection;
use identity::crypto::KeyPair;
use identity::crypto::PublicKey;
use identity::crypto::SecretKey;
use identity::did::resolution::resolve;
use identity::did::resolution::Resolution;
use identity::did::MethodScope;
use identity::iota::Client;
use identity::iota::ClientBuilder;
use identity::iota::Document;
use identity::iota::Method;
use identity::iota::Result;
use identity::iota::TangleRef;
use rand::rngs::OsRng;
use rand::Rng;

const LEAVES: usize = 1 << 10;

#[tokio::main]
async fn main() -> Result<()> {
    // Create a new client connected to the Testnet (Chrysalis).
    // Node-syncing has to be disabled for now.
    let client: Client = ClientBuilder::new().node_sync_disabled().build().await?;


  // Create a new DID Document, signed and published.
  let (mut doc, auth): (Document, KeyPair) = common::document(&client).await?;

  // Generate a collection of ed25519 keys for signing credentials
  let keys: KeyCollection = KeyCollection::new_ed25519(LEAVES)?;

  // Generate a Merkle Key Collection Verification Method
  // with SHA-256 as  the digest algorithm.
  let method: Method = Method::create_merkle_key::<Sha256, _>(doc.id().clone(), &keys, "key-collection")?;

  // Append the new verification method to the set of existing methods
  doc.insert_method(MethodScope::VerificationMethod, method);

  // Sign and publish the updated document
  doc.set_previous_message_id(doc.message_id().clone());
  doc.sign(auth.secret())?;
  doc.publish(&client).await?;

  println!("document: {:#}", doc);

  // Create a Verifiable Credential
  let mut credential: VerifiableCredential = CredentialBuilder::default()
    .issuer(Url::parse(doc.id().as_str())?)
    .type_("MyCredential")
    .subject(Subject::from_json(r#"{"claim": true}"#)?)
    .build()
    .map(|credential| VerifiableCredential::new(credential, Vec::new()))?;

  println!("credential (unsigned): {:#}", credential);

  // Select a random key from the collection
  let index: usize = OsRng.gen_range(0..LEAVES);

  let public: &PublicKey = keys.public(index).unwrap();
  let secret: &SecretKey = keys.secret(index).unwrap();

  // Generate an inclusion proof for the selected key
  let proof: Proof<Sha256> = keys.merkle_proof(index).unwrap();

  // Sign the Verifiable Credential with the DID Document
  doc
    .signer(secret)
    .method("key-collection")
    .merkle_key((public, &proof))
    .sign(&mut credential)?;

  println!("credential (signed): {:#}", credential);

  let verified: Result<(), _> = doc.verifier().verify(&credential);

  println!("verified: {:?}", verified.is_ok());

  // Revoke the previously used key - assume it was compromised
  let mut revocation: BitSet = BitSet::new();

  revocation.insert(index as u32);

  unsafe {
    doc
      .as_document_mut()
      .try_resolve_mut("key-collection")?
      .properties_mut()
      .insert("revocation".into(), revocation.to_json_value()?);
  }

  // Publish the new document with the updated revocation state
  doc.set_previous_message_id(doc.message_id().clone());
  doc.sign(auth.secret())?;
  doc.publish(&client).await?;

  println!("document: {:#}", doc);

  // Set false claims about the credential subject
  let subject = credential.credential_subject.get_mut(0).unwrap();
  subject.properties.insert("claim".into(), false.into());
  subject.properties.insert("new-claim".into(), "not-false".into());

  // Sign the Credential with the compromised key
  doc
    .signer(secret)
    .method("key-collection")
    .merkle_key((public, &proof))
    .sign(&mut credential)?;

  println!("credential (compro-signed): {:#}", credential);

  // Resolve the DID and receive the latest document version
  let resolution: Resolution = resolve(doc.id().as_str(), Default::default(), &client).await?;
  let document: Document = resolution.document.map(Document::try_from_core).transpose()?.unwrap();

  println!("metadata: {:#?}", resolution.metadata);
  println!("document: {:#?}", document);

  // Check the verification status again - the credential SHOULD NOT be valid
  let verified: Result<(), _> = doc.verifier().verify(&credential);

  println!("verified: {:?}", verified.is_ok());

  Ok(())
}
