use core::ops::Deref;
use identity_core::convert::FromJson as _;

use crate::{
    client::TangleMessage,
    did::{IotaDID, IotaDocument},
};

#[derive(Debug)]
pub struct TangleDocument {
    pub(crate) document: IotaDocument,
    pub(crate) message: TangleMessage,
}

impl TangleDocument {
    pub fn new(did: &IotaDID, message: TangleMessage) -> Option<Self> {
        // Convert the Tangle message content to a UTF8 string.
        let json: String = message.message_utf8().ok()?;

        // Deserialize the message; ignore any documents that fail.
        let document: IotaDocument = IotaDocument::from_json(&json).ok()?;

        // Ignore any documents that don't belong to that target DID.
        if did.authority() != document.id().authority() {
            return None;
        }

        Some(Self { document, message })
    }
}

impl Deref for TangleDocument {
    type Target = IotaDocument;

    fn deref(&self) -> &Self::Target {
        &self.document
    }
}
