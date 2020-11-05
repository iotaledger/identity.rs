use serde::{Deserialize, Serialize};

use crate::resolver::{DocumentMetadata, ResolutionMetadata, Resource};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Dereference {
    #[serde(rename = "did-url-dereferencing-metadata")]
    pub metadata: ResolutionMetadata,
    #[serde(rename = "content-stream", skip_serializing_if = "Option::is_none")]
    pub content: Option<Resource>,
    #[serde(rename = "content-metadata", skip_serializing_if = "Option::is_none")]
    pub content_metadata: Option<DocumentMetadata>,
}

impl Dereference {
    pub fn new() -> Self {
        Self {
            metadata: ResolutionMetadata::new(),
            content: None,
            content_metadata: None,
        }
    }
}
