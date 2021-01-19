//! cargo run --example merkle_tree

use digest::Digest;
use identity_core::crypto::merkle_tree::DigestExt;
use identity_core::crypto::merkle_tree::Hash;
use identity_core::crypto::merkle_tree::MTree;
use identity_core::crypto::merkle_tree::Proof;
use identity_core::crypto::KeyPair;
use identity_core::error::Result;
use identity_core::proof::JcsEd25519Signature2020;
use rand::rngs::OsRng;
use rand::Rng;
use sha2::Sha256;
use std::time::Instant;

const LEAVES: usize = 1 << 8;

struct Timer(Instant);

impl Timer {
    fn new() -> Self {
        Self(Instant::now())
    }

    fn step(&mut self, label: &str) {
        println!("{}: {:?}", label, self.0.elapsed());
        self.0 = Instant::now();
    }
}

fn generate_leaves(count: usize) -> Vec<KeyPair> {
    (0..count).map(|_| JcsEd25519Signature2020::new_keypair()).collect()
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
        .map(|leaf| digest.hash_data(leaf))
        .collect()
}

fn main() -> Result<()> {
    let mut digest: Sha256 = Sha256::new();
    let mut timer: Timer = Timer::new();

    let index: usize = OsRng.gen_range(0, LEAVES);

    println!("Target Leaves: {}", LEAVES);
    println!("Target Index:  {}", index);

    let kpairs: Vec<KeyPair> = generate_leaves(LEAVES);
    timer.step("Keys");

    let leaves: _ = kpairs.iter().map(KeyPair::public);
    let hashes: Vec<Hash<Sha256>> = generate_hashes(&mut digest, leaves);
    timer.step("Hash");

    let tree: MTree<Sha256> = MTree::from_leaves(&hashes).unwrap();
    timer.step("Tree");

    let proof: Proof<Sha256> = tree.proof(index).unwrap();
    timer.step("Proof");

    let target: Hash<Sha256> = digest.hash_data(kpairs[index].public().as_ref());
    let verified: bool = tree.verify(&proof, target);
    timer.step("Verify");

    println!("Merkle Tree: {:#?}", tree);
    println!("Inclusion Proof: {:#?}", proof);
    println!("Proof Verified: {:#?}", verified);

    Ok(())
}
