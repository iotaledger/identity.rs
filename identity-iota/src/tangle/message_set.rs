// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;
use std::ops::Deref;

use crate::did::DocumentDiff;
use crate::did::IotaDID;
use crate::did::IotaVerificationMethod;
use crate::did::Verifier;
use crate::tangle::Message;
use crate::tangle::MessageId;
use crate::tangle::MessageIndex;
use crate::tangle::TangleRef;
use crate::tangle::TryFromMessage;

/// Set of messages for a particular DID.
///
/// Retains a list of "spam" messages not matching the given message type or DID.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessageSet<T> {
  /// Valid messages.
  data: BTreeMap<MessageId, T>,
  /// Invalid messages that do not match the given DID or type.
  spam: Vec<MessageId>,
}

impl<T> MessageSet<T> {
  pub fn get(&self, message_id: &MessageId) -> Option<&T> {
    self.data.get(message_id)
  }

  pub fn data(&self) -> &BTreeMap<MessageId, T> {
    &self.data
  }

  pub fn spam(&self) -> &[MessageId] {
    self.spam.deref()
  }

  pub fn message_ids(&self) -> impl Iterator<Item=&MessageId> {
    self.data.keys()
  }

  pub fn resources(&self) -> impl Iterator<Item=&T> {
    self.data.values()
  }
}

impl<T: TryFromMessage> MessageSet<T> {
  /// Construct a new [`MessageSet`] from a list of [`Messages`][Message].
  pub fn new(did: &IotaDID, messages: &[Message]) -> Self {
    let mut data: BTreeMap<MessageId, T> = BTreeMap::new();
    let mut spam: Vec<MessageId> = Vec::new();

    for message in messages {
      let message_id: MessageId = message.id().0;

      match T::try_from_message(message, did) {
        Some(resource) => {
          data.insert(message_id, resource);
        }
        None => {
          spam.push(message_id);
        }
      }
    }

    Self {
      data,
      spam,
    }
  }
}

impl<T: Clone + TangleRef> MessageSet<T> {
  pub fn to_index(&self) -> MessageIndex<T> {
    self.resources().cloned().collect()
  }
}

/// List of [`DocumentDiff`] messages forming a diff chain.
///
/// Retains a list of "spam" messages that are valid but do not form part of the resulting chain.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DiffSet {
  /// Diff chain
  data: Vec<DocumentDiff>,
  /// Messages that are valid [`DocumentDiffs`][DocumentDiff] but not part of the resulting chain
  spam: Vec<MessageId>,
}

impl DiffSet {
  /// Constructs a [`DiffSet`] from a list of [`Messages`][Message], starting from a particular
  /// [`MessageId`].
  pub fn new(did: &IotaDID, method: &IotaVerificationMethod, message_id: &MessageId, messages: &[Message]) -> Self {
    let message_set: MessageSet<DocumentDiff> = MessageSet::new(did, messages);

    let mut index: MessageIndex<DocumentDiff> = message_set.to_index();
    let mut target: MessageId = *message_id;

    let mut data: Vec<DocumentDiff> = Vec::new();
    let mut spam: Vec<MessageId> = Vec::new();

    while let Some(mut list) = index.remove(&target) {
      let mut found: bool = false;
      while let Some(next) = list.pop() {
        // TODO: ensure this matches document resolution behaviour when there are multiple diff
        //       documents with the same previous_message_id.
        if !found && Verifier::do_verify(method, &next).is_ok() {
          target = *next.message_id();
          data.push(next);
          found = true;
          // Do not break early, otherwise we may miss some spam message that are valid documents
          // but not part of the resulting diff chain.
        } else {
          spam.push(*next.message_id());
        }
      }
    }

    spam.extend(index.drain_keys());

    Self {
      data,
      spam,
    }
  }
}
