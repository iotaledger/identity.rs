use core::time::Duration;
use did_url::DID;
use serde::{Deserialize, Serialize};

use crate::{common::Object, resolver::ErrorKind};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ResolutionMetadata {
    /// The error code from the resolution process, if an error occurred.
    ///
    /// [More Info](https://www.w3.org/TR/did-spec-registries/#error)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorKind>,
    /// The MIME type of the returned document stream.
    ///
    /// Note: This is only relevant when using stream-based resolution.
    ///
    /// [More Info](https://www.w3.org/TR/did-spec-registries/#content-type)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// The elapsed time of the resolution operation.
    pub duration: Duration,
    /// The parsed DID that was used for resolution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<DID>,
    /// Additional resolution metadata properties.
    #[serde(flatten)]
    pub properties: Object,
}

impl ResolutionMetadata {
    pub fn new() -> Self {
        Self {
            error: None,
            content_type: None,
            duration: Duration::from_secs(0),
            resolved: None,
            properties: Object::new(),
        }
    }
}
