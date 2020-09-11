use serde::{Deserialize, Serialize};

use crate::Diff;
use std::fmt::{Debug, Formatter, Result as FmtResult};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct DiffString(#[serde(skip_serializing_if = "Option::is_none")] pub Option<String>);

impl Diff for String {
    type Type = DiffString;

    fn diff(&self, other: &Self) -> Self::Type {
        if self == other {
            DiffString(None)
        } else {
            other.clone().into_diff()
        }
    }

    fn merge(&self, diff: Self::Type) -> Self {
        if diff.0.is_none() {
            self.to_string()
        } else {
            Self::from_diff(diff)
        }
    }

    fn from_diff(diff: Self::Type) -> Self {
        match diff.0 {
            Some(s) => s,
            None => panic!("DiffString error"),
        }
    }

    fn into_diff(self) -> Self::Type {
        DiffString(Some(self))
    }
}

impl Debug for DiffString {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match &self.0 {
            Some(val) => write!(f, "DiffString {:#?}", val),
            None => write!(f, "DiffString None"),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_diff() {
        let sa = String::from("test");
        let sb = String::from("another_string");

        let diff = sa.diff(&sb);

        assert_eq!(diff, DiffString(Some("another_string".into())));

        let sc = sa.merge(diff);

        assert_eq!(sb, sc);
    }

    #[test]
    fn test_same_string() {
        let sa = String::from("test");
        let sb = String::from("test");

        let diff = sa.diff(&sb);

        assert_eq!(diff, DiffString(None));

        let sc = sa.merge(diff);

        assert_eq!(sb, sc);
        assert_eq!(sa, sc);
    }
}
