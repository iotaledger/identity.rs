use identity_core::common::{Object, Value};
use serde::{de::Error as _, ser::SerializeMap, Deserialize, Deserializer, Serialize, Serializer};
use std::convert::TryFrom;

use crate::{
    error::{Error, Result},
    signature::{SignatureValue, PROPERTY_JWS, PROPERTY_PROOF, PROPERTY_SIGNATURE},
};

/// Contains the value and misc. properties of a linked data signature
#[derive(Clone, Debug, PartialEq)]
pub struct SignatureData {
    pub value: SignatureValue,
    pub properties: Object,
}

impl SignatureData {
    pub fn key(&self) -> &'static str {
        self.value.key()
    }

    pub fn value(&self) -> &str {
        self.value.value()
    }
}

impl TryFrom<Object> for SignatureData {
    type Error = Error;

    fn try_from(mut properties: Object) -> Result<Self, Self::Error> {
        let values = (
            properties.remove(PROPERTY_JWS),
            properties.remove(PROPERTY_PROOF),
            properties.remove(PROPERTY_SIGNATURE),
        );

        let value: SignatureValue = match values {
            (Some(Value::String(jws)), None, None) => SignatureValue::Jws(jws),
            (Some(_), None, None) => return Err(Error::InvalidLDSignature("Invaid Proof Type".into())),
            (None, Some(Value::String(proof)), None) => SignatureValue::Proof(proof),
            (None, Some(_), None) => return Err(Error::InvalidLDSignature("Invaid Proof Type".into())),
            (None, None, Some(Value::String(signature))) => SignatureValue::Signature(signature),
            (None, None, Some(_)) => return Err(Error::InvalidLDSignature("Invaid Proof Type".into())),
            (None, None, None) => return Err(Error::InvalidLDSignature("Missing Proof Value".into())),
            (_, _, _) => return Err(Error::InvalidLDSignature("Multiple Proof Values".into())),
        };

        Ok(Self { value, properties })
    }
}

impl From<SignatureValue> for SignatureData {
    fn from(other: SignatureValue) -> Self {
        Self {
            value: other,
            properties: Object::new(),
        }
    }
}

impl From<SignatureData> for Object {
    fn from(other: SignatureData) -> Self {
        let mut properties = other.properties.clone();
        properties.insert(other.key().into(), other.value().into());
        properties
    }
}

impl<'de> Deserialize<'de> for SignatureData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Object::deserialize(deserializer).and_then(|object| Self::try_from(object).map_err(D::Error::custom))
    }
}

impl Serialize for SignatureData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(1 + self.properties.len()))?;

        for (key, value) in self.properties.iter() {
            map.serialize_entry(key, value)?;
        }

        let value: &str = self.value();

        if !value.is_empty() {
            map.serialize_entry(self.key(), value)?;
        }

        map.end()
    }
}
