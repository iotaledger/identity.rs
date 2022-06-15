// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::diff::Diff;
use identity_core::diff::DiffString;

use crate::did::IotaDID;

impl Diff for IotaDID {
  type Type = DiffString;

  fn diff(&self, other: &Self) -> identity_core::diff::Result<Self::Type> {
    self.to_string().diff(&other.to_string())
  }

  fn merge(&self, diff: Self::Type) -> identity_core::diff::Result<Self> {
    self
      .to_string()
      .merge(diff)
      .and_then(|this| Self::parse(&this).map_err(identity_core::diff::Error::merge))
  }

  fn from_diff(diff: Self::Type) -> identity_core::diff::Result<Self> {
    String::from_diff(diff).and_then(|this| Self::parse(&this).map_err(identity_core::diff::Error::convert))
  }

  fn into_diff(self) -> identity_core::diff::Result<Self::Type> {
    self.to_string().into_diff()
  }
}
