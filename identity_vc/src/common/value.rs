use std::fmt;

use crate::common::{Object, URI};

#[derive(Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Value {
  Null,
  Boolean(bool),
  Number(Number),
  String(String),
  Array(Vec<Self>),
  Object(Object),
}

impl fmt::Debug for Value {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::Null => write!(f, "Null"),
      Self::Boolean(inner) => fmt::Debug::fmt(inner, f),
      Self::Number(inner) => fmt::Debug::fmt(inner, f),
      Self::String(inner) => fmt::Debug::fmt(inner, f),
      Self::Array(inner) => fmt::Debug::fmt(inner, f),
      Self::Object(inner) => fmt::Debug::fmt(inner, f),
    }
  }
}

impl From<()> for Value {
  fn from(_: ()) -> Self {
    Self::Null
  }
}

impl From<bool> for Value {
  fn from(other: bool) -> Self {
    Self::Boolean(other)
  }
}

impl From<u64> for Value {
  fn from(other: u64) -> Self {
    Self::Number(Number::UInt(other))
  }
}

impl From<i64> for Value {
  fn from(other: i64) -> Self {
    Self::Number(Number::SInt(other))
  }
}

impl From<f64> for Value {
  fn from(other: f64) -> Self {
    Self::Number(Number::Float(other))
  }
}

impl From<&'_ str> for Value {
  fn from(other: &'_ str) -> Self {
    Self::String(other.into())
  }
}

impl From<String> for Value {
  fn from(other: String) -> Self {
    Self::String(other)
  }
}

impl From<URI> for Value {
  fn from(other: URI) -> Self {
    Self::String(other.0)
  }
}

impl<T> From<Vec<T>> for Value
where
  T: Into<Value>,
{
  fn from(other: Vec<T>) -> Self {
    Self::Array(other.into_iter().map(Into::into).collect())
  }
}

impl From<Object> for Value {
  fn from(other: Object) -> Self {
    Self::Object(other)
  }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Number {
  UInt(u64),
  SInt(i64),
  Float(f64),
}

impl fmt::Debug for Number {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::UInt(inner) => fmt::Debug::fmt(inner, f),
      Self::SInt(inner) => fmt::Debug::fmt(inner, f),
      Self::Float(inner) => fmt::Debug::fmt(inner, f),
    }
  }
}
