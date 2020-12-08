use crate::did::{DocumentDiff, IotaDocument};

pub trait TangleRef {
    fn message_id(&self) -> Option<&str>;
    fn previous_message_id(&self) -> &str;
}

impl TangleRef for IotaDocument {
    fn message_id(&self) -> Option<&str> {
        IotaDocument::message_id(self)
    }

    fn previous_message_id(&self) -> &str {
        IotaDocument::previous_message_id(self).unwrap_or_default()
    }
}

impl TangleRef for DocumentDiff {
    fn message_id(&self) -> Option<&str> {
        DocumentDiff::message_id(self)
    }

    fn previous_message_id(&self) -> &str {
        DocumentDiff::previous_message_id(self)
    }
}
