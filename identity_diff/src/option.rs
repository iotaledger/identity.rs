use serde::{Deserialize, Serialize};

use std::fmt::{Debug, Formatter, Result as FmtResult};

use crate::Diff;

/// A `DiffOption<T>` type which represents a Diffed `Option<T>`.  By default this value is untagged for `serde`. It
/// also converts `to` and `from` Option<T> when serialized/deserialized
#[derive(Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged, into = "Option<T>", from = "Option<T>")]
pub enum DiffOption<T: Diff> {
    Some(<T as Diff>::Type),
    None,
}

/// `Diff` Implementation for `Option<T>`
impl<T> Diff for Option<T>
where
    T: Diff + Clone + Debug + PartialEq + Default + for<'de> Deserialize<'de> + Serialize,
{
    /// The Corresponding Diff type for `Option<T>`
    type Type = DiffOption<T>;

    /// Compares two `Option<T>` types; `self` and `other` and finds the Difference between them, returning a
    /// `DiffOption<T>` type.
    fn diff(&self, other: &Self) -> Self::Type {
        match (self, other) {
            (Some(x), Some(y)) => Self::Type::Some(x.diff(&y)),
            (None, Some(y)) => Self::Type::Some(y.clone().into_diff()),
            _ => Self::Type::None,
        }
    }

    /// Merges a `DiffOption<T>`; `diff` type with an `Option<T>` type; `self`.
    fn merge(&self, diff: Self::Type) -> Self {
        match (self, diff) {
            (None, DiffOption::None) => None,
            (Some(_), DiffOption::None) => self.clone(),
            (None, DiffOption::Some(ref d)) => Some(<T>::from_diff(d.clone())),
            (Some(t), DiffOption::Some(ref d)) => Some(t.merge(d.clone())),
        }
    }

    /// converts a `DiffOption<T>`; `diff` to an `Option<T>` type.
    fn from_diff(diff: Self::Type) -> Self {
        match diff {
            Self::Type::None => None,
            Self::Type::Some(diff) => Some(<T>::from_diff(diff)),
        }
    }

    /// converts a `Option<T>`; `self` to an `DiffOption<T>` type.
    fn into_diff(self) -> Self::Type {
        match self {
            Self::None => DiffOption::None,
            Self::Some(t) => DiffOption::Some(t.into_diff()),
        }
    }
}

/// Debug implementation for `DiffOption<T>`.
impl<T: Diff> std::fmt::Debug for DiffOption<T> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match &self {
            Self::Some(d) => write!(f, "DiffOption::Some({:#?})", d),
            Self::None => write!(f, "DiffOption::None"),
        }
    }
}

/// Default implementation for `DiffOption<T>`.
impl<T: Diff> Default for DiffOption<T> {
    fn default() -> Self {
        Self::None
    }
}

/// Into `Option<T>` implementation for `DiffOption<T>`.
impl<T> Into<Option<T>> for DiffOption<T>
where
    T: Diff,
{
    fn into(self) -> Option<T> {
        match self {
            DiffOption::Some(s) => Some(Diff::from_diff(s)),
            DiffOption::None => None,
        }
    }
}
/// From `Option<T>` implementation for `DiffOption<T>`.
impl<T> From<Option<T>> for DiffOption<T>
where
    T: Diff,
{
    fn from(opt: Option<T>) -> Self {
        match opt {
            Some(s) => DiffOption::Some(s.into_diff()),
            None => DiffOption::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::string::DiffString;

    #[test]
    fn test_option_diff() {
        let a = Some("A".to_owned());
        let b = Some("B".to_owned());

        let diff = a.diff(&b);

        assert_eq!(diff, DiffOption::Some(DiffString(Some("B".to_owned()))));

        let c = a.merge(diff);

        assert_eq!(b, c);
    }
}
