// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_diff::Diff;
use identity_diff::DiffString;
use identity_diff::Result;
use serde::Deserialize;
use serde::Serialize;

use crate::jwk::JwkParamsEc;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DiffJwkParamsEc {
  pub crv: DiffString, // Curve
  pub x: DiffString,   // X Coordinate
  pub y: DiffString,   // Y Coordinate
}

impl Diff for JwkParamsEc {
  type Type = DiffJwkParamsEc;

  /// Finds the difference between `self` and `other` and returns the result as
  /// a [`DiffJwkParamsEc`].
  ///
  /// # Errors
  /// Errors if either `self` or `other` contains private components.
  fn diff(&self, other: &Self) -> identity_diff::Result<Self::Type> {
    if !(self.is_public() && other.is_public()) {
      return Err(identity_diff::Error::DiffError(
        "cannot diff jwk with private components".to_owned(),
      ));
    }
    Ok(DiffJwkParamsEc {
      crv: self.crv.diff(&other.crv)?,
      x: self.x.diff(&other.x)?,
      y: self.y.diff(&other.y)?,
    })
  }

  fn merge(&self, diff: Self::Type) -> identity_diff::Result<Self> {
    let crv = self.crv.merge(diff.crv)?;
    let x = self.x.merge(diff.x)?;
    let y = self.y.merge(diff.y)?;
    Ok(JwkParamsEc { crv, x, y, d: None })
  }

  fn from_diff(diff: Self::Type) -> identity_diff::Result<Self> {
    let DiffJwkParamsEc { crv, x, y } = diff;
    Ok(Self {
      crv: Diff::from_diff(crv)?,
      x: Diff::from_diff(x)?,
      y: Diff::from_diff(y)?,
      d: None,
    })
  }

  /// Converts the [`JwkParamsEc`] into [`DiffJwkParamsEc`].
  ///
  /// # Errors
  /// Errors if the params contain a private component.
  fn into_diff(mut self) -> identity_diff::Result<Self::Type> {
    if !self.is_public() {
      return Err(identity_diff::Error::ConversionError(
        "cannot convert jwk with private components to diff".to_owned(),
      ));
    }

    let (crv, x, y): (String, String, String) = {
      // Cannot directly destructure because of #[zeroize(drop)]
      let JwkParamsEc {
        ref mut crv,
        ref mut x,
        ref mut y,
        ..
      } = self;
      (std::mem::take(crv), std::mem::take(x), std::mem::take(y))
    };

    Ok(DiffJwkParamsEc {
      crv: crv.into_diff()?,
      x: x.into_diff()?,
      y: y.into_diff()?,
    })
  }
}
