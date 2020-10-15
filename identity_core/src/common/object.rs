use core::{
    fmt,
    iter::FromIterator,
    ops::{Deref, DerefMut},
};
use identity_diff::{
    self as diff,
    hashmap::{DiffHashMap, InnerValue},
    Diff,
};
use serde::{Deserialize, Serialize};
use serde_json::Map;

use crate::{
    common::{OneOrMany, Value},
    error::{Error, Result},
};

type Inner = Map<String, Value>;

// An String -> Value `HashMap` wrapper
#[derive(Clone, Default, PartialEq, Deserialize, Serialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct Object(Inner);

impl Object {
    pub fn new() -> Self {
        Self(Map::new())
    }

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

impl From<Object> for Inner {
    fn from(other: Object) -> Self {
        other.into_inner()
    }
}

impl<I, T> From<I> for Object
where
    I: IntoIterator<Item = (String, T)>,
    T: Into<Value>,
{
    fn from(other: I) -> Self {
        Self::from_iter(other)
    }
}

impl From<Object> for Value {
    fn from(other: Object) -> Self {
        Value::Object(other.into())
    }
}

impl<T> FromIterator<(String, T)> for Object
where
    T: Into<Value>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (String, T)>,
    {
        let inner: Inner = iter.into_iter().map(|(key, value)| (key, value.into())).collect();

        Self(inner)
    }
}

impl Diff for Object {
    type Type = DiffHashMap<String, Value>;

    fn diff(&self, other: &Self) -> diff::Result<Self::Type> {
        use std::collections::HashSet;

        let old: HashSet<&String> = self.keys().collect();
        let new: HashSet<&String> = other.keys().collect();
        let changed_keys = old.intersection(&new).filter(|key| self[**key] != other[**key]);

        let removed_keys = old.difference(&new);
        let added_keys = new.difference(&old);

        let mut changes: Vec<InnerValue<String, Value>> = Vec::new();

        for key in changed_keys {
            changes.push(InnerValue::Change {
                key: key.to_string(),
                value: self[*key].diff(&other[*key])?,
            });
        }

        for key in added_keys {
            changes.push(InnerValue::Add {
                key: key.to_string(),
                value: other[*key].clone().into_diff()?,
            });
        }

        for key in removed_keys {
            changes.push(InnerValue::Remove { key: key.to_string() });
        }

        if changes.is_empty() {
            Ok(DiffHashMap(None))
        } else {
            Ok(DiffHashMap(Some(changes)))
        }
    }

    fn merge(&self, diff: Self::Type) -> diff::Result<Self> {
        let mut this: Self = self.clone();

        for change in diff.0.into_iter().flatten() {
            match change {
                InnerValue::Change { key, value } => {
                    if let Some(entry) = this.get_mut(&key) {
                        *entry = Value::from_diff(value)?;
                    }
                }
                InnerValue::Add { key, value } => {
                    this.insert(key, Value::from_diff(value)?);
                }
                InnerValue::Remove { key } => {
                    this.remove(&key);
                }
            }
        }

        Ok(this)
    }

    fn from_diff(diff: Self::Type) -> diff::Result<Self> {
        let mut this: Self = Self::new();

        if let Some(diff) = diff.0 {
            for (index, inner) in diff.into_iter().enumerate() {
                if let InnerValue::Add { key, value } = inner {
                    this.insert(key, Value::from_diff(value)?);
                } else {
                    panic!("Unable to create Diff at index: {:?}", index);
                }
            }
        }

        Ok(this)
    }

    fn into_diff(self) -> diff::Result<Self::Type> {
        let changes: Vec<_> = self
            .0
            .into_iter()
            .map(|(key, value)| value.into_diff().map(|value| InnerValue::Add { key, value }))
            .collect::<diff::Result<_>>()?;

        if changes.is_empty() {
            Ok(DiffHashMap(None))
        } else {
            Ok(DiffHashMap(Some(changes)))
        }
    }
}
