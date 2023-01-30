// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::Diff;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashSet;
use std::fmt::Debug;
use std::fmt::Formatter;

use std::hash::Hash;
use std::iter::empty;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct DiffHashSet<T: Diff>(#[serde(skip_serializing_if = "Option::is_none")] pub Option<Vec<InnerValue<T>>>);

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InnerValue<T: Diff> {
  Add(<T as Diff>::Type),
  Remove { remove: <T as Diff>::Type },
}

impl<T> Diff for HashSet<T>
where
  T: Debug + Clone + PartialEq + Eq + Diff + Hash + for<'de> Deserialize<'de> + Serialize,
{
  type Type = DiffHashSet<T>;

  fn diff(&self, other: &Self) -> crate::Result<Self::Type> {
    Ok(DiffHashSet(if self == other {
      None
    } else {
      let mut val_diffs: Vec<InnerValue<T>> = vec![];
      for add in other.difference(self) {
        let add = add.clone().into_diff()?;
        val_diffs.push(InnerValue::Add(add));
      }

      for remove in self.difference(other) {
        let remove = remove.clone().into_diff()?;
        val_diffs.push(InnerValue::Remove { remove });
      }

      Some(val_diffs)
    }))
  }

  fn merge(&self, diff: Self::Type) -> crate::Result<Self> {
    match diff.0 {
      None => Ok(self.clone()),
      Some(val_diffs) => {
        let mut new: Self = self.clone();
        for val_diff in val_diffs {
          match val_diff {
            InnerValue::Add(val) => {
              new.insert(<T>::from_diff(val)?);
            }
            InnerValue::Remove { remove } => {
              new.remove(&(<T>::from_diff(remove)?));
            }
          }
        }
        Ok(new)
      }
    }
  }

  fn into_diff(self) -> crate::Result<Self::Type> {
    Ok(DiffHashSet(if self.is_empty() {
      None
    } else {
      let mut diffs: Vec<InnerValue<T>> = vec![];
      for val in self {
        diffs.push(InnerValue::Add(val.into_diff()?));
      }
      Some(diffs)
    }))
  }

  fn from_diff(diff: Self::Type) -> crate::Result<Self> {
    let mut set = Self::new();
    if let Some(vals) = diff.0 {
      for val in vals {
        match val {
          InnerValue::Add(val) => {
            set.insert(<T>::from_diff(val)?);
          }
          InnerValue::Remove { remove } => {
            let val = <T>::from_diff(remove)?;
            set.remove(&val);
          }
        }
      }
    }
    Ok(set)
  }
}

impl<T> Debug for DiffHashSet<T>
where
  T: Debug + Diff,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    write!(f, "DiffHashSet")?;
    let mut buf = f.debug_list();
    if let Some(d) = &self.0 {
      buf.entries(d.iter());
    } else {
      buf.entries(empty::<Vec<InnerValue<T>>>());
    }
    buf.finish()
  }
}

impl<T> Debug for InnerValue<T>
where
  T: Debug + Diff,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    match &self {
      Self::Add(val) => f.debug_tuple("Add").field(val).finish(),
      Self::Remove { remove } => f.debug_tuple("Remove").field(remove).finish(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::collections::HashSet;

  macro_rules! set {
        ($($val:expr),* $(,)?) => {{ #[allow(redundant_semicolons)] {
            let mut set = HashSet::new();
            $( set.insert($val); )* ;
            set
        }}}
    }

  #[test]
  fn test_hashset_diff() {
    let s: HashSet<String> = set! {
        "test".into(),
        "foo".into(),
    };

    let s1: HashSet<String> = set! {
        "test".into(),
        "foo".into(),
    };

    let diff = s.diff(&s1).unwrap();
    let expected = DiffHashSet(None);

    assert_eq!(diff, expected);
    let s2 = s.merge(diff).unwrap();

    assert_eq!(s, s2);
    assert_eq!(s1, s2);
  }

  #[test]
  fn test_hashset_diff_add_and_remove() {
    let s: HashSet<String> = set! {
        "test".into(),
        "foo".into(),
        "faux".into(),
    };

    let s1: HashSet<String> = set! {
        "test".into(),
        "foo".into(),
        "bar".into(),
    };

    let diff = s.diff(&s1).unwrap();

    let json = serde_json::to_string(&diff).unwrap();

    println!("{json}");

    let diff: DiffHashSet<String> = serde_json::from_str(&json).unwrap();

    println!("{diff:?}");
  }
}
