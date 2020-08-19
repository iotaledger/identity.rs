use serde::{Deserialize, Serialize};

use std::str::FromStr;

use crate::utils::{Context, Subject};

#[derive(Debug, Eq, PartialEq, Deserialize, Serialize, Clone)]
pub struct Service {
    #[serde(default)]
    pub id: Subject,
    #[serde(rename = "type")]
    pub service_type: String,
    #[serde(rename = "serviceEndpoint")]
    pub endpoint: ServiceEndpoint,
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct ServiceEndpoint {
    pub context: Context,
    pub endpoint_type: Option<String>,
    pub instances: Option<Vec<String>>,
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
        Ok(serde_json::from_str(s)?)
    }
}

impl ToString for ServiceEndpoint {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Unable to serialize the Service Endpoint struct")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_service_endpoint() {
        let se1 = ServiceEndpoint::new("some_endpoint".into(), None, None).unwrap();
        let se2 = ServiceEndpoint::new(
            "some_endpoint".into(),
            Some("test".into()),
            Some(vec!["test".into(), "test".into()]),
        )
        .unwrap();

        println!("{:?}", se1);
        println!("{}", se1.to_string());
        println!("{:?}", se2);
        println!("{}", se2.to_string());

        let rstr = r#"{
        "@context": "https://schema.identity.foundation/hub",
        "type": "UserHubEndpoint",
        "instances": ["did:example:456", "did:example:789"]
      }"#;

        println!("{:?}", ServiceEndpoint::from_str(rstr).unwrap());
    }
}
