use identity_core::common::{Object, Timestamp};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Properties {
    pub(crate) created: Timestamp,
    pub(crate) updated: Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) previous_message_id: Option<String>,
    #[serde(flatten)]
    pub(crate) properties: Object,
}

impl Properties {
    pub fn new() -> Self {
        Self {
            created: Timestamp::now(),
            updated: Timestamp::now(),
            previous_message_id: None,
            properties: Object::new(),
        }
    }
}

impl Default for Properties {
    fn default() -> Self {
        Self::new()
    }
}
