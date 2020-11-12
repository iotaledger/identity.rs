use serde::{Deserialize, Serialize};

use crate::common::Object;

pub const MIME_ANY: &str = "*/*";
pub const MIME_DID: &str = "application/did+json";
pub const MIME_DID_LD: &str = "application/did+ld+json";

// TODO: Support versioning via `version-id`/`version-time`
// TODO: Support caching via `no-cache`
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputMetadata {
    /// The MIME type of the preferred representation of the DID document.
    ///
    /// Note: This is only relevant when using stream-based resolution.
    ///
    /// [More Info](https://www.w3.org/TR/did-spec-registries/#accept)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept: Option<String>,
    /// Additional input metadata properties.
    #[serde(flatten)]
    pub properties: Object,
}

impl InputMetadata {
    pub fn new() -> Self {
        Self {
            accept: None,
            properties: Object::new(),
        }
    }
}
