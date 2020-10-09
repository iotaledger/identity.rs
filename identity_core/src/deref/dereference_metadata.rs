use core::time::Duration;
use serde::{Deserialize, Serialize};

use crate::resolver::ErrorKind;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DereferenceMetadata {
    pub duration: Duration,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorKind>,
}

impl DereferenceMetadata {
    pub const fn new() -> Self {
        Self {
            duration: Duration::from_secs(0),
            error: None,
        }
    }
}
