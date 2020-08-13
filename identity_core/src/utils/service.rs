use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::utils::{Context, Subject};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Service {
    pub context: Context,
    pub id: Subject,
    pub service_type: String,
    pub endpoint: String,
    pub metadata: HashMap<String, String>,
}
