use core::slice::Iter;

use crate::{
    chain::AuthChain,
    client::{TangleIndex, TangleMessage},
    did::{DocumentDiff, IotaDID},
    error::Result,
};

#[derive(Debug)]
pub struct DiffChain {
    inner: Vec<DocumentDiff>,
}

impl DiffChain {
    pub fn try_from_messages(auth: &AuthChain, messages: &[TangleMessage]) -> Result<Self> {
        if messages.is_empty() {
            return Ok(Self::new());
        }

        let did: &IotaDID = auth.current().id();

        let mut index: TangleIndex<DocumentDiff> = messages
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

    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn iter(&self) -> Iter<'_, DocumentDiff> {
        self.inner.iter()
    }

    pub fn current_message_id(&self) -> Option<&str> {
        self.inner.last().and_then(|diff| diff.message_id())
    }
}

impl Default for DiffChain {
    fn default() -> Self {
        Self::new()
    }
}

fn message_id<'a>(auth: &'a AuthChain, diff: &'a DiffChain) -> &'a str {
    diff.current_message_id().unwrap_or_else(|| auth.current_message_id())
}
