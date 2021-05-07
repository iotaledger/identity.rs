// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::merkle_key::MerkleKey;
use identity_core::crypto::merkle_key::Sha256;
use identity_core::crypto::merkle_tree::Hash;
use identity_core::crypto::merkle_tree::Proof;
use identity_core::crypto::Ed25519;
use identity_core::crypto::KeyCollection;
use identity_core::crypto::KeyPair;
use identity_core::crypto::PublicKey;
use identity_core::crypto::SecretKey;
use identity_core::crypto::SetSignature;
use identity_core::crypto::Signature;
use identity_core::crypto::TrySignature;
use identity_core::crypto::TrySignatureMut;

use crate::did::DID;
use crate::document::CoreDocument;
use crate::verifiable::Properties;
use crate::verification::MethodData;
use crate::verification::MethodType;
use crate::verification::MethodUriType;
use crate::verification::TryMethod;
use crate::verification::VerificationMethod;

#[derive(Debug, Serialize)]
struct That {
  data: u32,
  #[serde(skip_serializing_if = "Option::is_none")]
  proof: Option<Signature>,
}

impl That {
  fn new(data: u32) -> Self {
    Self { data, proof: None }
  }
}

impl TrySignature for That {
  fn signature(&self) -> Option<&Signature> {
    self.proof.as_ref()
  }
}

impl TrySignatureMut for That {
  fn signature_mut(&mut self) -> Option<&mut Signature> {
    self.proof.as_mut()
  }
}

impl SetSignature for That {
  fn set_signature(&mut self, signature: Signature) {
    self.proof = Some(signature);
  }
}

impl TryMethod for That {
  const TYPE: MethodUriType = MethodUriType::Relative;
}

// ===========================================================================
// ===========================================================================

#[test]
fn test_sign_verify_this_ed25519() {
  let key: KeyPair = KeyPair::new_ed25519().unwrap();
  let controller: DID = "did:example:1234".parse().unwrap();

  let method: VerificationMethod = VerificationMethod::builder(Default::default())
    .id(controller.join("#key-1").unwrap())
    .controller(controller.clone())
    .key_type(MethodType::Ed25519VerificationKey2018)
    .key_data(MethodData::new_b58(key.public()))
    .build()
    .unwrap();

  let mut document: CoreDocument<Properties> = CoreDocument::builder(Default::default())
    .id(controller)
    .verification_method(method)
    .build()
    .unwrap();

  assert!(document.verify_this().is_err());

  document.sign_this("#key-1", key.secret()).unwrap();

  assert!(document.verify_this().is_ok());
}

#[test]
fn test_sign_verify_that_merkle_key_ed25519_sha256() {
  let total: usize = 1 << 11;
  let index: usize = 1 << 9;

  let keys: KeyCollection = KeyCollection::new_ed25519(total).unwrap();
  let controller: DID = "did:example:1234".parse().unwrap();

  let root: Hash<Sha256> = keys.merkle_root();
  let proof: Proof<Sha256> = keys.merkle_proof(index).unwrap();
  let mkey: Vec<u8> = MerkleKey::encode_key::<Sha256, Ed25519>(&root);

  let method: VerificationMethod = VerificationMethod::builder(Default::default())
    .id(controller.join("#key-collection").unwrap())
    .controller(controller.clone())
    .key_type(MethodType::MerkleKeyCollection2021)
    .key_data(MethodData::new_b58(mkey))
    .build()
    .unwrap();

  let document: CoreDocument<Properties> = CoreDocument::builder(Default::default())
    .id(controller)
    .verification_method(method)
    .build()
    .unwrap();

  let public: &PublicKey = keys.public(index).unwrap();
  let secret: &SecretKey = keys.secret(index).unwrap();

  let mut that: That = That::new(123);

  assert!(document.verifier().verify(&that).is_err());

  document
    .signer(secret)
    .method("#key-collection")
    .merkle_key((public, &proof))
    .sign(&mut that)
    .unwrap();

  assert!(document.verifier().verify(&that).is_ok());
}
