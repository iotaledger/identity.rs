use serde::{Deserialize, Serialize};

use crate::{common::Object, did::DIDDocument, resolver::ResolutionMetadata};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Resolution {
    #[serde(rename = "didResolutionMetadata")]
    pub metadata: ResolutionMetadata,
    #[serde(rename = "didDocument")]
    pub did_document: Option<DIDDocument>,
    #[serde(rename = "didResolutionMetadata")]
    pub did_document_metadata: Option<Object>,
}

impl Resolution {
    pub const fn new() -> Self {
        Self {
            metadata: ResolutionMetadata::new(),
            did_document: None,
            did_document_metadata: None,
        }
    }
}
