// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::cmp::Ordering;
use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result;
use subtle::Choice;
use subtle::ConstantTimeEq;

use crate::crypto::merkle_tree::DigestExt;
use crate::crypto::merkle_tree::Output;
use crate::utils::encode_b58;

/// The output of a hash function.
pub struct Hash<D>(Output<D>)
where
  D: DigestExt;

impl<D: DigestExt> Hash<D> {
  /// Creates a new [`struct@Hash`] from a slice of bytes.
  pub fn from_slice(slice: &[u8]) -> Option<Self> {
    if slice.len() != D::OUTPUT_SIZE {
      return None;
    }

    // SAFETY: We just asserted the length of `slice`
    Some(unsafe { Self::from_slice_unchecked(slice) })
  }

  /// Creates a new [`struct@Hash`] from a slice of bytes.
  ///
  /// # Safety
  ///
  /// This function is unsafe because it does not ensure the input slice
  /// has the correct length.
  pub unsafe fn from_slice_unchecked(slice: &[u8]) -> Self {
    let mut this: Self = Self::default();
    this.0.copy_from_slice(slice);
    this
  }

  /// Returns the [`struct@Hash`] as a slice of bytes.
  pub fn as_slice(&self) -> &[u8] {
    self.0.as_ref()
  }
}

impl<D: DigestExt> Clone for Hash<D>
where
  Output<D>: Clone,
{
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

impl<D: DigestExt> Copy for Hash<D> where Output<D>: Copy {}

impl<D: DigestExt> PartialEq for Hash<D>
where
  Output<D>: PartialEq,
{
  fn eq(&self, other: &Self) -> bool {
    self.ct_eq(other).into()
  }
}

impl<D: DigestExt> Eq for Hash<D> where Output<D>: Eq {}

impl<D: DigestExt> PartialOrd for Hash<D>
where
  Output<D>: PartialOrd,
{
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.0.partial_cmp(&other.0)
  }
}

impl<D: DigestExt> Ord for Hash<D>
where
  Output<D>: Ord,
{
  fn cmp(&self, other: &Self) -> Ordering {
    self.0.cmp(&other.0)
  }
}

impl<D: DigestExt> Debug for Hash<D> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.write_str(&encode_b58(self.as_slice()))
  }
}

impl<D: DigestExt> Default for Hash<D> {
  fn default() -> Self {
    Self(Output::<D>::default())
  }
}

impl<D: DigestExt> From<Output<D>> for Hash<D> {
  fn from(other: Output<D>) -> Self {
    Self(other)
  }
}

impl<D: DigestExt> ConstantTimeEq for Hash<D> {
  fn ct_eq(&self, other: &Self) -> Choice {
    self.as_slice().ct_eq(other.as_slice())
  }
}
