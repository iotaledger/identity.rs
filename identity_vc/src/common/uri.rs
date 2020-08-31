use std::{fmt, ops::Deref};

/// A simple wrapper for URIs adhering to RFC 3986
///
/// TODO: Parse/Validate according to RFC 3986
/// TODO: impl From<DID> for URI
#[derive(Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct URI(pub(crate) String);

impl URI {
  pub fn into_inner(self) -> String {
    self.0
  }
}

impl fmt::Debug for URI {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "URI({:?})", self.0)
  }
}

impl Deref for URI {
  type Target = String;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl From<&'_ str> for URI {
  fn from(other: &'_ str) -> Self {
    Self(other.into())
  }
}

impl From<String> for URI {
  fn from(other: String) -> Self {
    Self(other)
  }
}

impl PartialEq<str> for URI {
  fn eq(&self, other: &str) -> bool {
    self.0.eq(other)
  }
}
