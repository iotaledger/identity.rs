// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::borrow::Borrow;
use core::cmp::Ordering;
use core::convert::AsMut;
use core::convert::AsRef;
use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;
use core::hash::Hash;
use core::hash::Hasher;
use core::ops::Deref;
use core::ops::DerefMut;

use crate::did::CoreDIDUrl;

/// A helper struct for comparing types only by `DID`.
///
/// Types are expected to implement `AsRef<CoreDIDUrl>` which allows access to traits
/// for ordering and comparison.
#[derive(Clone, Copy, Deserialize, Serialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct DIDKey<T>(T);

impl<T> DIDKey<T> {
  /// Create a new `DIDKey`.
  #[inline]
  pub const fn new(inner: T) -> Self {
    Self(inner)
  }

  /// Consumes the `DIDKey` and returns the inner `T`.
  #[inline]
  pub fn into_inner(self) -> T {
    self.0
  }

  /// Returns a reference to the `DID`.
  #[inline]
  pub fn as_did_url(&self) -> &CoreDIDUrl
  where
    T: AsRef<CoreDIDUrl>,
  {
    self.0.as_ref()
  }
}

impl<T> PartialEq for DIDKey<T>
where
  T: AsRef<CoreDIDUrl>,
{
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.as_did_url().eq(other.as_did_url())
  }
}

impl<T> Eq for DIDKey<T> where T: AsRef<CoreDIDUrl> {}

impl<T> PartialOrd for DIDKey<T>
where
  T: AsRef<CoreDIDUrl>,
{
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.as_did_url().partial_cmp(other.as_did_url())
  }
}

impl<T> Ord for DIDKey<T>
where
  T: AsRef<CoreDIDUrl>,
{
  #[inline]
  fn cmp(&self, other: &Self) -> Ordering {
    self.as_did_url().cmp(other.as_did_url())
  }
}

impl<T> Hash for DIDKey<T>
where
  T: AsRef<CoreDIDUrl>,
{
  fn hash<H>(&self, hasher: &mut H)
  where
    H: Hasher,
  {
    self.as_did_url().hash(hasher)
  }
}

impl<T> Deref for DIDKey<T> {
  type Target = T;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<T> DerefMut for DIDKey<T> {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl<T> AsRef<T> for DIDKey<T> {
  #[inline]
  fn as_ref(&self) -> &T {
    &self.0
  }
}

impl<T> AsMut<T> for DIDKey<T> {
  #[inline]
  fn as_mut(&mut self) -> &mut T {
    &mut self.0
  }
}

impl<T> Borrow<CoreDIDUrl> for DIDKey<T>
where
  T: AsRef<CoreDIDUrl>,
{
  #[inline]
  fn borrow(&self) -> &CoreDIDUrl {
    self.as_did_url()
  }
}

impl<T> From<T> for DIDKey<T> {
  #[inline]
  fn from(other: T) -> Self {
    Self(other)
  }
}

impl<T> Debug for DIDKey<T>
where
  T: Debug,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    Debug::fmt(&self.0, f)
  }
}

impl<T> Display for DIDKey<T>
where
  T: Display,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    Display::fmt(&self.0, f)
  }
}
