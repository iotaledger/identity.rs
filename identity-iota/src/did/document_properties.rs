use identity_core::common::{Object, Timestamp};

use crate::tangle::MessageId;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Properties {
    pub(crate) created: Timestamp,
    pub(crate) updated: Timestamp,
    pub(crate) immutable: bool,
    #[serde(default, skip_serializing_if = "MessageId::is_none")]
    pub(crate) previous_message_id: MessageId,
    #[serde(flatten)]
    pub(crate) properties: Object,
}

impl Properties {
    pub fn new() -> Self {
        Self {
            created: Timestamp::now(),
            updated: Timestamp::now(),
            immutable: false,
            previous_message_id: MessageId::NONE,
            properties: Object::new(),
        }
    }
}

impl Default for Properties {
    fn default() -> Self {
        Self::new()
    }
}
