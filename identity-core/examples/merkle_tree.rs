// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example merkle_tree

use digest::Digest;
use identity_core::crypto::merkle_tree::DigestExt;
use identity_core::crypto::merkle_tree::Hash;
use identity_core::crypto::merkle_tree::MTree;
use identity_core::crypto::merkle_tree::Proof;
use identity_core::crypto::KeyPair;
use identity_core::error::Result;
use rand::rngs::OsRng;
use rand::Rng;
use sha2::Sha256;

const LEAVES: usize = 1 << 8;

fn generate_leaves(count: usize) -> Result<Vec<KeyPair>> {
  (0..count).map(|_| KeyPair::new_ed25519()).collect()
}

fn generate_hashes<'a, D, T, I>(digest: &mut D, leaves: I) -> Vec<Hash<D>>
where
  D: Digest,
  T: AsRef<[u8]> + 'a,
  I: IntoIterator<Item = &'a T>,
{
  leaves
    .into_iter()
    .map(AsRef::as_ref)
    .map(|leaf| digest.hash_leaf(leaf))
    .collect()
}

fn main() -> Result<()> {
  let mut digest: Sha256 = Sha256::new();

  // Choose a random index from 0..LEAVES.
  //
  // We will generate a proof-of-inclusion for this public key.
  let index: usize = OsRng.gen_range(0, LEAVES);

  println!("Target Leaves: {}", LEAVES);
  println!("Target Index:  {}", index);

  // Generate a list of keypairs to use for the Merkle tree.
  let kpairs: Vec<KeyPair> = generate_leaves(LEAVES).unwrap();

  // Hash all keypairs with SHA-256.
  let leaves: _ = kpairs.iter().map(KeyPair::public);
  let hashes: Vec<Hash<Sha256>> = generate_hashes(&mut digest, leaves);

  // Construct the Merkle tree from the list of hashes.
  let tree: MTree<Sha256> = MTree::from_leaves(&hashes).unwrap();
  println!("Merkle Tree: {:#?}", tree);

  // Generate a proof-of-inclusion for the leaf node at the specified index.
  let proof: Proof<Sha256> = tree.proof(index).unwrap();
  println!("Inclusion Proof: {:#?}", proof);

  // Hash the target public key with SHA-256.
  let target: Hash<Sha256> = digest.hash_leaf(kpairs[index].public().as_ref());
  println!("Target Hash: {:?}", target);

  // Use the generated proof to verify inclusion of the target hash in the
  // Merkle tree.
  let verified: bool = tree.verify(&proof, target);
  println!("Proof Verified: {:#?}", verified);

  Ok(())
}
