// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryInto;
use core::iter;

use crate::crypto::merkle_key::Digest;
use crate::crypto::merkle_key::Ed25519;
use crate::crypto::merkle_key::Signature;
use crate::crypto::merkle_tree::Hash;
use crate::crypto::merkle_tree::MTree;
use crate::crypto::merkle_tree::Node;
use crate::crypto::merkle_tree::Proof;

/// Common utilities for working with Merkle Key Collection Signatures.
#[derive(Clone, Copy, Debug)]
pub struct MerkleKey;

impl MerkleKey {
  /// The `type` value of a Merkle Key Collection Signature.
  pub const SIGNATURE_NAME: &'static str = "MerkleKeySignature2021";

  /// A tag byte identifying a left tree node.
  pub const TAG_L: u8 = 0b11110000;

  /// A tag byte identifying a right tree node.
  pub const TAG_R: u8 = 0b00001111;

  /// Encodes the given [`MTree`] as a DID Document public key.
  pub fn encode_key<D, S>(tree: &MTree<D>) -> Vec<u8>
  where
    D: Digest,
    S: Signature,
  {
    let mut output: Vec<u8> = Vec::with_capacity(2 + D::OUTPUT_SIZE);
    output.push(S::TAG);
    output.push(D::TAG);
    output.extend_from_slice(tree.root().as_ref());
    output
  }

  /// Encodes the given [`MTree`] as a DID Document public key using `ed25519`
  /// as the signature algorithm.
  pub fn encode_ed25519_key<D>(tree: &MTree<D>) -> Vec<u8>
  where
    D: Digest,
  {
    Self::encode_key::<D, Ed25519>(tree)
  }

  // Encodes a proof in the following form:
  //
  //   [ U32(PATH-LEN) [ [ U8(NODE-TAG) | HASH(NODE-PATH) ] ... ] ]
  pub(crate) fn encode_proof<D>(proof: &Proof<D>) -> Vec<u8>
  where
    D: Digest,
  {
    let size: usize = proof.nodes().len();
    let size: [u8; 4] = (size as u32).to_be_bytes();

    let data: _ = proof.nodes().iter().flat_map(|node| match node {
      Node::L(hash) => iter::once(Self::TAG_L).chain(hash.as_ref().iter().copied()),
      Node::R(hash) => iter::once(Self::TAG_R).chain(hash.as_ref().iter().copied()),
    });

    size.iter().copied().chain(data).collect()
  }

  // Decodes a proof in the following form:
  //
  //   [ U32(PATH-LEN) [ [ U8(NODE-TAG) | HASH(NODE-PATH) ] ... ] ]
  pub(crate) fn decode_proof<D>(data: &[u8]) -> Option<Proof<D>>
  where
    D: Digest,
  {
    let size: [u8; 4] = data.get(0..4)?.try_into().ok()?;
    let size: usize = u32::from_be_bytes(size).try_into().ok()?;

    let mut nodes: Vec<Node<D>> = Vec::with_capacity(size);
    let mut slice: &[u8] = data.get(4..)?;

    for _ in 0..size {
      let ntag: u8 = slice.get(0).copied()?;
      let data: &[u8] = slice.get(1..1 + D::OUTPUT_SIZE)?;
      let hash: Hash<D> = Hash::from_slice(data)?;

      match ntag {
        Self::TAG_L => nodes.push(Node::L(hash)),
        Self::TAG_R => nodes.push(Node::R(hash)),
        _ => return None,
      }

      slice = slice.get(1 + D::OUTPUT_SIZE..)?;
    }

    Some(Proof::new(nodes.into_boxed_slice()))
  }
}
