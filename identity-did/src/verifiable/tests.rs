// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Timestamp;
use identity_core::crypto::merkle_key::MerkleKey;
use identity_core::crypto::merkle_key::Sha256;
use identity_core::crypto::merkle_tree::Hash;
use identity_core::crypto::merkle_tree::Proof;
use identity_core::crypto::Ed25519;
use identity_core::crypto::KeyCollection;
use identity_core::crypto::KeyPair;
use identity_core::crypto::PrivateKey;
use identity_core::crypto::ProofPurpose;
use identity_core::crypto::PublicKey;
use identity_core::crypto::SetSignature;
use identity_core::crypto::Signature;
use identity_core::crypto::TrySignature;
use identity_core::crypto::TrySignatureMut;

use crate::did::CoreDID;
use crate::did::DID;
use crate::document::CoreDocument;
use crate::verification::MethodData;
use crate::verification::MethodRelationship;
use crate::verification::MethodScope;
use crate::verification::MethodType;
use crate::verification::MethodUriType;
use crate::verification::TryMethod;
use crate::verification::VerificationMethod;

#[derive(Debug, Serialize)]
struct MockObject {
  data: u32,
  #[serde(skip_serializing_if = "Option::is_none")]
  proof: Option<Signature>,
}

impl MockObject {
  fn new(data: u32) -> Self {
    Self { data, proof: None }
  }
}

impl TrySignature for MockObject {
  fn signature(&self) -> Option<&Signature> {
    self.proof.as_ref()
  }
}

impl TrySignatureMut for MockObject {
  fn signature_mut(&mut self) -> Option<&mut Signature> {
    self.proof.as_mut()
  }
}

impl SetSignature for MockObject {
  fn set_signature(&mut self, signature: Signature) {
    self.proof = Some(signature);
  }
}

impl TryMethod for MockObject {
  const TYPE: MethodUriType = MethodUriType::Relative;
}

// ===========================================================================
// ===========================================================================

#[test]
fn test_sign_verify_data_ed25519() {
  for method_data_base in [MethodData::new_b58, MethodData::new_multibase] {
    let key: KeyPair = KeyPair::new_ed25519().unwrap();
    let controller: CoreDID = "did:example:1234".parse().unwrap();
    let public_key = key.public().as_ref().to_vec();

    let method: VerificationMethod = VerificationMethod::builder(Default::default())
      .id(controller.to_url().join("#key-1").unwrap())
      .controller(controller.clone())
      .key_type(MethodType::Ed25519VerificationKey2018)
      .key_data(method_data_base(public_key))
      .build()
      .unwrap();

    let document: CoreDocument = CoreDocument::builder(Default::default())
      .id(controller)
      .verification_method(method)
      .build()
      .unwrap();

    let mut data: MockObject = MockObject::new(123);

    assert!(document.verifier().verify(&data).is_err());

    document.signer(key.private()).method("#key-1").sign(&mut data).unwrap();

    assert!(document.verifier().verify(&data).is_ok());
  }
}

#[test]
fn test_sign_verify_data_merkle_key_ed25519_sha256() {
  for method_data_base in [MethodData::new_b58, MethodData::new_multibase] {
    let total: usize = 1 << 11;
    let index: usize = 1 << 9;

    let keys: KeyCollection = KeyCollection::new_ed25519(total).unwrap();
    let controller: CoreDID = "did:example:1234".parse().unwrap();

    let root: Hash<Sha256> = keys.merkle_root();
    let proof: Proof<Sha256> = keys.merkle_proof(index).unwrap();
    let mkey: Vec<u8> = MerkleKey::encode_key::<Sha256, Ed25519>(&root);

    let method: VerificationMethod = VerificationMethod::builder(Default::default())
      .id(controller.to_url().join("#key-collection").unwrap())
      .controller(controller.clone())
      .key_type(MethodType::MerkleKeyCollection2021)
      .key_data(method_data_base(mkey))
      .build()
      .unwrap();

    let document: CoreDocument = CoreDocument::builder(Default::default())
      .id(controller)
      .verification_method(method)
      .build()
      .unwrap();

    let public: &PublicKey = keys.public(index).unwrap();
    let private: &PrivateKey = keys.private(index).unwrap();

    let mut data: MockObject = MockObject::new(123);

    assert!(document.verifier().verify(&data).is_err());

    document
      .signer(private)
      .method("#key-collection")
      .merkle_key((public, &proof))
      .sign(&mut data)
      .unwrap();

    assert!(document.verifier().verify(&data).is_ok());
  }
}

