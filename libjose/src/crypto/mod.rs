use alloc::boxed::Box;
use alloc::vec::Vec;
use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::marker::PhantomData;
use zeroize::Zeroize;

cfg_if::cfg_if! {
  if #[cfg(feature = "ring")] {
    mod ring;
    pub(crate) use self::ring::*;
  } else {
    mod noop;
    pub(crate) use self::noop::*;
  }
}

pub type KeyPair = (PKey<Public>, PKey<Secret>);

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Public {}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Secret {}

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PKey<T>(Box<[u8]>, PhantomData<T>);

impl<T> Drop for PKey<T> {
  fn drop(&mut self) {
    self.0.zeroize();
  }
}

impl Debug for PKey<Public> {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    f.write_str("Public")
  }
}

impl Debug for PKey<Secret> {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    f.write_str("Secret")
  }
}

impl<T> Zeroize for PKey<T> {
  fn zeroize(&mut self) {
    self.0.zeroize();
  }
}

impl<T> AsRef<[u8]> for PKey<T> {
  fn as_ref(&self) -> &[u8] {
    self.0.as_ref()
  }
}

impl<T> From<Box<[u8]>> for PKey<T> {
  fn from(other: Box<[u8]>) -> Self {
    Self(other, PhantomData)
  }
}

impl<T> From<Vec<u8>> for PKey<T> {
  fn from(other: Vec<u8>) -> Self {
    Self(other.into(), PhantomData)
  }
}

impl<'a, T> From<&'a [u8]> for PKey<T> {
  fn from(other: &'a [u8]) -> Self {
    Self(other.into(), PhantomData)
  }
}
