// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::jwk::Jwk;
use crate::jwk::JwkOperation;
use crate::jwk::JwkType;
use crate::jwk::JwkUse;
use identity_core::common::Url;
use identity_core::diff::Diff;
use identity_core::diff::DiffOption;
use identity_core::diff::Result as DiffResult;
use serde::Deserialize;
use serde::Serialize;

use super::DiffJwkParams;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiffJwk {
  kty: <JwkType as Diff>::Type,
  use_: DiffOption<JwkUse>,
  key_ops: DiffOption<Vec<JwkOperation>>,
  alg: DiffOption<String>,
  kid: DiffOption<String>,
  x5u: DiffOption<Url>,
  x5c: DiffOption<Vec<String>>,
  x5t: DiffOption<String>,
  x5t_s256: DiffOption<String>,
  params: DiffJwkParams,
}

impl Diff for Jwk {
  type Type = DiffJwk;
  /// Finds the difference of `self` and `other`. This is guaranteed to error if either
  /// `self` or `other` contain params with private key components.
  fn diff(&self, other: &Self) -> DiffResult<Self::Type> {
    let kty = self.kty.diff(&other.kty)?;
    let use_ = self.use_.diff(&other.use_)?;
    let key_ops = self.key_ops.diff(&other.key_ops)?;
    let alg = self.alg.diff(&other.alg)?;
    let kid = self.kid.diff(&other.kid)?;
    let x5u = self.x5u.diff(&other.x5u)?;
    let x5c = self.x5c.diff(&other.x5c)?;
    let x5t = self.x5t.diff(&other.x5t)?;
    let x5t_s256 = self.x5t_s256.diff(&other.x5t_s256)?;
    let params = self.params.diff(&other.params)?;
    Ok(DiffJwk {
      kty,
      use_,
      key_ops,
      alg,
      kid,
      x5u,
      x5c,
      x5t,
      x5t_s256,
      params,
    })
  }
  fn merge(&self, diff: Self::Type) -> DiffResult<Self> {
    let DiffJwk {
      kty,
      use_,
      key_ops,
      alg,
      kid,
      x5u,
      x5c,
      x5t,
      x5t_s256,
      params,
    } = diff;
    let kty = self.kty.merge(kty)?;
    let use_ = self.use_.merge(use_)?;
    let key_ops = self.key_ops.merge(key_ops)?;
    let alg = self.alg.merge(alg)?;
    let kid = self.kid.merge(kid)?;
    let x5u = self.x5u.merge(x5u)?;
    let x5c = self.x5c.merge(x5c)?;
    let x5t = self.x5t.merge(x5t)?;
    let x5t_s256 = self.x5t_s256.merge(x5t_s256)?;
    let params = self.params.merge(params)?;
    Ok(Jwk {
      kty,
      use_,
      key_ops,
      alg,
      kid,
      x5u,
      x5c,
      x5t,
      x5t_s256,
      params,
    })
  }

  fn from_diff(diff: Self::Type) -> identity_core::diff::Result<Self> {
    let DiffJwk {
      kty,
      use_,
      key_ops,
      alg,
      kid,
      x5u,
      x5c,
      x5t,
      x5t_s256,
      params,
    } = diff;
    let kty = Diff::from_diff(kty)?;
    let use_ = Diff::from_diff(use_)?;
    let key_ops = Diff::from_diff(key_ops)?;
    let alg = Diff::from_diff(alg)?;
    let kid = Diff::from_diff(kid)?;
    let x5u = Diff::from_diff(x5u)?;
    let x5c = Diff::from_diff(x5c)?;
    let x5t = Diff::from_diff(x5t)?;
    let x5t_s256 = Diff::from_diff(x5t_s256)?;
    let params = Diff::from_diff(params)?;
    Ok(Jwk {
      kty,
      use_,
      key_ops,
      alg,
      kid,
      x5u,
      x5c,
      x5t,
      x5t_s256,
      params,
    })
  }

  /// Convert `self` into [`DiffJwk`]. This is guaranteed to error if `self`
  /// contains params with private key components.
  fn into_diff(mut self) -> identity_core::diff::Result<Self::Type> {
    self.take_diff()
  }
}

impl Jwk {
  fn take_diff(&mut self) -> DiffResult<DiffJwk> {
    Ok(DiffJwk {
      kty: self.kty.into_diff()?,
      use_: self.use_.into_diff()?,
      key_ops: self.key_ops.take().into_diff()?,
      alg: self.alg.take().into_diff()?,
      kid: self.kid.take().into_diff()?,
      x5u: self.x5u.take().into_diff()?,
      x5c: self.x5c.take().into_diff()?,
      x5t: self.x5t.take().into_diff()?,
      x5t_s256: self.x5t_s256.take().into_diff()?,
      params: self.params.take_diff()?,
    })
  }
}
