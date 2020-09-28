use identity_common::{Object, Value};
use serde::{de::Error as _, ser::SerializeMap, Deserialize, Deserializer, Serialize, Serializer};
use std::convert::TryFrom;

use crate::error::{Error, Result};

const PROPERTY_JWS: &str = "jws";
const PROPERTY_PROOF: &str = "proofValue";
const PROPERTY_SIGNATURE: &str = "signatureValue";

/// Represents one of the various proof values of a linked data signature suite
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum SignatureValue {
    Jws(String),
    Proof(String),
    Signature(String),
}

impl SignatureValue {
    pub fn key(&self) -> &'static str {
        match self {
            Self::Jws(_) => PROPERTY_JWS,
            Self::Proof(_) => PROPERTY_PROOF,
            Self::Signature(_) => PROPERTY_SIGNATURE,
        }
    }

    pub fn value(&self) -> &str {
        match self {
            Self::Jws(ref inner) => inner,
            Self::Proof(ref inner) => inner,
            Self::Signature(ref inner) => inner,
        }
    }
}

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

        map.serialize_entry(self.key(), self.value())?;

        map.end()
    }
}
