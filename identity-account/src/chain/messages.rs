// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::tangle::MessageId;

use crate::types::Index;
use crate::types::IndexMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessageSet {
  this: MessageId,
  last: Option<MessageId>,
  diff: IndexMap<MessageId>,
}

// =============================================================================
// =============================================================================

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChainMessages(IndexMap<MessageSet>);

impl ChainMessages {
  pub fn new() -> Self {
    Self(IndexMap::new())
  }

  pub fn this_message_id(&self, auth: Index) -> Option<&MessageId> {
    self.0.get(auth).map(|messages| &messages.this)
  }

  pub fn last_message_id(&self, auth: Index) -> Option<&MessageId> {
    self.0.get(auth).and_then(|messages| messages.last.as_ref())
  }

  pub fn diff_message_id(&self, auth: Index, diff: Index) -> Option<&MessageId> {
    self.0.get(auth).and_then(|messages| messages.diff.get(diff))
  }

  pub fn set_auth_message_id(&mut self, auth: Index, message: MessageId) -> bool {
    if self.0.exists(auth) {
      return false;
    }

    let previous: Option<MessageId> = auth
      .try_decrement()
      .ok()
      .and_then(|index| self.this_message_id(index))
      .cloned();

    let messages: MessageSet = MessageSet {
      this: message,
      last: previous,
      diff: IndexMap::new(),
    };

    self.0.insert(auth, messages)
  }

  pub fn set_diff_message_id(&mut self, auth: Index, diff: Index, message: MessageId) -> bool {
    match self.0.get_mut(auth) {
      Some(messages) => messages.diff.insert(diff, message),
      None => false,
    }
  }
}
