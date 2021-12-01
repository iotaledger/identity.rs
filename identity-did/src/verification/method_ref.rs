// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;

use identity_core::common::Object;

use crate::did::CoreDID;
use crate::did::CoreDIDUrl;
use crate::verification::VerificationMethod;

/// A reference to a verification method, either a `DID` or embedded `Method`.
#[derive(Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum MethodRef<T = Object> {
  Embed(VerificationMethod<T>),
  Refer(CoreDIDUrl), // TODO: could this also be a relative DIDUrl, or another enum variant?
}

impl<T> MethodRef<T> {
  /// Returns a reference to the `MethodRef` id.
  pub fn id(&self) -> &CoreDIDUrl {
    match self {
      Self::Embed(inner) => inner.id(),
      Self::Refer(inner) => inner,
    }
  }

  /// Returns a reference to the [`MethodRef`] controller.
  ///
  /// Always `None` for [`MethodRef::Refer`].
  pub fn controller(&self) -> Option<&CoreDID> {
    match self {
      Self::Embed(inner) => Some(inner.controller()),
      Self::Refer(_) => None,
    }
  }

  /// Returns a `bool` indicating if the `MethodRef` is an embedded `Method`.
  #[inline]
  pub const fn is_embedded(&self) -> bool {
    matches!(self, Self::Embed(_))
  }

  /// Returns a `bool` indicating if the `MethodRef` is a `DID` reference.
  #[inline]
  pub const fn is_referred(&self) -> bool {
    matches!(self, Self::Refer(_))
  }

  /// Returns the inner `Method` if this is an embedded `MethodRef`.
  ///
  /// Note: Returns `Err(self)` as a failure case.
  ///
  /// # Errors
  ///
  /// Fails if `MethodRef` is not an embedded method.
  pub fn try_into_embedded(self) -> Result<VerificationMethod<T>, Self> {
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
  pub fn try_into_referenced(self) -> Result<CoreDIDUrl, Self> {
    match self {
      Self::Embed(_) => Err(self),
      Self::Refer(inner) => Ok(inner),
    }
  }
}

impl<T> Debug for MethodRef<T>
where
  T: Debug,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    match self {
      Self::Embed(inner) => Debug::fmt(inner, f),
      Self::Refer(inner) => Debug::fmt(inner, f),
    }
  }
}

impl<T> From<VerificationMethod<T>> for MethodRef<T> {
  #[inline]
  fn from(other: VerificationMethod<T>) -> Self {
    Self::Embed(other)
  }
}

impl<T> From<CoreDIDUrl> for MethodRef<T> {
  #[inline]
  fn from(other: CoreDIDUrl) -> Self {
    Self::Refer(other)
  }
}

impl<T> AsRef<CoreDIDUrl> for MethodRef<T> {
  #[inline]
  fn as_ref(&self) -> &CoreDIDUrl {
    self.id()
  }
}
