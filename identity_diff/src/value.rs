use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::Diff;

#[derive(Clone, Eq, Serialize, Deserialize, PartialEq, Debug)]
pub struct DiffValue(Option<Value>);

impl Default for DiffValue {
    fn default() -> Self {
        DiffValue(None)
    }
}

impl Diff for Value {
    type Type = DiffValue;

    fn diff(&self, other: &Self) -> Self::Type {
        if self == other {
            DiffValue(None)
        } else {
            other.clone().into_diff()
        }
    }

    fn merge(&self, diff: Self::Type) -> Self {
        if diff.0.is_none() {
            self.clone()
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
        DiffValue(Some(self))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_json_value() {
        let v = Value::Null;

        let v2 = Value::Bool(true);

        let diff = v.diff(&v2);

        let res = v.merge(diff);

        let expected = Value::Bool(true);

        assert_eq!(expected, res);

        let v = json!("A string");

        let v2 = json!("A string");

        let diff = v.diff(&v2);

        let res = v.merge(diff);

        assert_eq!(res, v2);

        let v3 = json!("Another string");

        let diff = v.diff(&v3);

        let res = v.merge(diff);

        assert_eq!(v3, res);
    }
}
