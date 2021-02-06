// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use rand::rngs::OsRng;
use rand::seq::IteratorRandom;
use rand::Rng;
use sha2::Sha256;

use crate::common::BitSet;
use crate::crypto::merkle_key::DynSigner;
use crate::crypto::merkle_key::DynVerifier;
use crate::crypto::KeyCollection;
use crate::crypto::PublicKey;
use crate::crypto::SecretKey;
use crate::crypto::SignatureSign;
use crate::crypto::SignatureValue;
use crate::crypto::SignatureVerify;

#[test]
fn test_sign_verify() {
  let input: &[u8] = b"IOTA Identity";
  let total: usize = 1 << OsRng.gen_range(6, 10);
  let index: usize = OsRng.gen_range(0, total);

  let keys: KeyCollection = KeyCollection::new_ed25519(total).unwrap();
  let signer: DynSigner<'_, '_, Sha256> = keys.merkle_key_signer(index).unwrap();
  let mut verifier: DynVerifier<'_, '_, Sha256> = keys.merkle_key_verifier();

  let public: &PublicKey = keys.public(index).unwrap();
  let secret: &SecretKey = keys.secret(index).unwrap();

  // Test a few semi-valid keys - included in the Merkle root but not the signing key.
  let samples: Vec<&PublicKey> = keys
    .iter_public()
    .filter(|key| key.as_ref() != public.as_ref())
    .choose_multiple(&mut OsRng, 10);

  let signature: SignatureValue = signer.sign(&input, secret.as_ref()).unwrap();

  // The signature should be valid
  assert!(verifier.verify(&input, &signature, public.as_ref()).is_ok());

  // Ensure all other keys are NOT valid
  for key in samples.iter() {
    assert!(verifier.verify(&input, &signature, key.as_ref()).is_err());
  }

  // Revoke the target key and ensure the signature is not considered valid.
  let mut revocation: BitSet = BitSet::new();
  revocation.insert(index as u32);
  verifier.set_revocation(revocation);

  assert!(verifier.verify(&input, &signature, public.as_ref()).is_err());

  // Ensure all other keys are NOT valid
  for key in samples.iter() {
    assert!(verifier.verify(&input, &signature, key.as_ref()).is_err());
  }

  // Reinstate the key and ensure the signature is now valid.
  let mut revocation: BitSet = BitSet::new();
  revocation.insert_all((0u32..total as u32).into_iter());
  revocation.remove(index as u32);
  verifier.set_revocation(revocation);

  assert!(verifier.verify(&input, &signature, public.as_ref()).is_ok());

  // Ensure all other keys are NOT valid
  for key in samples.iter() {
    assert!(verifier.verify(&input, &signature, key.as_ref()).is_err());
  }
}
