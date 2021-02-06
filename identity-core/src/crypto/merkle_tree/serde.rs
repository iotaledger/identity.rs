// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryInto;

use crate::crypto::merkle_tree::DigestExt;
use crate::crypto::merkle_tree::Hash;
use crate::crypto::merkle_tree::Node;
use crate::crypto::merkle_tree::Proof;

const TAG_L: u8 = 0b11110000;
const TAG_R: u8 = 0b00001111;

impl<D> Node<D>
where
  D: DigestExt,
{
  /// Encodes `self` as a vector of bytes.
  ///
  /// See [`Node::encode_into`] for more details.
  pub fn encode(&self) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::with_capacity(1 + D::OUTPUT_SIZE);

    self.encode_into(&mut output);

    output
  }

  /// Encodes `self` as a vector of bytes, appended to `output`.
  ///
  /// The vector of bytes will have the following layout:
  ///
  ///   ```text
  ///   [ U8(TAG) | HASH ]
  ///   ```
  pub fn encode_into(&self, output: &mut Vec<u8>) {
    match self {
      Node::L(hash) => {
        output.push(self::TAG_L);
        output.extend_from_slice(hash.as_slice());
      }
      Node::R(hash) => {
        output.push(self::TAG_R);
        output.extend_from_slice(hash.as_slice());
      }
    }
  }

  /// Decodes a [`Node`] from a slice of bytes.
  ///
  /// See [`Node::encode_into`] for more details on the expected input format.
  pub fn decode(slice: &[u8]) -> Option<Self> {
    match slice.get(0).copied()? {
      self::TAG_L => slice.get(1..).and_then(__decode_hash).map(Self::L),
      self::TAG_R => slice.get(1..).and_then(__decode_hash).map(Self::R),
      _ => None,
    }
  }
}

impl<D> Proof<D>
where
  D: DigestExt,
{
  /// Encodes `self` as a vector of bytes.
  ///
  /// See [`Proof::encode_into`] for more details.
  pub fn encode(&self) -> Vec<u8> {
    let capacity: usize = __proof_len::<D>(self.nodes().len());
    let mut output: Vec<u8> = Vec::with_capacity(capacity);

    self.encode_into(&mut output);

    output
  }

  /// Encodes `self` as a vector of bytes, appended to `output`.
  ///
  /// The vector of bytes will have the following layout:
  ///
  ///   ```text
  ///   Node = [ U8(NODE-TAG) | NODE-HASH ]
  ///   List = [ Node0, Node1, ..., NodeN-1 ]
  ///
  ///   [ U32(LEN(List)) | List ]
  ///   ```
  pub fn encode_into(&self, output: &mut Vec<u8>) {
    let size: usize = self.nodes().len();
    let size: [u8; 4] = (size as u32).to_be_bytes();

    output.extend_from_slice(&size);

    for node in self.nodes() {
      node.encode_into(output);
    }
  }

  /// Decodes a [`Proof`] from a slice of bytes.
  ///
  /// See [`Proof::encode_into`] for more details on the expected input format.
  pub fn decode(slice: &[u8]) -> Option<Self> {
    let size: [u8; 4] = slice.get(0..4)?.try_into().ok()?;
    let size: usize = u32::from_be_bytes(size).try_into().ok()?;

    let mut nodes: Vec<Node<D>> = Vec::with_capacity(size);
    let mut slice: &[u8] = slice.get(4..)?;

    for _ in 0..size {
      nodes.push(Node::decode(slice)?);
      slice = slice.get(1 + D::OUTPUT_SIZE..)?;
    }

    Some(Self::new(nodes.into_boxed_slice()))
  }
}

// LEN + (NODES * (TAG + HASH))
#[inline]
fn __proof_len<D>(nodes: usize) -> usize
where
  D: DigestExt,
{
  4 + (nodes * (1 + D::OUTPUT_SIZE))
}

#[inline]
fn __decode_hash<D>(slice: &[u8]) -> Option<Hash<D>>
where
  D: DigestExt,
{
  slice.get(..D::OUTPUT_SIZE).and_then(Hash::from_slice)
}
