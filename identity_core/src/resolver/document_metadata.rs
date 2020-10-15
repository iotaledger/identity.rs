use crate::common::{Object, Timestamp};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DocumentMetadata {
    /// The timestamp of the Create operation.
    ///
    /// [More Info](https://www.w3.org/TR/did-spec-registries/#created)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<Timestamp>,
    /// The timestamp of the last Update operation.
    ///
    /// [More Info](https://www.w3.org/TR/did-spec-registries/#updated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<Timestamp>,
    /// Additional document metadata properties.
    #[serde(flatten)]
    pub properties: Object,
}

impl DocumentMetadata {
    pub fn new() -> Self {
        Self {
            created: None,
            updated: None,
            properties: Object::new(),
        }
    }
}
