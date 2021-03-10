// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use rand::rngs::OsRng;
use rand::seq::IteratorRandom;
use rand::Rng;

use crate::common::BitSet;
use crate::crypto::merkle_key::Blake2b256;
use crate::crypto::merkle_key::Ed25519;
use crate::crypto::merkle_key::MerkleDigest;
use crate::crypto::merkle_key::MerkleSignature;
use crate::crypto::merkle_key::Sha256;
use crate::crypto::merkle_key::Signer;
use crate::crypto::merkle_key::SigningKey;
use crate::crypto::merkle_key::VerificationKey;
use crate::crypto::merkle_key::Verifier;
use crate::crypto::KeyCollection;
use crate::crypto::PublicKey;
use crate::crypto::SecretKey;
use crate::crypto::SignatureSign;
use crate::crypto::SignatureValue;
use crate::crypto::SignatureVerify;
use crate::utils::encode_b58;

fn inject_key(signature: &SignatureValue, key: &PublicKey) -> SignatureValue {
  let value: &str = signature.as_str();
  let parts: Vec<&str> = value.split('.').collect();

  assert_eq!(parts.len(), 3);

  let encoded: String = encode_b58(key.as_ref());
  let value: String = format!("{}.{}.{}", encoded, parts[1], parts[2]);

  SignatureValue::Signature(value)
}

fn __test_sign_verify<D, S>()
where
  D: MerkleDigest,
  S: MerkleSignature,
  S: for<'a> SignatureSign<'a, Secret = SecretKey>,
  S: for<'a> SignatureVerify<'a, Public = PublicKey>,
{
  assert_eq!(S::TAG, Ed25519::TAG);

  let input: &[u8] = b"IOTA Identity";
  let total: usize = 1 << OsRng.gen_range(6..10);
  let index: usize = OsRng.gen_range(0..total);

  let keys: KeyCollection = KeyCollection::new_ed25519(total).unwrap();
  let mkey: Vec<u8> = keys.encode_merkle_key::<D>();

  let skey: SigningKey<'_, D> = keys.merkle_key(index).unwrap();
  let vkey: VerificationKey<'_> = VerificationKey::from_borrowed(&mkey);

  let signer: Signer<'_, D, S> = Signer::create(&skey);
  let verifier: Verifier<'_, D, S> = Verifier::create(&vkey);

  let public: &PublicKey = keys.public(index).unwrap();

  // Test a few semi-valid keys - included in the Merkle root but not the signing key.
  let samples: Vec<&PublicKey> = keys
    .iter_public()
    .filter(|key| key.as_ref() != public.as_ref())
    .choose_multiple(&mut OsRng, 10);

  let signature: SignatureValue = signer.sign(&input).unwrap();

  // The signature should be valid
  assert!(verifier.verify(&input, &signature).is_ok());

  // Ensure all other keys are NOT valid
  for key in samples.iter() {
    assert!(verifier.verify(&input, &inject_key(&signature, key)).is_err());
  }

  // Revoke the target key and ensure the signature is not considered valid.
  let mut revocation: BitSet = BitSet::new();
  revocation.insert(index as u32);
  let mut vkey: VerificationKey<'_> = VerificationKey::from_borrowed(&mkey);
  vkey.set_revocation(&revocation);
  let verifier: Verifier<'_, D, S> = Verifier::create(&vkey);

  assert!(verifier.verify(&input, &signature).is_err());

  // Ensure all other keys are NOT valid
  for key in samples.iter() {
    assert!(verifier.verify(&input, &inject_key(&signature, key)).is_err());
  }

  // Reinstate the key and ensure the signature is now valid.
  revocation.remove(index as u32);
  let mut vkey: VerificationKey<'_> = VerificationKey::from_borrowed(&mkey);
  vkey.set_revocation(&revocation);
  let verifier: Verifier<'_, D, S> = Verifier::create(&vkey);

  assert!(verifier.verify(&input, &signature).is_ok());

  // Ensure all other keys are NOT valid
  for key in samples.iter() {
    assert!(verifier.verify(&input, &inject_key(&signature, key)).is_err());
  }
}

#[test]
fn test_sign_verify_sha256_ed25519() {
  __test_sign_verify::<Sha256, Ed25519>();
}

#[test]
fn test_sign_verify_blake2b_ed25519() {
  __test_sign_verify::<Blake2b256, Ed25519>();
}
