use core::mem;

use crate::{
    client::{TangleIndex, TangleMessage},
    did::{IotaDID, IotaDocument},
    error::Result,
};

#[derive(Debug)]
pub struct AuthChain {
    pub(crate) current: IotaDocument,
    pub(crate) history: Option<Vec<IotaDocument>>,
}

impl AuthChain {
    pub fn try_from_messages(did: &IotaDID, messages: &[TangleMessage]) -> Result<Self> {
        let mut index: TangleIndex<IotaDocument> = messages
            .iter()
            .flat_map(|message| message.try_extract_document(did))
            .collect();

        let current: Option<IotaDocument> = index.remove_where("", |doc| doc.verify().is_ok());
        let current: IotaDocument = current.expect("Error: Invalid Root Document");

        let mut this: Self = Self::new(current)?;

        while let Some(mut list) = index.remove(this.current_message_id()) {
            'inner: while let Some(document) = list.pop() {
                if this.try_push(document).is_ok() {
                    break 'inner;
                }
            }
        }

        Ok(this)
    }

    pub fn new(current: IotaDocument) -> Result<Self> {
        if current.verify().is_err() {
            todo!("Error: Invalid Signature")
        }

        if current.message_id().is_none() {
            todo!("Error: Invalid Message Id")
        }

        if current.message_id().unwrap().is_empty() {
            todo!("Error: Invalid Message Id")
        }

        Ok(Self { current, history: None })
    }

    pub fn current(&self) -> &IotaDocument {
        &self.current
    }

    pub fn current_mut(&mut self) -> &mut IotaDocument {
        &mut self.current
    }

    pub fn current_message_id(&self) -> &str {
        self.current().message_id().expect("Auth Chain Corrupted")
    }

    pub fn try_push(&mut self, document: IotaDocument) -> Result<()> {
        if self.current.verify_data(&document).is_err() {
            todo!("Error: Invalid Signature")
        }

        if document.message_id().is_none() {
            todo!("Error: Invalid Message Id")
        }

        if document.message_id().unwrap().is_empty() {
            todo!("Error: Invalid Message Id")
        }

        if document.previous_message_id().is_none() {
            todo!("Error: Invalid Previus Message Id")
        }

        if self.current_message_id() != document.previous_message_id().unwrap() {
            todo!("Error: Invalid Previus Message Id")
        }

        self.history
            .get_or_insert_with(Vec::new)
            .push(mem::replace(&mut self.current, document));

        Ok(())
    }
}
