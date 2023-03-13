// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use super::Deserialize;
use super::Diff;
use super::DiffResult;
use super::DiffString;
use super::Serialize;

use crate::jwk::JwkParamsOkp;
/// Represents the difference of two [`JwkParamsOkps`](JwkParamsOkp) without any private
/// components.   
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct DiffJwkParamsOkp {
  pub crv: DiffString, // Key SubType
  pub x: DiffString,   // Public Key
}

impl Diff for JwkParamsOkp {
  type Type = DiffJwkParamsOkp;

  /// Finds the difference between `self` and `other` and returns the result as
  /// a [`DiffJwkParamsOkp`].
  ///
  /// # Errors
  /// Errors if either `self` or `other` contains private components.
  fn diff(&self, other: &Self) -> DiffResult<Self::Type> {
    if !(self.is_public() && other.is_public()) {
      return Err(identity_core::diff::Error::DiffError(
        "cannot diff jwk okp params with private components".to_owned(),
      ));
    }
    Ok(DiffJwkParamsOkp {
      crv: self.crv.diff(&other.crv)?,
      x: self.x.diff(&other.x)?,
    })
  }

  fn merge(&self, diff: Self::Type) -> DiffResult<Self> {
    let crv = self.crv.merge(diff.crv)?;
    let x = self.x.merge(diff.x)?;
    Ok(JwkParamsOkp { crv, x, d: None })
  }

  fn from_diff(diff: Self::Type) -> DiffResult<Self> {
    let DiffJwkParamsOkp { crv, x } = diff;
    Ok(Self {
      crv: Diff::from_diff(crv)?,
      x: Diff::from_diff(x)?,
      d: None,
    })
  }

  /// Converts the [`JwkParamsOkp`] into [`DiffJwkParamsOkp`].
  ///
  /// # Errors
  /// Errors if the params contain a private component.
  fn into_diff(mut self) -> DiffResult<Self::Type> {
    self.take_diff()
  }
}

impl JwkParamsOkp {
  /// Obtain a [`DiffJwkParamsOkp`] from a [`&mut JwkParamsOkp`](JwkParamsOkp) leaving
  /// empty strings as public parameters in `self`.
  ///
  /// # Errors
  /// Errors immediately if the params contain a private component.
  ///
  /// # Motivation
  /// [`JwkParamsOkp`] cannot directly be destructured because of [zeroize(drop)]
  /// hence this provides workaround to enable a cheap implementation of `into_diff`.
  pub(super) fn take_diff(&mut self) -> DiffResult<DiffJwkParamsOkp> {
    if !self.is_public() {
      return Err(identity_core::diff::Error::ConversionError(
        "cannot convert jwk okp params with private components to diff".to_owned(),
      ));
    }

    let (crv, x): (String, String) = {
      // Cannot directly destructure because of #[zeroize(drop)]
      let JwkParamsOkp {
        ref mut crv, ref mut x, ..
      } = self;
      (std::mem::take(crv), std::mem::take(x))
    };

    Ok(DiffJwkParamsOkp {
      crv: crv.into_diff()?,
      x: x.into_diff()?,
    })
  }
}