// ===========================================================================
// Test DocumentVerifier
// ===========================================================================

fn setup() -> (KeyPair, CoreDocument) {
  let key: KeyPair = KeyPair::new_ed25519().unwrap();
  let controller: CoreDID = "did:example:1234".parse().unwrap();
  let public_key = key.public().as_ref().to_vec();

  let method: VerificationMethod = VerificationMethod::builder(Default::default())
    .id(controller.to_url().join("#key-1").unwrap())
    .controller(controller.clone())
    .key_type(MethodType::Ed25519VerificationKey2018)
    .key_data(MethodData::new_multibase(public_key))
    .build()
    .unwrap();

  let document: CoreDocument = CoreDocument::builder(Default::default())
    .id(controller)
    .verification_method(method)
    .build()
    .unwrap();

  (key, document)
}

#[test]
fn test_sign_verify_method_type() {
  let (key, document) = setup();
  let mut data: MockObject = MockObject::new(123);
  assert!(document.verifier().verify(&data).is_err());

  // Sign.
  document.signer(key.private()).method("#key-1").sign(&mut data).unwrap();

  // VALID: verifying without checking the method type succeeds.
  document.verifier().verify(&data).unwrap();
  document.verifier().method_type(vec![]).verify(&data).unwrap();
  // VALID: verifying with the correct method type succeeds.
  document
    .verifier()
    .method_type(vec![MethodType::Ed25519VerificationKey2018])
    .verify(&data)
    .unwrap();
  document
    .verifier()
    .method_type(vec![
      MethodType::Ed25519VerificationKey2018,
      MethodType::MerkleKeyCollection2021,
    ])
    .verify(&data)
    .unwrap();
  document
    .verifier()
    .method_type(vec![
      MethodType::MerkleKeyCollection2021,
      MethodType::Ed25519VerificationKey2018,
    ])
    .verify(&data)
    .unwrap();

  // INVALID: verifying with the wrong method type fails.
  assert!(document
    .verifier()
    .method_type(vec![MethodType::MerkleKeyCollection2021])
    .verify(&data)
    .is_err());
}

#[test]
fn test_sign_verify_method_scope() {
  let (key, document) = setup();
  let mut data: MockObject = MockObject::new(123);
  assert!(document.verifier().verify(&data).is_err());

  // Sign.
  document.signer(key.private()).method("#key-1").sign(&mut data).unwrap();
  // VALID: verifying without checking the method scope succeeds.
  document.verifier().verify(&data).unwrap();
  // VALID: verifying with the correct method scope succeeds.
  document
    .verifier()
    .method_scope(MethodScope::VerificationMethod)
    .verify(&data)
    .unwrap();

  // INVALID: verifying with the wrong method scope fails.
  for relationship in [
    MethodRelationship::AssertionMethod,
    MethodRelationship::CapabilityDelegation,
    MethodRelationship::CapabilityInvocation,
    MethodRelationship::KeyAgreement,
    MethodRelationship::Authentication,
  ] {
    assert!(document
      .verifier()
      .method_scope(MethodScope::VerificationRelationship(relationship))
      .verify(&data)
      .is_err());
  }
}

#[test]
fn test_sign_verify_challenge() {
  let (key, document) = setup();
  let mut data: MockObject = MockObject::new(123);
  assert!(document.verifier().verify(&data).is_err());

  // Sign with a challenge.
  document
    .signer(key.private())
    .method("#key-1")
    .challenge("some-challenge".to_string())
    .sign(&mut data)
    .unwrap();
  assert_eq!(data.proof.clone().unwrap().challenge.unwrap(), "some-challenge");

  // VALID: verifying without checking the challenge succeeds.
  document.verifier().verify(&data).unwrap();
  // VALID: verifying with the correct challenge succeeds.
  document
    .verifier()
    .challenge("some-challenge".into())
    .verify(&data)
    .unwrap();

  // INVALID: verifying with the wrong challenge fails.
  assert!(document.verifier().challenge("invalid".into()).verify(&data).is_err());
  assert!(document.verifier().challenge(" ".into()).verify(&data).is_err());
  assert!(document.verifier().challenge("".into()).verify(&data).is_err());
}

