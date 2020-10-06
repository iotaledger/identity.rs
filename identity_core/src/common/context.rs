use core::fmt;
use identity_diff::Diff;
use serde::{Deserialize, Serialize};

use crate::common::{Object, Uri};

/// A reference to a JSON-LD context
///
/// [More Info](https://www.w3.org/TR/vc-data-model/#contexts)
#[derive(Clone, PartialEq, Deserialize, Serialize, Diff)]
#[serde(untagged)]
pub enum Context {
    Uri(Uri),
    Obj(Object),
}

impl fmt::Debug for Context {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Uri(inner) => fmt::Debug::fmt(inner, f),
            Self::Obj(inner) => fmt::Debug::fmt(inner, f),
        }
    }
}

impl From<Uri> for Context {
    fn from(other: Uri) -> Self {
        Self::Uri(other)
    }
}

impl From<&'_ str> for Context {
    fn from(other: &'_ str) -> Self {
        Self::Uri(other.into())
    }
}

impl From<String> for Context {
    fn from(other: String) -> Self {
        Self::Uri(other.into())
    }
}

impl From<Object> for Context {
    fn from(other: Object) -> Self {
        Self::Obj(other)
    }
}
