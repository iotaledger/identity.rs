use core::cmp::Ordering;
use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result;
use digest::Output;
use digest::Digest;
use digest::generic_array::typenum::Unsigned;
use subtle::ConstantTimeEq;
use subtle::Choice;

use crate::utils::encode_hex;

pub struct Hash<D>(Output<D>)
where
  D: Digest;

impl<D: Digest> Hash<D> {
  pub fn from_slice(slice: &[u8]) -> Option<Self> {
    if slice.len() != D::OutputSize::USIZE {
      return None;
    }

    let mut this: Self = Self::default();

    this.0.copy_from_slice(slice);

    Some(this)
  }
}

impl<D: Digest> Clone for Hash<D>
where
  Output<D>: Clone,
{
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

impl<D: Digest> Copy for Hash<D> where Output<D>: Copy {}

impl<D: Digest> PartialEq for Hash<D>
where
  Output<D>: PartialEq,
{
  fn eq(&self, other: &Self) -> bool {
    self.0.eq(&other.0)
  }
}

impl<D: Digest> Eq for Hash<D> where Output<D>: Eq {}

impl<D: Digest> PartialOrd for Hash<D>
where
  Output<D>: PartialOrd,
{
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.0.partial_cmp(&other.0)
  }
}

impl<D: Digest> Ord for Hash<D>
where
  Output<D>: Ord,
{
  fn cmp(&self, other: &Self) -> Ordering {
    self.0.cmp(&other.0)
  }
}

impl<D: Digest> Debug for Hash<D> {
  fn fmt(&self, f: &mut Formatter) -> Result {
    f.write_str(&encode_hex(self))
  }
}

impl<D: Digest> Default for Hash<D> {
  fn default() -> Self {
    Self(Output::<D>::default())
  }
}

impl<D: Digest> AsRef<[u8]> for Hash<D> {
  fn as_ref(&self) -> &[u8] {
    self.0.as_ref()
  }
}

impl<D: Digest> From<Output<D>> for Hash<D> {
  fn from(other: Output<D>) -> Self {
    Self(other)
  }
}

impl<D: Digest> ConstantTimeEq for Hash<D> {
  fn ct_eq(&self, other: &Self) -> Choice {
    self.as_ref().ct_eq(other.as_ref())
  }
}
