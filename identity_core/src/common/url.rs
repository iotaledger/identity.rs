use core::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    ops::{Deref, DerefMut},
    str::FromStr,
};
use did_doc::url;

use crate::error::{Error, Result};

/// A parsed URL.
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct Url(url::Url);

impl Url {
    /// Parses an absolute `Url` from the given input string.
    pub fn parse(input: impl AsRef<str>) -> Result<Self> {
        url::Url::parse(input.as_ref()).map_err(Into::into).map(Self)
    }

    /// Consumes the `Url` and returns the value as a `String`.
    pub fn into_string(self) -> String {
        self.0.into_string()
    }

    /// Parses the given input string as a `Url`, with `self` as the base `Url`.
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

impl<T> PartialEq<T> for Url
where
    T: AsRef<str> + ?Sized,
{
    fn eq(&self, other: &T) -> bool {
        self.as_str() == other.as_ref()
    }
}
