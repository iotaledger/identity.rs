use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::utils::{helpers::string_or_list, Context, Subject};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
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
