use serde::{Deserialize, Serialize};
use serde_json::Map;
use std::{
    collections::HashMap,
    fmt,
    iter::FromIterator,
    ops::{Deref, DerefMut},
};

use crate::common::Value;

type Inner = HashMap<String, Value>;

// An String -> Value `HashMap` wrapper
#[derive(Clone, Default, PartialEq, Deserialize, Serialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct Object(Inner);

impl Object {
    pub fn into_inner(self) -> Inner {
        self.0
    }
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl Deref for Object {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Object {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Inner> for Object {
    fn from(other: Inner) -> Self {
        Self(other)
    }
}

impl From<Object> for Inner {
    fn from(other: Object) -> Self {
        other.into_inner()
    }
}

impl From<Map<String, Value>> for Object {
    fn from(other: Map<String, Value>) -> Self {
        Self::from_iter(other.into_iter())
    }
}

impl From<Object> for Map<String, Value> {
    fn from(other: Object) -> Self {
        Self::from_iter(other.into_inner().into_iter())
    }
}

impl From<Object> for Value {
    fn from(other: Object) -> Self {
        Value::Object(other.into())
    }
}

impl FromIterator<(String, Value)> for Object {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (String, Value)>,
    {
        Self(Inner::from_iter(iter))
    }
}
