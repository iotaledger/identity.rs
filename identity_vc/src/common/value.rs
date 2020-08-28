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

impl<T> From<T> for Value where T: Into<Number> {
  fn from(other: T) -> Self {
    Self::Number(other.into())
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

macro_rules! impl_number_primitive {
  ($src:ty, $ident:ident) => {
    impl_number_primitive!($src as $src, $ident);
  };
  ($src:ty as $dst:ty, $ident:ident) => {
    impl From<$src> for Number {
      fn from(other: $src) -> Self {
        Self::$ident(other as $dst)
      }
    }
  };
  ($($src:ty),* as $dst:ty, $ident:ident) => {
    $(
      impl_number_primitive!($src as $dst, $ident);
    )*
  };
}

impl_number_primitive!(u8, u16, u32, u64 as u64, UInt);
impl_number_primitive!(i8, i16, i32, i64 as i64, SInt);
impl_number_primitive!(f32, f64 as f64, Float);
