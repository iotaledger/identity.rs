use core::slice::Iter;

use crate::{
    chain::AuthChain,
    did::{DocumentDiff, IotaDID},
    error::Result,
    tangle::{Message, MessageId, MessageIndex, TangleRef as _},
};

#[derive(Debug)]
pub struct DiffChain {
    inner: Vec<DocumentDiff>,
}

impl DiffChain {
    /// Constructs a new `DiffChain` for the given `AuthChain` from a slice of `Message`s.
    pub fn try_from_messages(auth: &AuthChain, messages: &[Message]) -> Result<Self> {
        if messages.is_empty() {
            return Ok(Self::new());
        }

        let did: &IotaDID = auth.current().id();

        let mut index: MessageIndex<DocumentDiff> = messages
            .iter()
            .flat_map(|message| message.try_extract_diff(did))
            .collect();

        let mut this: Self = Self::new();

        while let Some(mut list) = index.remove(message_id(auth, &this)) {
            'inner: while let Some(next) = list.pop() {
                if auth.current().verify_data(&next).is_ok() {
                    this.inner.push(next);
                    break 'inner;
                }
            }
        }

        Ok(this)
    }

    /// Creates a new `DiffChain`.
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    /// Returns the total number of diffs in the chain.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns `true` if the diff chain is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns an iterator yielding references to `DocumentDiff`s.
    pub fn iter(&self) -> Iter<'_, DocumentDiff> {
        self.inner.iter()
    }

    /// Returns the `MessageId` of the latest diff if the chain, if any.
    pub fn current_message_id(&self) -> Option<&MessageId> {
        self.inner.last().map(|diff| diff.message_id())
    }
}

impl Default for DiffChain {
    fn default() -> Self {
        Self::new()
    }
}

fn message_id<'a>(auth: &'a AuthChain, diff: &'a DiffChain) -> &'a MessageId {
    diff.current_message_id().unwrap_or_else(|| auth.current_message_id())
}
