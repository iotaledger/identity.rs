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
    pub fn new(context: String, id: String, service_type: String, endpoint: String) -> crate::Result<Self> {
        Ok(Self {
            context: Context::new(vec![context]),
            id: Subject::from_str(&id)?,
            service_type,
            endpoint,
        })
    }
}
