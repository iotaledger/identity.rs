// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example merkle_key

use identity::core::BitSet;
use identity::core::FromJson;
use identity::core::Object;
use identity::core::ToJson;
use identity::core::Url;
use identity::credential::CredentialBuilder;
use identity::credential::Subject;
use identity::credential::VerifiableCredential;
use identity::crypto::merkle_tree::MTree;
use identity::crypto::merkle_tree::Proof;
use identity::crypto::KeyCollection;
use identity::crypto::MerkleKey;
use identity::crypto::MerkleKeySignerEd25519;
use identity::crypto::MerkleKeyVerifierEd25519;
use identity::crypto::PublicKey;
use identity::crypto::SecretKey;
use identity::crypto::SignatureOptions;
use identity::did::verifiable::Properties;
use identity::did::Document;
use identity::did::DocumentBuilder;
use identity::did::Method;
use identity::did::MethodBuilder;
use identity::did::MethodData;
use identity::did::MethodType;
use identity::did::DID;
use rand::rngs::OsRng;
use rand::Rng;
use sha2::Sha256;

type VerifiableDocument = Document<Properties, Object, ()>;

type Signer<'a> = MerkleKeySignerEd25519<'a, Sha256>;
type Verifier<'a> = MerkleKeyVerifierEd25519<'a, Sha256>;

const LEAVES: usize = 1 << 10;

fn main() {
  // Generate a collection of ed25519 keys
  let keys: KeyCollection = KeyCollection::new_ed25519(LEAVES).unwrap();

  // Construct a Merkle tree from the public keys
  let tree: MTree<Sha256> = keys.to_merkle_tree().unwrap();

  // Generate a Merkle Key Collection public key value with ed25519 as the
  // signature algorithm, SHA-256 as the digest algorithm, and the Merkle root
  // of the key collection
  let key_data: Vec<u8> = MerkleKey::encode_ed25519_key::<Sha256>(&tree);

  // Create a DID for the document owner
  let controller: DID = "did:example:123".parse().unwrap();

  // Create a `MerkleKeyCollection2021` verification method with
  // the previously generated public key value
  let method: Method<Object> = MethodBuilder::default()
    .id(controller.join("#key-collection").unwrap())
    .controller(controller.clone())
    .key_type(MethodType::MerkleKeyCollection2021)
    .key_data(MethodData::new_b58(&key_data))
    .build()
    .unwrap();

  println!("method: {:#}", method);

  // Create a DID Document with the `MerkleKeyCollection2021` method
  // as the sole verification method
  let mut document: VerifiableDocument = DocumentBuilder::default()
    .id(controller.clone())
    .verification_method(method)
    .build()
    .unwrap();

  println!("document: {:#}", document);

  // Create a Verifiable Credential
  let mut credential: VerifiableCredential = CredentialBuilder::default()
    .issuer(Url::parse(controller.as_str()).unwrap())
    .type_("MyCredential")
    .subject(Subject::from_json(r#"{"claim": true}"#).unwrap())
    .build()
    .map(|credential| VerifiableCredential::new(credential, Vec::new()))
    .unwrap();

  println!("credential (unsigned): {:#}", credential);

  // Select a random key from the collection
  let index: usize = OsRng.gen_range(0, LEAVES);
  let public: &PublicKey = keys[index].public();
  let secret: &SecretKey = keys[index].secret();

  println!("index: {}", index);

  // Generate an inclusion proof the the selected key
  let proof: Proof<Sha256> = tree.proof(index).unwrap();

  // Create a `Signer` and `SignatureOptions` with the Merkle proof and
  // a reference to the DID Document verification method
  let suite: Signer<'_> = Signer::new_ed25519(&proof);
  let options: SignatureOptions = document.resolve_options(0).unwrap();

  // Sign the Credential with the DID Document
  suite.sign(&mut credential, options, secret).unwrap();

  println!("credential (signed): {:#}", credential);

  // Create a verifier and check if the signature is valid
  let suite: Verifier<'_> = Verifier::new_ed25519(public);

  println!("verified: {:?}", document.verify_merkle_key(&credential, suite));

  // Revoke the previous key - assume it was compromised
  let mut revocation: BitSet = BitSet::new();
  revocation.insert(index as u32);

  // Update the DID Document with the new revocation state
  document
    .verification_method_mut()
    .head_mut()
    .unwrap()
    .properties_mut()
    .insert("revocation".into(), revocation.to_json_value().unwrap());

  // Set false claims about the credential subject
  let subject = credential.credential_subject.get_mut(0).unwrap();
  subject.properties.insert("claim".into(), false.into());
  subject.properties.insert("new-claim".into(), "not-false".into());

  // Generate a new signature using the same proof as before, which proves
  // existence of the compromised key
  let suite: Signer<'_> = Signer::new_ed25519(&proof);
  let options: SignatureOptions = document.resolve_options(0).unwrap();

  // Sign the Credential with the compromised key
  suite.sign(&mut credential, options, secret).unwrap();

  println!("credential (compro-signed): {:#}", credential);

  // Create a verifier and check if the signature is valid
  let suite: Verifier<'_> = Verifier::new_ed25519(public);

  println!("verified: {:?}", document.verify_merkle_key(&credential, suite));
}
