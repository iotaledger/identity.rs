use serde::{Deserialize, Serialize};

use std::fmt::{Debug, Formatter, Result as FmtResult};

use crate::Diff;

#[derive(Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged, into = "Option<T>", from = "Option<T>")]
pub enum DiffOption<T: Diff> {
    Some(<T as Diff>::Type),
    None,
}

impl<T> Diff for Option<T>
where
    T: Diff + Clone + Debug + PartialEq + Default + for<'de> Deserialize<'de> + Serialize,
{
    type Type = DiffOption<T>;

    fn diff(&self, other: &Self) -> Self::Type {
        match (self, other) {
            (Some(x), Some(y)) => Self::Type::Some(x.diff(&y)),
            (None, Some(y)) => Self::Type::Some(y.clone().into_diff()),
            _ => Self::Type::None,
        }
    }

    fn merge(&self, diff: Self::Type) -> Self {
        match (self, diff) {
            (None, DiffOption::None) => None,
            (Some(_), DiffOption::None) => self.clone(),
            (None, DiffOption::Some(ref d)) => Some(<T>::from_diff(d.clone())),
            (Some(t), DiffOption::Some(ref d)) => Some(t.merge(d.clone())),
        }
    }

    fn from_diff(diff: Self::Type) -> Self {
        match diff {
            Self::Type::None => None,
            Self::Type::Some(diff) => Some(<T>::from_diff(diff)),
        }
    }

    fn into_diff(self) -> Self::Type {
        match self {
            Self::None => DiffOption::None,
            Self::Some(t) => DiffOption::Some(t.into_diff()),
        }
    }
}

impl<T: Diff> std::fmt::Debug for DiffOption<T> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match &self {
            Self::Some(d) => write!(f, "DiffOption::Some({:#?})", d),
            Self::None => write!(f, "DiffOption::None"),
        }
    }
}

impl<T: Diff> Default for DiffOption<T> {
    fn default() -> Self {
        Self::None
    }
}

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
