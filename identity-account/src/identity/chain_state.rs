// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Serialize;

use identity_iota::tangle::MessageId;
use identity_iota::tangle::MessageIdExt;

/// The state of the integration and diff chain.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChainState {
  #[serde(default = "MessageId::null", skip_serializing_if = "MessageId::is_null")]
  this_message_id: MessageId,
  #[serde(default = "MessageId::null", skip_serializing_if = "MessageId::is_null")]
  last_integration_message_id: MessageId,
  #[serde(default = "MessageId::null", skip_serializing_if = "MessageId::is_null")]
  last_diff_message_id: MessageId,
}

impl ChainState {
  pub fn new() -> Self {
    Self {
      this_message_id: MessageId::null(),
      last_integration_message_id: MessageId::null(),
      last_diff_message_id: MessageId::null(),
    }
  }

  /// Returns the current integration Tangle message id of the identity.
  pub fn this_message_id(&self) -> &MessageId {
    &self.this_message_id
  }

  /// Returns the previous integration Tangle message id of the identity.
  pub fn last_message_id(&self) -> &MessageId {
    &self.last_integration_message_id
  }

  /// Returns the previous diff Tangle message id, or the current integration message id.
  pub fn diff_message_id(&self) -> &MessageId {
    if self.last_diff_message_id.is_null() {
      &self.this_message_id
    } else {
      &self.last_diff_message_id
    }
  }

  /// Sets the current Tangle integration message id of the identity.
  pub fn set_integration_message_id(&mut self, message: MessageId) {
    // Set the current integration message id as the previous integration message.
    self.last_integration_message_id = self.this_message_id;

    // Clear the diff message id
    self.last_diff_message_id = MessageId::null();

    // Set the new integration message id
    self.this_message_id = message;
  }

  /// Sets the current Tangle diff message id of the identity.
  pub fn set_diff_message_id(&mut self, message: MessageId) {
    self.last_diff_message_id = message;
  }

  /// Returns whether the identity has been published before.
  pub fn is_new_identity(&self) -> bool {
    self.this_message_id() == &MessageId::null()
  }
}

impl Default for ChainState {
  fn default() -> Self {
    Self::new()
  }
}
