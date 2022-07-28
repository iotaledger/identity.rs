// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::did::DIDError;
use crate::did::DIDUrl;
use crate::did::DID;

/// Convert to [`DIDUrl<D>`](DIDUrl).
pub trait ToDIDUrl<D: DID> {
  /// Constructs a [`DIDUrl`] by attempting to append a string representing a path, query, and/or
  /// fragment to this [`DID`].
  ///
  /// See [`DIDUrl::join`].
  ///
  /// # Errors
  ///
  /// Returns `Err` if any base or relative DID segments are invalid.
  fn join(self, value: impl AsRef<str>) -> Result<DIDUrl<D>, DIDError>;

  /// Clones the [`DID`] into a [`DIDUrl`] of the same method.
  fn to_url(&self) -> DIDUrl<D>;
}

/// Blanket implementation for `DID -> DIDUrl`.
impl<D> ToDIDUrl<D> for D
where
  D: DID,
{
  fn join(self, value: impl AsRef<str>) -> Result<DIDUrl<D>, DIDError> {
    DIDUrl::new(self, None).join(value)
  }

  fn to_url(&self) -> DIDUrl<D> {
    DIDUrl::new(self.clone(), None)
  }
}

/// Convert into [`DIDUrl<D>`](DIDUrl).
///
/// Workaround for lack of specialisation preventing a generic `From` implementation for
/// different [`DIDUrl`] structs.
pub trait IntoDIDUrl<D: DID> {
  /// Construct a [`DIDUrl<D>`](DIDUrl) from this.
  fn into_url(self) -> DIDUrl<D>;
}

/// Blanket implementation for `DID -> DIDUrl`.
impl<D> IntoDIDUrl<D> for D
where
  D: DID,
{
  fn into_url(self) -> DIDUrl<D> {
    DIDUrl::new(self, None)
  }
}

/// Blanket implementation for `DIDUrl<D> -> DIDUrl<E>`.
impl<D, E> IntoDIDUrl<E> for DIDUrl<D>
where
  D: DID + Into<E>,
  E: DID,
{
  fn into_url(self) -> DIDUrl<E> {
    DIDUrl::from(self)
  }
}
