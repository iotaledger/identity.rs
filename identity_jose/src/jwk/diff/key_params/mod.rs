// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use identity_core::diff::Diff;
use identity_core::diff::DiffString;
use identity_core::diff::Result as DiffResult;
use serde::Deserialize;
use serde::Serialize;
mod ec;
mod oct;
mod okp;
mod rsa;
pub use ec::*;
pub use oct::*;
pub use okp::*;
pub use rsa::*;

use crate::jwk::JwkParams;

/// The difference of two [`JwkParams`].
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum DiffJwkParams {
  Ec(DiffJwkParamsEc),
  Rsa(DiffJwkParamsRsa),
  Oct(DiffJwkParamsOct),
  Okp(DiffJwkParamsOkp),
}

impl Diff for JwkParams {
  type Type = DiffJwkParams;

  /// Finds the difference between `self` and `other` and returns the result represented as a [`DiffJwkParams`].
  ///
  /// # Errors
  /// Errors if `self` or `others` contain private components.
  fn diff(&self, other: &Self) -> DiffResult<Self::Type> {
    match (self, other) {
      (Self::Okp(a), Self::Okp(b)) => Ok(DiffJwkParams::Okp(a.diff(b)?)),
      (Self::Ec(a), Self::Ec(b)) => Ok(DiffJwkParams::Ec(a.diff(b)?)),
      (Self::Oct(a), Self::Oct(b)) => Ok(DiffJwkParams::Oct(a.diff(b)?)),
      (Self::Rsa(a), Self::Rsa(b)) => Ok(DiffJwkParams::Rsa(a.diff(b)?)),
      (_, _) => other.clone().into_diff(),
    }
  }

  fn merge(&self, diff: Self::Type) -> DiffResult<Self> {
    match (self, diff) {
      (Self::Okp(a), DiffJwkParams::Okp(b)) => a.merge(b).map(Self::Okp),
      (Self::Ec(a), DiffJwkParams::Ec(b)) => a.merge(b).map(Self::Ec),
      (Self::Oct(a), DiffJwkParams::Oct(b)) => a.merge(b).map(Self::Oct),
      (Self::Rsa(a), DiffJwkParams::Rsa(b)) => a.merge(b).map(Self::Rsa),
      (_, diff) => Self::from_diff(diff),
    }
  }

  fn from_diff(diff: Self::Type) -> DiffResult<Self> {
    match diff {
      DiffJwkParams::Okp(diff_params) => Diff::from_diff(diff_params).map(Self::Okp),
      DiffJwkParams::Ec(diff_params) => Diff::from_diff(diff_params).map(Self::Ec),
      DiffJwkParams::Oct(diff_params) => Diff::from_diff(diff_params).map(Self::Oct),
      DiffJwkParams::Rsa(diff_params) => Diff::from_diff(diff_params).map(Self::Rsa),
    }
  }

  /// Converts the [`JwkParams`] into [`DiffJwkParams`].
  ///
  /// # Errors
  /// Errors if the [`JwkParams`] contain private components.
  fn into_diff(mut self) -> DiffResult<Self::Type> {
    match &mut self {
      Self::Okp(params) => params.take_diff().map(DiffJwkParams::Okp),
      Self::Ec(params) => params.take_diff().map(DiffJwkParams::Ec),
      Self::Oct(params) => params.take_diff().map(DiffJwkParams::Oct),
      Self::Rsa(params) => params.take_diff().map(DiffJwkParams::Rsa),
    }
  }
}
