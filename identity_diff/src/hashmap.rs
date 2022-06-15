// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::Diff;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::fmt::Formatter;

use std::hash::Hash;
use std::iter::empty;

/// Inner value of the `DiffHashMap` type.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InnerValue<K, V: Diff> {
  // Logs if a value has changed between the two types being Diffed.
  Change {
    #[serde(rename = "c:k")]
    key: K,
    #[serde(rename = "c:v")]
    value: <V as Diff>::Type,
  },
  // Logs an addition.
  Add {
    #[serde(rename = "a:k")]
    key: K,
    #[serde(rename = "a:v")]
    value: <V as Diff>::Type,
  },
  // Logs a removal.
  Remove {
    #[serde(rename = "r:k")]
    key: K,
  },
}

/// A `DiffHashMap` type which represents a Diffed `HashMap`.
/// By default this value is transparent to `serde`.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DiffHashMap<K: Diff, V: Diff>(
  #[serde(skip_serializing_if = "Option::is_none")] pub Option<Vec<InnerValue<K, V>>>,
);

/// Diff Implementation on a HashMap<K, V>
impl<K, V> Diff for HashMap<K, V>
where
  K: Clone + Debug + PartialEq + Eq + Hash + Diff + for<'de> Deserialize<'de> + Serialize,
  V: Clone + Debug + PartialEq + Diff + for<'de> Deserialize<'de> + Serialize,
{
  /// the Diff type of the HashMap<K, V>
  type Type = DiffHashMap<K, V>;

  /// Diffs two `HashMaps`; `self` and `other` and creates a `DiffHashMap<K, V>`
  fn diff(&self, other: &Self) -> crate::Result<Self::Type> {
    let old: HashSet<&K> = self.keys().collect();
    let new: HashSet<&K> = other.keys().collect();

    let changed_keys = old.intersection(&new).filter(|k| self[k] != other[k]);
    let removed_keys = old.difference(&new);
    let added_keys = new.difference(&old);

    let mut changes: Vec<InnerValue<K, V>> = Vec::new();

    for key in changed_keys {
      let (old_val, new_val): (&V, &V) = (&self[key], &other[key]);

      let diff = old_val.diff(new_val)?;

      changes.push(InnerValue::Change {
        key: (*key).clone(),
        value: diff,
      });
    }
    for key in added_keys {
      changes.push(InnerValue::Add {
        key: (*key).clone(),
        value: other[key].clone().into_diff()?,
      });
    }
    for key in removed_keys {
      changes.push(InnerValue::Remove { key: (*key).clone() });
    }

    Ok(DiffHashMap(if changes.is_empty() { None } else { Some(changes) }))
  }

  /// Merges the changes in a `DiffHashMap<K, V>`, `diff` with a `HashMap<K, V>`, `self`.
  fn merge(&self, diff: Self::Type) -> crate::Result<Self> {
    let mut new = self.clone();

    for change in diff.0.into_iter().flatten() {
      match change {
        InnerValue::Change { key, value } => {
          let fake: &mut V = &mut *new.get_mut(&key).expect("Failed to get value");

          *fake = <V>::from_diff(value)?;
        }
        InnerValue::Add { key, value } => {
          new.insert(key, <V>::from_diff(value)?);
        }
        InnerValue::Remove { key } => {
          new.remove(&key);
        }
      }
    }

    Ok(new)
  }

  /// Converts a `DiffHashMap<K, V>`, `diff` into a `HashMap<K, V>`.
  fn from_diff(diff: Self::Type) -> crate::Result<Self> {
    let mut map = Self::new();
    if let Some(diff) = diff.0 {
      for (idx, elm) in diff.into_iter().enumerate() {
        match elm {
          InnerValue::Add { key, value } => {
            map.insert(key, <V>::from_diff(value)?);
          }
          _ => {
            panic!("Unable to create Diff at index: {:?}", idx);
          }
        }
      }
    }

    Ok(map)
  }

  /// Converts a `HashMap<K, V>`, `diff` into a `DiffHashMap<K, V>`.
  fn into_diff(self) -> crate::Result<Self::Type> {
    let mut changes: Vec<InnerValue<K, V>> = Vec::new();
    for (key, val) in self {
      changes.push(InnerValue::Add {
        key,
        value: val.into_diff()?,
      });
    }

    Ok(DiffHashMap(if changes.is_empty() { None } else { Some(changes) }))
  }
}

/// Debug implementation for the `DiffHashMap<K, V>` type.
impl<K, V> Debug for DiffHashMap<K, V>
where
  K: Debug + Diff,
  V: Debug + Diff,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    write!(f, "DiffHashMap")?;

    let mut buf = f.debug_list();

    if let Some(val) = &self.0 {
      buf.entries(val.iter());
    } else {
      buf.entries(empty::<Vec<InnerValue<K, V>>>());
    }
    buf.finish()
  }
}

/// Default implementation for the `DiffHashMap<K, V>` type.
impl<K, V> Default for DiffHashMap<K, V>
where
  K: Diff,
  V: Diff,
{
  fn default() -> Self {
    DiffHashMap(None)
  }
}

/// Debug implementation for the `InnerValue<K, V>` type.
impl<K, V> Debug for InnerValue<K, V>
where
  K: Debug + Diff,
  V: Debug + Diff,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    match &self {
      Self::Change { key, value } => f
        .debug_struct("Change")
        .field("key", key)
        .field("value", value)
        .finish(),
      Self::Add { key, value } => f.debug_struct("Add").field("key", key).field("value", value).finish(),
      Self::Remove { key } => f.debug_struct("Remove").field("key", key).finish(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::collections::HashMap;

  /// Quickly creates a simple map using `map! { "key" => "value"}
  macro_rules! map {
        ($($key:expr => $val:expr),* $(,)?) => {{
            let mut map = HashMap::new();
            $( map.insert($key, $val); )*
                map
        }}
    }

  #[test]
  fn test_hashmap_diff() {
    let m0: HashMap<String, usize> = map! {
        "test".into() => 300usize,
        "foo".into() => 10usize,
        "bar".into() => 20usize,
        "baz".into() => 1usize,
    };

    let m1: HashMap<String, usize> = map! {
        "test".into() => 300usize,
        "foo".into() => 0usize,
        "bar".into() => 20usize,
        "quux".into() => 10usize,
    };

    let diff = m0.diff(&m1).unwrap();

    let expected: DiffHashMap<String, usize> = DiffHashMap(Some(vec![
      InnerValue::Change {
        key: "foo".into(),
        value: 0usize.into_diff().unwrap(),
      },
      InnerValue::Add {
        key: "quux".into(),
        value: 10usize.into_diff().unwrap(),
      },
      InnerValue::Remove { key: "baz".into() },
    ]));

    assert_eq!(expected, diff);

    let m2 = m0.merge(diff).unwrap();

    assert_eq!(m1, m2);
  }
}