#[test]
fn test_sign_verify_domain() {
  let (key, document) = setup();
  let mut data: MockObject = MockObject::new(123);
  assert!(document.verifier().verify(&data).is_err());

  // Sign with a domain.
  document
    .signer(key.private())
    .method("#key-1")
    .domain("some.domain".to_string())
    .sign(&mut data)
    .unwrap();
  assert_eq!(data.proof.clone().unwrap().domain.unwrap(), "some.domain");

  // VALID: verifying without checking the domain succeeds.
  document.verifier().verify(&data).unwrap();
  // VALID: verifying with the correct domain succeeds.
  document.verifier().domain("some.domain".into()).verify(&data).unwrap();

  // INVALID: verifying with the wrong domain fails.
  assert!(document.verifier().domain("invalid".into()).verify(&data).is_err());
  assert!(document.verifier().domain(" ".into()).verify(&data).is_err());
  assert!(document.verifier().domain("".into()).verify(&data).is_err());
}

#[test]
fn test_sign_verify_purpose() {
  let (key, mut document) = setup();
  let mut data: MockObject = MockObject::new(123);
  assert!(document.verifier().verify(&data).is_err());
  document
    .attach_method_relationship("#key-1", MethodRelationship::Authentication)
    .unwrap();

  // Sign with a purpose.
  document
    .signer(key.private())
    .method("#key-1")
    .purpose(ProofPurpose::Authentication)
    .sign(&mut data)
    .unwrap();
  assert_eq!(
    data.proof.clone().unwrap().purpose.unwrap(),
    ProofPurpose::Authentication
  );

  // VALID: verifying without checking the purpose succeeds.
  document.verifier().verify(&data).unwrap();
  // VALID: verifying with the correct purpose succeeds.
  document
    .verifier()
    .purpose(ProofPurpose::Authentication)
    .verify(&data)
    .unwrap();

  // INVALID: verifying with the wrong purpose fails.
  assert!(document
    .verifier()
    .purpose(ProofPurpose::AssertionMethod)
    .verify(&data)
    .is_err());

  // VALID: purpose overrides the method scope.
  document
    .verifier()
    .method_scope(MethodScope::capability_delegation())
    .purpose(ProofPurpose::Authentication)
    .verify(&data)
    .unwrap();
  // INVALID: purpose overrides the otherwise correct method scope.
  assert!(document
    .verifier()
    .method_scope(MethodScope::authentication())
    .purpose(ProofPurpose::AssertionMethod)
    .verify(&data)
    .is_err());
}

#[test]
fn test_sign_verify_expires() {
  let (key, document) = setup();
  let mut data: MockObject = MockObject::new(123);
  assert!(document.verifier().verify(&data).is_err());

  // Sign with an expiration in the FUTURE.
  let expires_future: Timestamp = Timestamp::from_unix(Timestamp::now_utc().to_unix() + 60).unwrap();
  document
    .signer(key.private())
    .method("#key-1")
    .expires(expires_future)
    .sign(&mut data)
    .unwrap();
  assert_eq!(data.proof.clone().unwrap().expires.unwrap(), expires_future);

  // VALID: verifying before expiration succeeds.
  document.verifier().verify(&data).unwrap();
  document.verifier().allow_expired(false).verify(&data).unwrap();
  document.verifier().allow_expired(true).verify(&data).unwrap();

  // Sign with an expiration in the PAST.
  let expires_past: Timestamp = Timestamp::from_unix(Timestamp::now_utc().to_unix() - 60).unwrap();
  document
    .signer(key.private())
    .method("#key-1")
    .expires(expires_past)
    .sign(&mut data)
    .unwrap();
  assert_eq!(data.proof.clone().unwrap().expires.unwrap(), expires_past);

  // VALID: verifying without checking expiration succeeds.
  document.verifier().allow_expired(true).verify(&data).unwrap();
  // INVALID: verifying after expiration fails.
  assert!(document.verifier().verify(&data).is_err());
  assert!(document.verifier().allow_expired(false).verify(&data).is_err());
}
