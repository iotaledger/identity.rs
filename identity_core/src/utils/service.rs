use serde::{Deserialize, Serialize};

use std::str::FromStr;

use crate::utils::{Context, Subject};
use serde_diff::SerdeDiff;

/// Describes a `Service` in a `DIDDocument` type. Contains an `id`, `service_type` and `endpoint`.  The `endpoint` can
/// be represented as a `String` or a `ServiceEndpoint` in json.
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize, SerdeDiff, Clone, Default)]
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
#[derive(Debug, Eq, PartialEq, Clone, SerdeDiff, Default)]
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

impl FromStr for Service {
    type Err = crate::Error;

    fn from_str(s: &str) -> crate::Result<Service> {
        Ok(serde_json::from_str(s)?)
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
        Ok(serde_json::from_str(s)?)
    }
}

impl ToString for ServiceEndpoint {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Unable to serialize the Service Endpoint struct")
    }
}
