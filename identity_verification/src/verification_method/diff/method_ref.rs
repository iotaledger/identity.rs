// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::diff::Diff;
use identity_core::diff::Error;
use identity_core::diff::Result;
use serde::Deserialize;
use serde::Serialize;

use super::diff_method::DiffMethod;
use crate::verification_method::MethodRef;

use identity_did::DIDUrl;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DiffMethodRef {
  Embed(#[serde(skip_serializing_if = "Option::is_none")] Option<DiffMethod>),
  Refer(#[serde(skip_serializing_if = "Option::is_none")] Option<<DIDUrl as Diff>::Type>),
}

impl Diff for MethodRef {
  type Type = DiffMethodRef;

  fn diff(&self, other: &Self) -> Result<Self::Type> {
    match (self, other) {
      (Self::Embed(a), Self::Embed(b)) if a == b => Ok(DiffMethodRef::Embed(None)),
      (Self::Embed(a), Self::Embed(b)) => a.diff(b).map(Some).map(DiffMethodRef::Embed),
      (Self::Refer(a), Self::Refer(b)) if a == b => Ok(DiffMethodRef::Refer(None)),
      (Self::Refer(a), Self::Refer(b)) => a.diff(b).map(Some).map(DiffMethodRef::Refer),
      (_, _) => other.clone().into_diff(),
    }
  }

  fn merge(&self, diff: Self::Type) -> Result<Self> {
    match (self, diff) {
      (Self::Embed(a), DiffMethodRef::Embed(Some(b))) => a.merge(b).map(Self::Embed),
      (Self::Embed(a), DiffMethodRef::Embed(None)) => Ok(Self::Embed(a.clone())),
      (Self::Refer(a), DiffMethodRef::Refer(Some(b))) => a.merge(b).map(Self::Refer),
      (Self::Refer(a), DiffMethodRef::Refer(None)) => Ok(Self::Refer(a.clone())),
      (_, diff) => Self::from_diff(diff),
    }
  }

  fn from_diff(diff: Self::Type) -> Result<Self> {
    match diff {
      DiffMethodRef::Embed(Some(value)) => Diff::from_diff(value).map(Self::Embed),
      DiffMethodRef::Embed(None) => Err(Error::convert("Invalid MethodRef Diff")),
      DiffMethodRef::Refer(Some(value)) => Diff::from_diff(value).map(Self::Refer),
      DiffMethodRef::Refer(None) => Err(Error::convert("Invalid MethodRef Diff")),
    }
  }

  fn into_diff(self) -> Result<Self::Type> {
    match self {
      Self::Embed(value) => value.into_diff().map(Some).map(DiffMethodRef::Embed),
      Self::Refer(value) => value.to_string().into_diff().map(Some).map(DiffMethodRef::Refer),
    }
  }
}
