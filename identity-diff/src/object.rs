// Copyright 2020-2022 IOTA Stiftung
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

#[cfg(test)]
mod tests {
  use super::*;
  use crate::hashmap::InnerValue;
  use serde_json::json;

  #[test]
  fn test_diff_empty() {
    // Ensure diff is none.
    let a: ObjectSrc = ObjectSrc::default();
    let b: ObjectSrc = ObjectSrc::default();
    let diff: DiffObject = Diff::diff(&a, &b).unwrap();
    assert!(diff.0.is_none());
    let merge: ObjectSrc = a.merge(diff.clone()).unwrap();
    assert_eq!(merge, a);

    // Test serde round-trip.
    let serialized: String = serde_json::to_string(&diff).unwrap();
    let deserialized: DiffObject = serde_json::from_str(&serialized).unwrap();
    assert_eq!(diff, deserialized)
  }

  #[test]
  fn test_diff() {
    let mut a: ObjectSrc = ObjectSrc::new();
    a.insert("foo".into(), 12.into());
    a.insert("bar".into(), 34.into());
    a.insert("baz".into(), 56.into());
    a.insert("qux".into(), 78.into());

    let mut b: ObjectSrc = ObjectSrc::new();
    b.insert("foo".into(), 56.into());
    b.insert("thud".into(), 9.into());
    b.insert("bar".into(), 34.into());
    b.insert("qux".into(), 78.into());

    let diff: DiffObject = Diff::diff(&a, &b).unwrap();
    let expected: DiffObject = DiffHashMap(Some(vec![
      InnerValue::Change {
        key: "foo".into(),
        value: json!(56).into_diff().unwrap(),
      },
      InnerValue::Add {
        key: "thud".into(),
        value: json!(9).into_diff().unwrap(),
      },
      InnerValue::Remove { key: "baz".into() },
    ]));
    assert_eq!(expected, diff);

    let merge: ObjectSrc = a.merge(diff.clone()).unwrap();
    assert_eq!(merge, b);

    // Test serde round-trip.
    let serialized: String = serde_json::to_string(&diff).unwrap();
    let deserialized: DiffObject = serde_json::from_str(&serialized).unwrap();
    assert_eq!(diff, deserialized)
  }
}
