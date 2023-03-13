// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use super::Deserialize;
use super::Diff;
use super::DiffResult;
use super::DiffString;
use super::Serialize;

use crate::jwk::JwkParamsRsa;

/// Represents the difference of two [`JwkParamsRsa`](JwkParamsRsa) without any private
/// components.   
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct DiffJwkParamsRsa {
  pub n: DiffString, // Modulus
  pub e: DiffString, // Exponent
}

impl Diff for JwkParamsRsa {
  type Type = DiffJwkParamsRsa;

  /// Finds the difference between `self` and `other` and returns the result as
  /// a [`DiffJwkParamsRsa`].
  ///
  /// # Errors
  /// Errors if either `self` or `other` contains private components.
  fn diff(&self, other: &Self) -> DiffResult<Self::Type> {
    if !(self.is_public() && other.is_public()) {
      return Err(identity_diff::Error::DiffError(
        "cannot diff jwk with private components".to_owned(),
      ));
    }
    Ok(DiffJwkParamsRsa {
      n: self.n.diff(&other.n)?,
      e: self.e.diff(&other.e)?,
    })
  }

  fn merge(&self, diff: Self::Type) -> DiffResult<Self> {
    let n = self.n.merge(diff.n)?;
    let e = self.e.merge(diff.e)?;
    Ok(JwkParamsRsa {
      n,
      e,
      d: None,
      p: None,
      q: None,
      dp: None,
      dq: None,
      qi: None,
      oth: None,
    })
  }

  fn from_diff(diff: Self::Type) -> DiffResult<Self> {
    let DiffJwkParamsRsa { n, e } = diff;
    Ok(JwkParamsRsa {
      n: Diff::from_diff(n)?,
      e: Diff::from_diff(e)?,
      d: None,
      p: None,
      q: None,
      dp: None,
      dq: None,
      qi: None,
      oth: None,
    })
  }

  /// Converts the [`JwkParamsRsa`] into [`DiffJwkParamsRsa`].
  ///
  /// # Errors
  /// Errors if the params contain a private component.
  fn into_diff(mut self) -> DiffResult<Self::Type> {
    self.take_diff()
  }
}

impl JwkParamsRsa {
  /// Obtain a [`DiffJwkParamsRsa`] from a [`&mut JwkParamsRsa`](JwkParamsRsa) leaving
  /// empty strings as public parameters in `self`.
  ///
  /// # Errors
  /// Errors immediately if the params contain a private component.
  ///
  /// # Motivation
  /// [`JwkParamsRsa`] cannot directly be destructured because of [zeroize(drop)]
  /// hence this provides workaround to enable a cheap implementation of `into_diff`.
  pub(super) fn take_diff(&mut self) -> DiffResult<DiffJwkParamsRsa> {
    if !self.is_public() {
      return Err(identity_diff::Error::ConversionError(
        "cannot convert jwk with private components to diff".to_owned(),
      ));
    }

    let (n, e): (String, String) = {
      // Cannot directly destructure because of #[zeroize(drop)]
      let JwkParamsRsa {
        ref mut n, ref mut e, ..
      } = self;
      (std::mem::take(n), std::mem::take(e))
    };

    Ok(DiffJwkParamsRsa {
      n: n.into_diff()?,
      e: e.into_diff()?,
    })
  }
}
