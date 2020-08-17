use serde::{Deserialize, Serialize};

use std::str::FromStr;

use crate::utils::{helpers::string_or_list, Context, Subject};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct Service {
    #[serde(
        rename = "@context",
        skip_serializing_if = "Context::is_empty",
        deserialize_with = "string_or_list",
        default
    )]
    pub context: Context,
    #[serde(default)]
    pub id: Subject,
    #[serde(rename = "type")]
    pub service_type: String,
    #[serde(rename = "serviceEndpoint")]
    pub endpoint: String,
}

impl Service {
    pub fn new(context: Vec<String>, id: String, service_type: String, endpoint: String) -> crate::Result<Self> {
        Ok(Self {
            context: Context::new(context),
            id: Subject::from_str(&id)?,
            service_type,
            endpoint,
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
        serde_json::to_string(self).expect("Unable to serialize Public Key")
    }
}
