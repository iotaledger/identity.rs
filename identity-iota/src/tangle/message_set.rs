// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;
// use identity_common::core::FromJson;

use crate::did::DocumentDiff;
use crate::did::IotaDID;
use crate::did::IotaVerificationMethod;
use crate::did::Verifier;
use crate::tangle::Message;
use crate::tangle::MessageId;
use crate::tangle::MessageIndex;
use crate::tangle::TangleRef;
use crate::tangle::TryFromMessage;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessageSet<T> {
  data: BTreeMap<MessageId, T>,
  spam: Option<Vec<MessageId>>,
}

impl<T> MessageSet<T> {
  pub fn get(&self, message_id: &MessageId) -> Option<&T> {
    self.data.get(message_id)
  }

  pub fn data(&self) -> &BTreeMap<MessageId, T> {
    &self.data
  }

  pub fn spam(&self) -> Option<&[MessageId]> {
    self.spam.as_deref()
  }

  pub fn message_ids(&self) -> impl Iterator<Item = &MessageId> {
    self.data.keys()
  }

  pub fn resources(&self) -> impl Iterator<Item = &T> {
    self.data.values()
  }
}

impl<T: TryFromMessage> MessageSet<T> {
  pub fn new(did: &IotaDID, messages: &[Message]) -> Self {
    let mut data: BTreeMap<MessageId, T> = BTreeMap::new();
    let mut spam: Option<Vec<MessageId>> = None;

    for message in messages {
      let message_id: MessageId = message.id().0;

      match T::try_from_message(message, did) {
        Some(resource) => {
          data.insert(message_id, resource);
        }
        None => {
          spam.get_or_insert_with(Default::default).push(message_id);
        }
      }
    }

    Self { data, spam }
  }
}

impl<T: Clone + TangleRef> MessageSet<T> {
  pub fn to_index(&self) -> MessageIndex<T> {
    self.resources().cloned().collect()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DiffSet {
  data: Vec<DocumentDiff>,
  spam: Option<Vec<MessageId>>,
}

impl DiffSet {
  pub fn new(did: &IotaDID, method: &IotaVerificationMethod, message_id: &MessageId, messages: &[Message]) -> Self {
    let message_set: MessageSet<DocumentDiff> = MessageSet::new(did, messages);

    let mut index: MessageIndex<DocumentDiff> = message_set.to_index();
    let mut target: MessageId = *message_id;

    let mut data: Vec<DocumentDiff> = Vec::new();
    let mut spam: Option<Vec<MessageId>> = None;

    while let Some(mut list) = index.remove(&target) {
      'inner: while let Some(next) = list.pop() {
        if Verifier::do_verify(method, &next).is_ok() {
          target = *next.message_id();
          data.push(next);
          break 'inner;
        } else {
          spam.get_or_insert_with(Vec::new).push(*next.message_id());
        }
      }
    }

    spam.get_or_insert_with(Vec::new).extend(index.drain_keys());

    Self { data, spam }
  }
}
