use derive_builder::Builder;
use identity_diff::Diff;
use serde::{
    de::{self, Deserializer, MapAccess, Visitor},
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};
use std::{
    fmt::{self, Formatter},
    str::FromStr,
};

use crate::{common::Url, did::DID, utils::HasId};

/// Describes a `Service` in a `DIDDocument` type. Contains an `id`, `service_type` and `endpoint`.  The `endpoint` can
/// be represented as a `String` or a `ServiceEndpoint` in json.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize, Diff, Builder)]
#[diff(from_into)]
#[builder(pattern = "owned")]
pub struct Service {
    #[serde(default)]
    #[builder(try_setter)]
    pub id: DID,
    #[serde(rename = "type")]
    #[builder(setter(into))]
    pub service_type: String,
    #[serde(rename = "serviceEndpoint")]
    pub endpoint: ServiceEndpoint,
}

/// Describes the `ServiceEndpoint` struct type. Contains a required `context` and two optional fields: `endpoint_type`
/// and `instances`.  If neither `instances` nor `endpoint_type` is specified, the `ServiceEndpoint` is represented as a
/// String in json using the `context`.
#[derive(Clone, Debug, Default, PartialEq, Diff, Builder)]
#[diff(from_into)]
#[builder(pattern = "owned")]
pub struct ServiceEndpoint {
    #[builder(try_setter)]
    pub context: Url,
    #[builder(default, setter(into, strip_option))]
    pub endpoint_type: Option<String>,
    #[builder(default, setter(into, strip_option))]
    pub instances: Option<Vec<String>>,
}

impl HasId for Service {
    type Id = DID;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl FromStr for Service {
    type Err = crate::Error;

    fn from_str(s: &str) -> crate::Result<Service> {
        serde_json::from_str(s).map_err(crate::Error::DecodeJSON)
    }
}

impl ToString for Service {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Unable to serialize the service")
    }
}

impl FromStr for ServiceEndpoint {
    type Err = crate::Error;

    fn from_str(s: &str) -> crate::Result<ServiceEndpoint> {
        serde_json::from_str(s).map_err(crate::Error::DecodeJSON)
    }
}

impl ToString for ServiceEndpoint {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Unable to serialize the Service Endpoint struct")
    }
}

impl From<&str> for ServiceEndpoint {
    fn from(s: &str) -> Self {
        serde_json::from_str(s).expect("Unable to parse string")
    }
}

/// The Json fields for the `ServiceEndpoint`.
enum Field {
    Context,
    Type,
    Instances,
}

/// A visitor for the service endpoint values.
struct ServiceEndpointVisitor;

/// A visitor for the service endpoint keys.
struct FieldVisitor;

/// Deserialize logic for the `ServiceEndpoint` type.
impl<'de> Deserialize<'de> for ServiceEndpoint {
    fn deserialize<D>(deserializer: D) -> Result<ServiceEndpoint, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ServiceEndpointVisitor)
    }
}

/// Deserialize logic for the `Field` type.
impl<'de> Deserialize<'de> for Field {
    fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(FieldVisitor)
    }
}

/// Visitor logic for the `ServiceEndpointVisitor` to deserialize the `ServiceEndpoint`.
impl<'de> Visitor<'de> for ServiceEndpointVisitor {
    type Value = ServiceEndpoint;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("Expecting a string or a Service Endpoint Struct")
    }

    /// If given a &str use this logic to create a `ServiceEndpoint`.
    fn visit_str<E>(self, value: &str) -> Result<ServiceEndpoint, E>
    where
        E: de::Error,
    {
        Ok(ServiceEndpoint {
            context: Url::parse(value).map_err(de::Error::custom)?,
            ..Default::default()
        })
    }

    /// given a map, use this logic to create a `ServiceEndpoint`.
    fn visit_map<M>(self, mut map: M) -> Result<ServiceEndpoint, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut context: Option<String> = None;
        let mut endpoint_type: Option<String> = None;
        let mut instances: Option<Vec<String>> = None;

        while let Some(key) = map.next_key()? {
            match key {
                Field::Context => {
                    if context.is_some() {
                        return Err(de::Error::duplicate_field("@context"));
                    }
                    context = Some(map.next_value()?);
                }
                Field::Type => {
                    if endpoint_type.is_some() {
                        return Err(de::Error::duplicate_field("type"));
                    }
                    endpoint_type = Some(map.next_value()?);
                }
                Field::Instances => {
                    if instances.is_some() {
                        return Err(de::Error::duplicate_field("instances"));
                    }
                    instances = Some(map.next_value()?);
                }
            }
        }

        let context = context.ok_or_else(|| de::Error::missing_field("@context"))?;

        Ok(ServiceEndpoint {
            context: Url::parse(context).map_err(de::Error::custom)?,
            endpoint_type,
            instances,
        })
    }
}

/// Visitor logic for the `FieldVisitor` to deserialize the `Field` type.
impl<'de> Visitor<'de> for FieldVisitor {
    type Value = Field;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("Expected `@context`, `type`, or `instances`")
    }

    /// If given a &str use this logic to create a `Field`.
    fn visit_str<E>(self, value: &str) -> Result<Field, E>
    where
        E: de::Error,
    {
        match value {
            "@context" => Ok(Field::Context),
            "type" => Ok(Field::Type),
            "instances" => Ok(Field::Instances),
            _ => Err(de::Error::unknown_field(value, &[])),
        }
    }
}

/// Serialize the `ServiceEndpoint`.
impl Serialize for ServiceEndpoint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.instances == None && self.endpoint_type == None {
            self.context.serialize(serializer)
        } else {
            let mut se = serializer.serialize_struct("", 3)?;
            se.serialize_field("@context", &self.context)?;
            se.serialize_field("type", &self.endpoint_type)?;
            se.serialize_field("instances", &self.instances)?;
            se.end()
        }
    }
}
