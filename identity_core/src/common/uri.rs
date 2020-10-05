use serde::{Deserialize, Serialize};
use std::{fmt, ops::Deref};

use crate::did::DID;

/// A simple wrapper for URIs adhering to RFC 3986
///
/// TODO: Parse/Validate according to RFC 3986
#[derive(Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct Uri(String);

impl Uri {
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl fmt::Debug for Uri {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Uri({:?})", self.0)
    }
}

impl Deref for Uri {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&'_ str> for Uri {
    fn from(other: &'_ str) -> Self {
        Self(other.into())
    }
}

impl From<String> for Uri {
    fn from(other: String) -> Self {
        Self(other)
    }
}

impl From<DID> for Uri {
    fn from(other: DID) -> Uri {
        Self(other.to_string())
    }
}

impl<T> PartialEq<T> for Uri
where
    T: AsRef<str> + ?Sized,
{
    fn eq(&self, other: &T) -> bool {
        self.0.eq(other.as_ref())
    }
}
