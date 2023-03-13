// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use super::Deserialize;
use super::Diff;
use super::DiffResult;
use super::DiffString;
use super::Serialize;

use crate::jwk::JwkParamsOct;

/// Represents the difference of two [`JwkParamsOct`].
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct DiffJwkParamsOct {
  pub k: DiffString, // Key value
}

impl Diff for JwkParamsOct {
  type Type = DiffJwkParamsOct;

  /// Finds the difference between `self` and `other` and returns the result as
  /// a [`DiffJwkParamsOct`].
  fn diff(&self, other: &Self) -> identity_diff::Result<Self::Type> {
    Ok(DiffJwkParamsOct {
      k: self.k.diff(&other.k)?,
    })
  }

  fn merge(&self, diff: Self::Type) -> identity_diff::Result<Self> {
    let k: String = self.k.merge(diff.k)?;
    Ok(Self { k })
  }

  fn from_diff(diff: Self::Type) -> identity_diff::Result<Self> {
    let k: String = Diff::from_diff(diff.k)?;
    Ok(Self { k })
  }

  /// Converts the [`JwkParamsOct`] into [`DiffJwkParamsOct`].
  fn into_diff(mut self) -> DiffResult<Self::Type> {
    self.take_diff()
  }
}

impl JwkParamsOct {
  /// Converts a [`&mut JwkParamsOct`](JwkParamsOct) to [`DiffJwkParamsOct`] leaving
  /// empty strings as public parameters in `self`.
  ///
  ///
  /// # Motivation
  /// [`JwkParamsOct`] cannot directly be destructured because of [zeroize(drop)]
  /// hence this provides workaround to enable a cheap implementation of `into_diff`.
  pub(super) fn take_diff(&mut self) -> DiffResult<DiffJwkParamsOct> {
    let k: String = {
      let JwkParamsOct { ref mut k } = self;
      std::mem::take(k)
    };

    Ok(DiffJwkParamsOct { k: k.into_diff()? })
  }
}
