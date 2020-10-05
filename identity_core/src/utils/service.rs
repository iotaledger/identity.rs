use serde::{Deserialize, Serialize};

use std::{hash::Hash, str::FromStr};

use crate::utils::{Context, HasId, Subject};
use identity_diff::Diff;

/// Describes a `Service` in a `DIDDocument` type. Contains an `id`, `service_type` and `endpoint`.  The `endpoint` can
/// be represented as a `String` or a `ServiceEndpoint` in json.
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize, Diff, Clone, Default, Hash, PartialOrd, Ord)]
#[diff(from_into)]
pub struct Service {
    #[serde(default)]
    pub id: Subject,
    #[serde(rename = "type")]
    pub service_type: String,
    #[serde(rename = "serviceEndpoint")]
    pub endpoint: ServiceEndpoint,
}

/// Describes the `ServiceEndpoint` struct type. Contains a required `context` and two optional fields: `endpoint_type`
/// and `instances`.  If neither `instances` nor `endpoint_type` is specified, the `ServiceEndpoint` is represented as a
/// String in json using the `context`.
#[derive(Debug, Eq, PartialEq, Clone, Diff, Default, Hash, PartialOrd, Ord)]
#[diff(from_into)]
pub struct ServiceEndpoint {
    pub context: Context,
    pub endpoint_type: Option<String>,
    pub instances: Option<Vec<String>>,
}

impl Service {
    pub fn init(self) -> Self {
        Self {
            id: self.id,
            service_type: self.service_type,
            endpoint: self.endpoint,
        }
    }
}

impl ServiceEndpoint {
    pub fn init(self) -> Self {
        Self {
            context: self.context,
            endpoint_type: self.endpoint_type,
            instances: self.instances,
        }
    }
}

impl HasId for Service {
    type Id = Subject;

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
