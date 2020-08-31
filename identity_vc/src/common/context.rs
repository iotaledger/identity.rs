use identity_core::common::Object;
use std::fmt;

use crate::common::URI;

/// A reference to a JSON-LD context
#[derive(Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Context {
  URI(URI),
  OBJ(Object),
}

impl fmt::Debug for Context {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::URI(inner) => fmt::Debug::fmt(inner, f),
      Self::OBJ(inner) => fmt::Debug::fmt(inner, f),
    }
  }
}

impl<T> From<T> for Context
where
  T: Into<URI>,
{
  fn from(other: T) -> Self {
    Self::URI(other.into())
  }
}

impl From<Object> for Context {
  fn from(other: Object) -> Self {
    Self::OBJ(other)
  }
}
