use std::iter::FromIterator;
use serde_json::Map;

/// Re-export `Value` from `serde_json` as the catch-all value type.
///
/// It's not ONLY compatible with JSON (implements Deserialize/Serialize)
pub use serde_json::Value;

use crate::common::{Object, URI};

impl From<URI> for Value {
  fn from(other: URI) -> Self {
    Self::String(other.0)
  }
}

impl From<Object> for Map<String, Value> {
  fn from(other: Object) -> Self {
    Map::from_iter(other.into_inner().into_iter())
  }
}

impl From<Object> for Value {
  fn from(other: Object) -> Self {
    Value::Object(other.into())
  }
}
