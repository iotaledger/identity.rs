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
use identity::crypto::merkle_key::MerkleKey;
use identity::crypto::merkle_tree::Hash;
use identity::crypto::merkle_tree::Proof;
use identity::crypto::KeyCollection;
use identity::crypto::KeyPair;
use identity::did::resolution::resolve;
use identity::did::resolution::Resolution;
use identity::did::verifiable::Public;
use identity::did::verifiable::Secret;
use identity::did::Method;
use identity::did::MethodBuilder;
use identity::did::MethodData;
use identity::did::MethodType;
use identity::iota::Client;
use identity::iota::Document;
use identity::iota::Result;
use identity::iota::TangleRef;
use rand::rngs::OsRng;
use rand::Rng;
use sha2::Sha256;

const LEAVES: usize = 1 << 10;

#[smol_potat::main]
async fn main() -> Result<()> {
  let client: Client = Client::new()?;

  // Create a new DID Document, signed and published.
  let (mut doc, auth): (Document, KeyPair) = common::document(&client).await?;

  // Generate a collection of ed25519 keys for signing credentials
  let keys: KeyCollection = KeyCollection::new_ed25519(LEAVES)?;

  // Compute the Merkle root hash of the public keys
  let merkle_root: Hash<Sha256> = keys.merkle_root();
  let key_data: Vec<u8> = MerkleKey::encode_ed25519_key::<Sha256>(&merkle_root);

  // Generate a Merkle Key Collection public key value with ed25519 as the
  // signature algorithm, SHA-256 as the digest algorithm, and the Merkle
  // root of the key collection - This is expressed as the public key value
  // of the `MerkleKeyCollection2021` verification method.
  let method: Method = MethodBuilder::default()
    .id(doc.id().as_ref().join("#key-collection")?)
    .controller(doc.id().as_ref().clone())
    .key_type(MethodType::MerkleKeyCollection2021)
    .key_data(MethodData::new_b58(&key_data))
    .build()?;

  // Append the new verification method to the set of existing methods
  unsafe {
    doc.as_document_mut().verification_method_mut().append(method.into());
  }

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
  let index: usize = OsRng.gen_range(0, LEAVES);

  // Generate an inclusion proof for the selected key
  let proof: Proof<Sha256> = keys.merkle_proof(index).unwrap();

  let secret: &[u8] = keys.secret(index).unwrap().as_ref();
  let secret: Secret<'_> = Secret::with_merkle_proof(secret, &proof);

  // Sign the Credential with the DID Document
  doc.sign_that(&mut credential, "#key-collection", secret)?;

  println!("credential (signed): {:#}", credential);

  let public: Public<'_> = Public::with_merkle_target(keys[index].as_ref());
  let verified: Result<(), _> = doc.verify_that(&credential, public);

  println!("verified: {:?}", verified);

  // Revoke the previously used key - assume it was compromised
  let mut revocation: BitSet = BitSet::new();

  revocation.insert(index as u32);

  unsafe {
    doc
      .as_document_mut()
      .verification_method_mut()
      .tail_mut()
      .unwrap()
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
  doc.sign_that(&mut credential, "#key-collection", secret)?;

  println!("credential (compro-signed): {:#}", credential);

  // Resolve the DID and receive the latest document version
  let resolution: Resolution = resolve(doc.id().as_str(), Default::default(), &client).await?;

  let document: Document = resolution
    .document
    .map(Document::try_from_document)
    .transpose()?
    .unwrap();

  println!("metadata: {:#?}", resolution.metadata);
  println!("document: {:#?}", document);

  let public: Public<'_> = Public::with_merkle_target(keys[index].as_ref());
  let verified: Result<(), _> = doc.verify_that(&credential, public);

  println!("verified: {:?}", verified);

  Ok(())
}
