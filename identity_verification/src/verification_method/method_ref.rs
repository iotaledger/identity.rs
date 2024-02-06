// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;

use identity_core::common::KeyComparable;

use crate::verification_method::VerificationMethod;
use identity_did::CoreDID;
use identity_did::DIDUrl;

/// A reference to a verification method, either a `DID` or embedded `Method`.
#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum MethodRef {
  /// A [`VerificationMethod`] embedded in a verification relationship.
  Embed(VerificationMethod),
  /// A reference to a [`VerificationMethod`] in a verification relationship.
  Refer(DIDUrl),
}

impl MethodRef {
  /// Returns a reference to the `MethodRef` id.
  pub fn id(&self) -> &DIDUrl {
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
  pub fn is_embedded(&self) -> bool {
    matches!(self, Self::Embed(_))
  }

  /// Returns a `bool` indicating if the `MethodRef` is a `DID` reference.
  #[inline]
  pub fn is_referred(&self) -> bool {
    matches!(self, Self::Refer(_))
  }

  /// Maps the [`MethodRef`] by applying a function `f` to the inner
  /// [`VerificationMethod`] or [`DIDUrl`]. This can be useful for DID methods
  /// where the DID is not known before the document has been published.
  pub fn map<F>(self, f: F) -> MethodRef
  where
    F: FnMut(CoreDID) -> CoreDID,
  {
    match self {
      MethodRef::Embed(method) => MethodRef::Embed(method.map(f)),
      MethodRef::Refer(id) => MethodRef::Refer(id.map(f)),
    }
  }

  /// Fallible version of [`MethodRef::map`].
  pub fn try_map<F, E>(self, f: F) -> Result<MethodRef, E>
  where
    F: FnMut(CoreDID) -> Result<CoreDID, E>,
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
  pub fn try_into_embedded(self) -> Result<VerificationMethod, Box<Self>> {
    match self {
      Self::Embed(inner) => Ok(inner),
      Self::Refer(_) => Err(self.into()),
    }
  }

  /// Returns the inner `Method` if this is an referenced `MethodRef`.
  ///
  /// Note: Returns `Err(self)` as a failure case.
  ///
  /// # Errors
  ///
  /// Fails if `MethodRef` is not an referenced method.
  pub fn try_into_referenced(self) -> Result<DIDUrl, Box<Self>> {
    match self {
      Self::Embed(_) => Err(self.into()),
      Self::Refer(inner) => Ok(inner),
    }
  }
}

impl Debug for MethodRef {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    match self {
      Self::Embed(inner) => Debug::fmt(inner, f),
      Self::Refer(inner) => Debug::fmt(inner, f),
    }
  }
}

impl From<VerificationMethod> for MethodRef {
  #[inline]
  fn from(other: VerificationMethod) -> Self {
    Self::Embed(other)
  }
}

impl From<DIDUrl> for MethodRef {
  #[inline]
  fn from(other: DIDUrl) -> Self {
    Self::Refer(other)
  }
}

impl AsRef<DIDUrl> for MethodRef {
  #[inline]
  fn as_ref(&self) -> &DIDUrl {
    self.id()
  }
}

impl KeyComparable for MethodRef {
  type Key = DIDUrl;

  #[inline]
  fn key(&self) -> &Self::Key {
    self.id()
  }
}
