use crate::Diff;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fmt::{Debug, Formatter, Result as FmtResult},
    hash::Hash,
    iter::empty,
};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct DiffHashSet<T: Diff>(#[serde(skip_serializing_if = "Option::is_none")] pub Option<Vec<InnerValue<T>>>);

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum InnerValue<T: Diff> {
    Add(<T as Diff>::Type),
    Remove(<T as Diff>::Type),
}

impl<T> Diff for HashSet<T>
where
    T: Debug + Clone + PartialEq + Ord + Diff + Hash + for<'de> Deserialize<'de> + Serialize,
{
    type Type = DiffHashSet<T>;

    fn diff(&self, other: &Self) -> Self::Type {
        DiffHashSet(if self == other {
            None
        } else {
            let mut val_diffs: Vec<InnerValue<T>> = vec![];
            for add in other.difference(&self) {
                let add = add.clone().into_diff();
                val_diffs.push(InnerValue::Add(add));
            }

            Some(val_diffs)
        })
    }

    fn merge(&self, diff: Self::Type) -> Self {
        match diff.0 {
            None => self.clone(),
            Some(val_diffs) => {
                let mut new: Self = self.clone();
                for val_diff in val_diffs {
                    match val_diff {
                        InnerValue::Add(val) => {
                            new.insert(<T>::from_diff(val));
                        }
                        InnerValue::Remove(val) => {
                            new.remove(&(<T>::from_diff(val)));
                        }
                    }
                }
                new
            }
        }
    }

    fn into_diff(self) -> Self::Type {
        DiffHashSet(if self.is_empty() {
            None
        } else {
            let mut diffs: Vec<InnerValue<T>> = vec![];
            for val in self {
                diffs.push(InnerValue::Add(val.into_diff()));
            }
            Some(diffs)
        })
    }

    fn from_diff(diff: Self::Type) -> Self {
        let mut set = Self::new();
        if let Some(vals) = diff.0 {
            for val in vals {
                match val {
                    InnerValue::Add(val) => {
                        set.insert(<T>::from_diff(val));
                    }
                    InnerValue::Remove(val) => {
                        let val = <T>::from_diff(val);
                        set.remove(&val);
                    }
                }
            }
        }
        set
    }
}

impl<T> DiffHashSet<T>
where
    T: Clone + Debug + PartialEq + Diff + for<'de> Deserialize<'de> + Serialize,
{
    pub fn iter<'v>(&'v self) -> Box<dyn Iterator<Item = &InnerValue<T>> + 'v> {
        match &self.0 {
            Some(diffs) => Box::new(diffs.iter()),
            None => Box::new(empty()),
        }
    }

    pub fn into_iter<'v>(self) -> Box<dyn Iterator<Item = InnerValue<T>> + 'v>
    where
        Self: 'v,
    {
        match self.0 {
            Some(diffs) => Box::new(diffs.into_iter()),
            None => Box::new(empty()),
        }
    }

    pub fn len(&self) -> usize {
        match &self.0 {
            Some(diffs) => diffs.len(),
            None => 0,
        }
    }
}

impl<T> Debug for DiffHashSet<T>
where
    T: Debug + Diff,
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
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
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match &self {
            Self::Add(val) => f.debug_tuple("Add").field(val).finish(),
            Self::Remove(val) => f.debug_tuple("Remove").field(val).finish(),
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

        let diff = s.diff(&s1);
        let expected = DiffHashSet(None);

        assert_eq!(diff, expected);
        let s2 = s.merge(diff);
        assert_eq!(s, s2);
        assert_eq!(s1, s2);
    }
}
