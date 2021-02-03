// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example merkle_tree

use identity::core::Result;
use identity::crypto::merkle_tree::MTree;
use identity::crypto::merkle_tree::Proof;
use identity::crypto::KeyCollection;
use sha2::Sha256;

// Generate a key collection of this size.
const LEAVES: usize = 1 << 8;

// Generate a proof-of-inclusion for this public key.
const INDEX: usize = LEAVES >> 1;

fn main() -> Result<()> {
  println!("Target Leaves: {}", LEAVES);
  println!("Target Index:  {}", INDEX);

  // Generate a collection of keypairs to use for the Merkle tree.
  let keys: KeyCollection = KeyCollection::new_ed25519(LEAVES).unwrap();

  // Construct the Merkle tree from the public keys in the collection.
  let tree: MTree<Sha256> = keys.to_merkle_tree().unwrap();

  println!("Merkle Tree: {:#?}", tree);

  // Generate a proof-of-inclusion for the leaf node at the specified index.
  let proof: Proof<Sha256> = tree.proof(INDEX).unwrap();

  println!("Merkle Proof: {:#?}", proof);

  // Use the generated proof to verify inclusion of the target hash in the tree.
  let verified: bool = tree.verify(&proof, keys[INDEX].public());

  println!("Verified: {:#?}", verified);

  Ok(())
}
