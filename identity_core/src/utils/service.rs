use serde::{Deserialize, Serialize};

use std::str::FromStr;

use crate::utils::{Context, Subject};

/// Describes a `Service` in a `DIDDocument` type. Contains an `id`, `service_type` and `endpoint`.  The `endpoint` can
/// be represented as a `String` or a `ServiceEndpoint` in json.
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize, Clone)]
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
#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct ServiceEndpoint {
    pub context: Context,
    pub endpoint_type: Option<String>,
    pub instances: Option<Vec<String>>,
}

impl Service {
    /// Creates a new `Service` given a `id`, `service_type`, `endpoint`, `endpoint_type`, and `instances`.
    /// `endpoint_type`, and `instances` are optional.
    pub fn new(
        id: String,
        service_type: String,
        endpoint: String,
        endpoint_type: Option<String>,
        instances: Option<Vec<String>>,
    ) -> crate::Result<Self> {
        Ok(Self {
            id: Subject::from_str(&id)?,
            service_type,
            endpoint: ServiceEndpoint::new(endpoint, endpoint_type, instances)?,
        })
    }
}

impl ServiceEndpoint {
    /// Builds a new `ServiceEndpoint` given an `endpoint`, `endpoint_type`, and `instances`. `endpoint_type`, and
    /// `instances` are optional.
    pub fn new(endpoint: String, endpoint_type: Option<String>, instances: Option<Vec<String>>) -> crate::Result<Self> {
        Ok(ServiceEndpoint {
            context: Context::from_str(&endpoint)?,
            endpoint_type,
            instances,
        })
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
