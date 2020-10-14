use serde::{Deserialize, Serialize};

use crate::{
    did::DIDDocument as Document,
    resolver::{DocumentMetadata, ResolutionMetadata},
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Resolution {
    #[serde(rename = "did-resolution-metadata")]
    pub metadata: ResolutionMetadata,
    #[serde(rename = "did-document", skip_serializing_if = "Option::is_none")]
    pub document: Option<Document>,
    #[serde(rename = "did-document-metadata", skip_serializing_if = "Option::is_none")]
    pub document_metadata: Option<DocumentMetadata>,
}

impl Resolution {
    pub fn new() -> Self {
        Self {
            metadata: ResolutionMetadata::new(),
            document: None,
            document_metadata: None,
        }
    }
}
