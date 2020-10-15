use core::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    ops::{Deref, DerefMut},
    str::FromStr,
};
use identity_diff::{self as diff, string::DiffString, Diff};
use serde::{Deserialize, Serialize};

use crate::{
    did::DID,
    error::{Error, Result},
};

/// A simple wrapper for URIs adhering to RFC 3986
///
/// TODO: Parse/Validate according to RFC 3986
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct Url(url::Url);

impl Url {
    pub fn parse(input: impl AsRef<str>) -> Result<Self> {
        url::Url::parse(input.as_ref()).map_err(Into::into).map(Self)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn into_string(self) -> String {
        self.0.into_string()
    }

    pub fn clone_into_string(&self) -> String {
        self.0.clone().into_string()
    }

    pub fn join(&self, input: impl AsRef<str>) -> Result<Self> {
        self.0.join(input.as_ref()).map_err(Into::into).map(Self)
    }
}

impl Debug for Url {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        Debug::fmt(&self.0, f)
    }
}

impl Display for Url {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl Default for Url {
    fn default() -> Self {
        Self::parse("did:").unwrap()
    }
}

impl Deref for Url {
    type Target = url::Url;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Url {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromStr for Url {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        Self::parse(string)
    }
}

impl From<&'_ str> for Url {
    fn from(other: &'_ str) -> Self {
        Self::parse(other).unwrap()
    }
}

impl From<String> for Url {
    fn from(other: String) -> Self {
        Self::parse(other).unwrap()
    }
}

impl From<DID> for Url {
    fn from(other: DID) -> Url {
        Self::parse(other.to_string()).unwrap()
    }
}

impl<T> PartialEq<T> for Url
where
    T: AsRef<str> + ?Sized,
{
    fn eq(&self, other: &T) -> bool {
        self.as_str() == other.as_ref()
    }
}

impl Diff for Url {
    type Type = DiffString;

    fn diff(&self, other: &Self) -> Result<Self::Type, diff::Error> {
        self.clone_into_string().diff(&other.clone_into_string())
    }

    fn merge(&self, diff: Self::Type) -> Result<Self, diff::Error> {
        Self::parse(self.clone_into_string().merge(diff)?)
            .map_err(|error| diff::Error::MergeError(format!("{}", error)))
    }

    fn from_diff(diff: Self::Type) -> Result<Self, diff::Error> {
        Self::parse(String::from_diff(diff)?).map_err(|error| diff::Error::MergeError(format!("{}", error)))
    }

    fn into_diff(self) -> Result<Self::Type, diff::Error> {
        self.clone_into_string().into_diff()
    }
}
