// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;

use identity_core::common::KeyComparable;

use crate::verification_method::VerificationMethod;
use identity_did::CoreDID;
use identity_did::DIDUrl;
use identity_did::DID;

/// A reference to a verification method, either a `DID` or embedded `Method`.
#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum MethodRef<D = CoreDID>
where
  D: DID,
{
  Embed(VerificationMethod<D>),
  Refer(DIDUrl<D>),
}

impl<D> MethodRef<D>
where
  D: DID,
{
  /// Returns a reference to the `MethodRef` id.
  pub fn id(&self) -> &DIDUrl<D> {
    match self {
      Self::Embed(inner) => inner.id(),
      Self::Refer(inner) => inner,
    }
  }

  /// Returns a reference to the [`MethodRef`] controller.
  ///
  /// Always `None` for [`MethodRef::Refer`].
  pub fn controller(&self) -> Option<&D> {
    match self {
      Self::Embed(inner) => Some(inner.controller()),
      Self::Refer(_) => None,
    }
  }

  /// Returns a `bool` indicating if the `MethodRef` is an embedded `Method`.
  #[inline]
  pub fn is_embedded(&self) -> bool {
    matches!(self, Self::Embed(_))
  }

  /// Returns a `bool` indicating if the `MethodRef` is a `DID` reference.
  #[inline]
  pub fn is_referred(&self) -> bool {
    matches!(self, Self::Refer(_))
  }

  /// Maps `MethodRef<D,T>` to `MethodRef<C,T>` by applying a function `f` to the inner
  /// [`VerificationMethod`] or [`DIDUrl`].
  pub fn map<C, F>(self, f: F) -> MethodRef<C>
  where
    C: DID,
    F: FnMut(D) -> C,
  {
    match self {
      MethodRef::Embed(method) => MethodRef::Embed(method.map(f)),
      MethodRef::Refer(id) => MethodRef::Refer(id.map(f)),
    }
  }

  /// Fallible version of [`MethodRef::map`].
  pub fn try_map<C, F, E>(self, f: F) -> Result<MethodRef<C>, E>
  where
    C: DID,
    F: FnMut(D) -> Result<C, E>,
  {
    Ok(match self {
      MethodRef::Embed(method) => MethodRef::Embed(method.try_map(f)?),
      MethodRef::Refer(id) => MethodRef::Refer(id.try_map(f)?),
    })
  }

  /// Returns the inner `Method` if this is an embedded `MethodRef`.
  ///
  /// Note: Returns `Err(self)` as a failure case.
  ///
  /// # Errors
  ///
  /// Fails if `MethodRef` is not an embedded method.
  pub fn try_into_embedded(self) -> Result<VerificationMethod<D>, Self> {
    match self {
      Self::Embed(inner) => Ok(inner),
      Self::Refer(_) => Err(self),
    }
  }

  /// Returns the inner `Method` if this is an referenced `MethodRef`.
  ///
  /// Note: Returns `Err(self)` as a failure case.
  ///
  /// # Errors
  ///
  /// Fails if `MethodRef` is not an referenced method.
  pub fn try_into_referenced(self) -> Result<DIDUrl<D>, Self> {
    match self {
      Self::Embed(_) => Err(self),
      Self::Refer(inner) => Ok(inner),
    }
  }
}

impl<D> Debug for MethodRef<D>
where
  D: DID + Debug,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    match self {
      Self::Embed(inner) => Debug::fmt(inner, f),
      Self::Refer(inner) => Debug::fmt(inner, f),
    }
  }
}

impl<D> From<VerificationMethod<D>> for MethodRef<D>
where
  D: DID,
{
  #[inline]
  fn from(other: VerificationMethod<D>) -> Self {
    Self::Embed(other)
  }
}

impl<D> From<DIDUrl<D>> for MethodRef<D>
where
  D: DID,
{
  #[inline]
  fn from(other: DIDUrl<D>) -> Self {
    Self::Refer(other)
  }
}

impl<D> AsRef<DIDUrl<D>> for MethodRef<D>
where
  D: DID,
{
  #[inline]
  fn as_ref(&self) -> &DIDUrl<D> {
    self.id()
  }
}

impl<D> KeyComparable for MethodRef<D>
where
  D: DID,
{
  type Key = DIDUrl<D>;

  #[inline]
  fn key(&self) -> &Self::Key {
    self.id()
  }
}
