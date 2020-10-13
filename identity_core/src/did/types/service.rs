use derive_builder::Builder;
use identity_diff::Diff;
use serde::{Deserialize, Serialize};

use crate::{common::Url, did::DID, utils::HasId};

/// Describes a `Service` in a `DIDDocument` type.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize, Diff, Builder)]
#[diff(from_into)]
#[builder(pattern = "owned")]
pub struct Service {
    #[serde(default)]
    #[builder(try_setter)]
    id: DID,
    #[serde(rename = "type")]
    #[builder(setter(into))]
    service_type: String,
    #[serde(rename = "serviceEndpoint")]
    #[builder(setter(into))]
    endpoint: ServiceEndpoint,
}

impl Service {
    pub fn id(&self) -> &DID {
        &self.id
    }

    pub fn service_type(&self) -> &str {
        &*self.service_type
    }

    pub fn endpoint(&self) -> &ServiceEndpoint {
        &self.endpoint
    }
}

impl HasId for Service {
    type Id = DID;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

/// Describes the `ServiceEndpoint` struct type.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Diff)]
#[serde(untagged)]
#[diff(from_into)]
pub enum ServiceEndpoint {
    Url(Url),
    Obj(ServiceEndpointObject),
}

impl ServiceEndpoint {
    pub fn context(&self) -> &Url {
        match self {
            Self::Url(url) => &url,
            Self::Obj(inner) => inner.context(),
        }
    }

    pub fn endpoint_type(&self) -> Option<&str> {
        match self {
            Self::Url(_) => None,
            Self::Obj(inner) => inner.endpoint_type(),
        }
    }

    pub fn instances(&self) -> Option<&[String]> {
        match self {
            Self::Url(_) => None,
            Self::Obj(inner) => inner.instances(),
        }
    }
}

impl Default for ServiceEndpoint {
    fn default() -> Self {
        Self::Url(Default::default())
    }
}

impl From<Url> for ServiceEndpoint {
    fn from(other: Url) -> Self {
        Self::Url(other)
    }
}

impl From<ServiceEndpointObject> for ServiceEndpoint {
    fn from(other: ServiceEndpointObject) -> Self {
        Self::Obj(other)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize, Diff, Builder)]
#[diff(from_into)]
#[builder(pattern = "owned", name = "ServiceEndpointBuilder")]
pub struct ServiceEndpointObject {
    #[serde(rename = "@context")]
    #[builder(try_setter)]
    context: Url,
    #[serde(rename = "type")]
    #[builder(default, setter(into, strip_option))]
    endpoint_type: Option<String>,
    #[builder(default, setter(into, strip_option))]
    instances: Option<Vec<String>>,
}

impl ServiceEndpointObject {
    pub fn context(&self) -> &Url {
        &self.context
    }

    pub fn endpoint_type(&self) -> Option<&str> {
        self.endpoint_type.as_deref()
    }

    pub fn instances(&self) -> Option<&[String]> {
        self.instances.as_deref()
    }
}
