use core::fmt::{Debug, Formatter, Result};
use serde::{Deserialize, Serialize};

use crate::common::{Object, Url};

/// A reference to a JSON-LD context
///
/// [More Info](https://www.w3.org/TR/vc-data-model/#contexts)
#[derive(Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Context {
    Url(Url),
    Obj(Object),
}

impl Debug for Context {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Url(inner) => Debug::fmt(inner, f),
            Self::Obj(inner) => Debug::fmt(inner, f),
        }
    }
}

impl From<Url> for Context {
    fn from(other: Url) -> Self {
        Self::Url(other)
    }
}

impl From<Object> for Context {
    fn from(other: Object) -> Self {
        Self::Obj(other)
    }
}

impl<T> PartialEq<T> for Context
where
    T: AsRef<str> + ?Sized,
{
    fn eq(&self, other: &T) -> bool {
        match self {
            Self::Url(inner) => inner.as_str() == other.as_ref(),
            Self::Obj(_) => false,
        }
    }
}
