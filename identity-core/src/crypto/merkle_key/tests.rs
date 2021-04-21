// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use rand::rngs::OsRng;
use rand::seq::IteratorRandom;
use rand::Rng;

use crate::common::BitSet;
use crate::crypto::merkle_key::Blake2b256;
use crate::crypto::merkle_key::MerkleDigest;
use crate::crypto::merkle_key::MerkleSignature;
use crate::crypto::merkle_key::MerkleSignatureTag;
use crate::crypto::merkle_key::MerkleSigner;
use crate::crypto::merkle_key::MerkleVerifier;
use crate::crypto::merkle_key::Sha256;
use crate::crypto::merkle_key::SigningKey;
use crate::crypto::merkle_key::VerificationKey;
use crate::crypto::Ed25519;
use crate::crypto::KeyCollection;
use crate::crypto::PublicKey;
use crate::crypto::Sign;
use crate::crypto::SignatureValue;
use crate::crypto::Signer as _;
use crate::crypto::Verifier as _;
use crate::crypto::Verify;
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
  S: MerkleSignature + Sign<Secret = [u8]> + Verify<Public = [u8]>,
  S::Output: AsRef<[u8]>,
{
  assert_eq!(S::TAG, MerkleSignatureTag::ED25519);

  let input: &[u8] = b"IOTA Identity";
  let total: usize = 1 << OsRng.gen_range(6..10);
  let index: usize = OsRng.gen_range(0..total);

  let keys: KeyCollection = KeyCollection::new_ed25519(total).unwrap();
  let mkey: Vec<u8> = keys.encode_merkle_key::<D>();

  let skey: SigningKey<'_, D> = keys.merkle_key(index).unwrap();
  let vkey: VerificationKey<'_> = VerificationKey::from_borrowed(&mkey);

  let public: &PublicKey = keys.public(index).unwrap();

  // Test a few semi-valid keys - included in the Merkle root but not the signing key.
  let samples: Vec<&PublicKey> = keys
    .iter_public()
    .filter(|key| key.as_ref() != public.as_ref())
    .choose_multiple(&mut OsRng, 10);

  let signature: SignatureValue = MerkleSigner::<D, S>::sign(&input, &skey).unwrap();

  // The signature should be valid
  assert!(MerkleVerifier::<D, S>::verify(&input, &signature, &vkey).is_ok());

  // Ensure all other keys are NOT valid
  for key in samples.iter() {
    assert!(MerkleVerifier::<D, S>::verify(&input, &inject_key(&signature, key), &vkey).is_err());
  }

  // Revoke the target key and ensure the signature is not considered valid.
  let mut revocation: BitSet = BitSet::new();
  revocation.insert(index as u32);
  let mut vkey: VerificationKey<'_> = VerificationKey::from_borrowed(&mkey);
  vkey.set_revocation(&revocation);

  assert!(MerkleVerifier::<D, S>::verify(&input, &signature, &vkey).is_err());

  // Ensure all other keys are NOT valid
  for key in samples.iter() {
    assert!(MerkleVerifier::<D, S>::verify(&input, &inject_key(&signature, key), &vkey).is_err());
  }

  // Reinstate the key and ensure the signature is now valid.
  revocation.remove(index as u32);
  let mut vkey: VerificationKey<'_> = VerificationKey::from_borrowed(&mkey);
  vkey.set_revocation(&revocation);

  assert!(MerkleVerifier::<D, S>::verify(&input, &signature, &vkey).is_ok());

  // Ensure all other keys are NOT valid
  for key in samples.iter() {
    assert!(MerkleVerifier::<D, S>::verify(&input, &inject_key(&signature, key), &vkey).is_err());
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
