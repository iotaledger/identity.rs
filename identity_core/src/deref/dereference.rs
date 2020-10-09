use serde::{Deserialize, Serialize};

use crate::{
    deref::{DereferenceMetadata, Resource},
    resolver::Resolution,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Dereference {
    pub content: Option<Resource>,     // content
    pub metadata: DereferenceMetadata, // did-url-dereferencing-metadata
    pub resolution: Resolution,        // content-metadata
}

impl Dereference {
    pub const fn new() -> Self {
        Self {
            content: None,
            metadata: DereferenceMetadata::new(),
            resolution: Resolution::new(),
        }
    }
}
