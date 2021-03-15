// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::mem::replace;
use identity_core::common::Timestamp;
use identity_iota::tangle::MessageId;

use crate::types::Metadata;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IdentityMetadata {
  // Equivalent to `DID::tag`
  pub(crate) id: String,
  pub(crate) index: u32,
  pub(crate) ident: String,
  pub(crate) created_at: Timestamp,
  pub(crate) updated_at: Timestamp,
  pub(crate) latest_auth_message_id: MessageId,
  pub(crate) latest_diff_message_id: MessageId,
  pub(crate) auth_chain_message_id: Vec<MessageId>,
  pub(crate) diff_chain_message_id: Vec<MessageId>,
}

impl IdentityMetadata {
  pub fn new(id: String, index: u32, ident: String) -> Self {
    Self {
      id,
      index,
      ident,
      created_at: Timestamp::now(),
      updated_at: Timestamp::now(),
      latest_auth_message_id: MessageId::NONE,
      latest_diff_message_id: MessageId::NONE,
      auth_chain_message_id: Vec::new(),
      diff_chain_message_id: Vec::new(),
    }
  }

  pub fn id(&self) -> &str {
    &self.id
  }

  pub fn index(&self) -> u32 {
    self.index
  }

  pub fn ident(&self) -> &str {
    &self.ident
  }

  pub fn created_at(&self) -> Timestamp {
    self.created_at
  }

  pub fn updated_at(&self) -> Timestamp {
    self.updated_at
  }

  pub fn latest_auth_message_id(&self) -> &MessageId {
    &self.latest_auth_message_id
  }

  pub fn latest_diff_message_id(&self) -> &MessageId {
    &self.latest_diff_message_id
  }

  pub fn auth_chain_message_id(&self) -> &[MessageId] {
    &self.auth_chain_message_id
  }

  pub fn diff_chain_message_id(&self) -> &[MessageId] {
    &self.diff_chain_message_id
  }

  pub(crate) fn set_auth_message_id(&mut self, message_id: MessageId) {
    let previous: MessageId = replace(&mut self.latest_auth_message_id, message_id);

    if previous == MessageId::NONE {
      self.auth_chain_message_id.push(previous);
    }
  }

  pub(crate) fn set_diff_message_id(&mut self, message_id: MessageId) {
    let previous: MessageId = replace(&mut self.latest_diff_message_id, message_id);

    if previous == MessageId::NONE {
      self.diff_chain_message_id.push(previous);
    }
  }
}

impl Metadata for IdentityMetadata {
  fn tag(&self) -> &str {
    &self.id
  }

  fn ident(&self) -> &str {
    &self.ident
  }

  fn index(&self) -> u32 {
    self.index
  }
}
