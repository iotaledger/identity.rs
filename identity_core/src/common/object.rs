use identity_diff::Diff;
use serde::{Deserialize, Serialize};
use serde_json::Map;
use std::{
    collections::HashMap,
    fmt,
    iter::FromIterator,
    ops::{Deref, DerefMut},
};

use crate::{
    common::{OneOrMany, Value},
    error::{Error, Result},
};

type Inner = HashMap<String, Value>;

// An String -> Value `HashMap` wrapper
#[derive(Clone, Default, PartialEq, Deserialize, Serialize, Diff)]
#[repr(transparent)]
#[serde(transparent)]
pub struct Object(Inner);

impl Object {
    pub fn into_inner(self) -> Inner {
        self.0
    }

    pub fn take_object_id(&mut self) -> Option<String> {
        match self.0.remove("id") {
            Some(Value::String(id)) => Some(id),
            Some(_) | None => None,
        }
    }

    pub fn try_take_object_id(&mut self) -> Result<String> {
        self.take_object_id().ok_or(Error::InvalidObjectId)
    }

    pub fn take_object_type(&mut self) -> Option<String> {
        match self.0.remove("type") {
            Some(Value::String(value)) => Some(value),
            Some(_) | None => None,
        }
    }

    pub fn try_take_object_type(&mut self) -> Result<String> {
        self.take_object_type().ok_or(Error::InvalidObjectType)
    }

    pub fn take_object_types(&mut self) -> Option<OneOrMany<String>> {
        match self.remove("type") {
            Some(Value::String(value)) => Some(value.into()),
            Some(Value::Array(values)) => Some(Self::collect_types(values)),
            Some(_) | None => None,
        }
    }

    pub fn try_take_object_types(&mut self) -> Result<OneOrMany<String>> {
        self.take_object_types().ok_or(Error::InvalidObjectType)
    }

    fn collect_types(values: Vec<Value>) -> OneOrMany<String> {
        let mut types: Vec<String> = Vec::with_capacity(values.len());

        for value in values {
            if let Value::String(value) = value {
                types.push(value);
            }
        }

        types.into()
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
