use serde::{
    ser::{Serialize, SerializeStruct, Serializer},
    Deserialize, Serialize as DSerialize,
};

use std::str::FromStr;

use crate::utils::{
    helpers::{string_or_list, string_or_struct},
    Context, Subject,
};

#[derive(Debug, Eq, PartialEq, Deserialize, DSerialize, Clone)]
pub struct Service {
    #[serde(default)]
    pub id: Subject,
    #[serde(rename = "type")]
    pub service_type: String,
    #[serde(rename = "serviceEndpoint", deserialize_with = "string_or_struct")]
    pub endpoint: ServiceEndpoint,
}

#[derive(Debug, Eq, PartialEq, Deserialize, Clone)]
pub struct ServiceEndpoint {
    #[serde(rename = "@context", deserialize_with = "string_or_list", default)]
    context: Context,
    endpoint_type: Option<String>,
    instances: Option<Vec<String>>,
}

impl Service {
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
        Ok(ServiceEndpoint {
            context: Context::from_str(s)?,
            endpoint_type: None,
            instances: None,
        })
    }
}

impl ToString for ServiceEndpoint {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Unable to serialize the Service Endpoint struct")
    }
}

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_service_endpoint() {
        let endpoint = ServiceEndpoint {
            context: Context::from_str("https://schema.identity.foundation/hub").unwrap(),
            endpoint_type: Some("AgentService".into()),
            instances: Some(vec!["did:example:456".into(), "did:example:789".into()]),
        };

        let endpoint = ServiceEndpoint {
            context: Context::from_str("https://schema.identity.foundation/hub").unwrap(),
            endpoint_type: None,
            instances: None,
        };

        println!("{}", endpoint.to_string())
    }
}
