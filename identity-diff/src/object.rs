// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde_json::Value;
use std::collections::BTreeMap;
use std::collections::HashMap;

use crate::error::Result;
use crate::hashmap::DiffHashMap;
use crate::traits::Diff;

pub type DiffObject = DiffHashMap<String, Value>;

type ObjectSrc = BTreeMap<String, Value>;
type ObjectDst = HashMap<String, Value>;

impl Diff for ObjectSrc {
  type Type = DiffObject;

  fn diff(&self, other: &Self) -> Result<Self::Type> {
    let a: ObjectDst = self.clone().into_iter().collect();
    let b: ObjectDst = other.clone().into_iter().collect();

    a.diff(&b)
  }

  fn merge(&self, diff: Self::Type) -> Result<Self> {
    let this: ObjectDst = self.clone().into_iter().collect();
    let this: ObjectDst = this.merge(diff)?;

    Ok(this.into_iter().collect())
  }

  fn from_diff(diff: Self::Type) -> Result<Self> {
    Ok(ObjectDst::from_diff(diff)?.into_iter().collect())
  }

  fn into_diff(self) -> Result<Self::Type> {
    self.into_iter().collect::<ObjectDst>().into_diff()
  }
}
