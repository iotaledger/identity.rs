use core::mem;

use crate::{
    did::{IotaDID, IotaDocument},
    error::{Error, Result},
    tangle::{Message, MessageId, MessageIndex, TangleRef as _},
};

#[derive(Debug)]
pub struct AuthChain {
    pub(crate) current: IotaDocument,
    pub(crate) history: Option<Vec<IotaDocument>>,
}

impl AuthChain {
    /// Constructs a new `AuthChain` from a slice of `Message`s.
    pub fn try_from_messages(did: &IotaDID, messages: &[Message]) -> Result<Self> {
        let mut index: MessageIndex<IotaDocument> = messages
            .iter()
            .flat_map(|message| message.try_extract_document(did))
            .collect();

        let current: IotaDocument =
            index
                .remove_where(&MessageId::NONE, |doc| doc.verify().is_ok())
                .ok_or(Error::ChainError {
                    error: "Invalid Root Document",
                })?;

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

    /// Creates a new `AuthChain` with the given `IotaDocument` as the latest.
    pub fn new(current: IotaDocument) -> Result<Self> {
        if current.verify().is_err() {
            return Err(Error::ChainError {
                error: "Invalid Signature",
            });
        }

        if current.message_id().is_none() {
            return Err(Error::ChainError {
                error: "Invalid Message Id",
            });
        }

        Ok(Self { current, history: None })
    }

    /// Returns a reference to the latest document in the auth chain.
    pub fn current(&self) -> &IotaDocument {
        &self.current
    }

    /// Returns a mutable reference to the latest document in the auth chain.
    pub fn current_mut(&mut self) -> &mut IotaDocument {
        &mut self.current
    }

    /// Returns the Tangle message Id of the latest auth document.
    pub fn current_message_id(&self) -> &MessageId {
        self.current.message_id()
    }

    /// Adds a new document to the auth chain.
    ///
    /// # Errors
    ///
    /// Fails if the document signature is invalid or the Tangle message
    /// references within the document are invalid.
    pub fn try_push(&mut self, document: IotaDocument) -> Result<()> {
        self.check_validity(&document)?;

        self.history
            .get_or_insert_with(Vec::new)
            .push(mem::replace(&mut self.current, document));

        Ok(())
    }

    /// Returns `true` if the `IotaDocument` can be added to the auth chain.
    pub fn is_valid(&self, document: &IotaDocument) -> bool {
        self.check_validity(document).is_ok()
    }

    /// Checks if the `IotaDocument` can be added to the auth chain.
    ///
    /// # Errors
    ///
    /// Fails if the `IotaDocument` is not a valid addition.
    pub fn check_validity(&self, document: &IotaDocument) -> Result<()> {
        if self.current.verify_data(document).is_err() {
            return Err(Error::ChainError {
                error: "Invalid Signature",
            });
        }

        if document.message_id().is_none() {
            return Err(Error::ChainError {
                error: "Invalid Message Id",
            });
        }

        if document.previous_message_id().is_none() {
            return Err(Error::ChainError {
                error: "Invalid Previous Message Id",
            });
        }

        if self.current_message_id() != document.previous_message_id() {
            return Err(Error::ChainError {
                error: "Invalid Previous Message Id",
            });
        }

        Ok(())
    }
}
