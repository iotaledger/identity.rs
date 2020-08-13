use std::collections::HashMap;

use crate::utils::{context::Context, subject::Subject};

#[derive(Debug, Eq, PartialEq)]
pub struct Service {
    pub context: Context,
    pub id: Subject,
    pub service_type: String,
    pub endpoint: String,
    pub metadata: HashMap<String, String>,
}
