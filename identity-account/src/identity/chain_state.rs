// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Serialize;

use identity_iota::tangle::MessageId;
use identity_iota::tangle::MessageIdExt;

/// Holds the last published message ids of the integration and diff chains.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChainState {
  #[serde(default = "MessageId::null", skip_serializing_if = "MessageId::is_null")]
  previous_integration_message_id: MessageId,
  #[serde(default = "MessageId::null", skip_serializing_if = "MessageId::is_null")]
  previous_diff_message_id: MessageId,
}

impl ChainState {
  pub fn new() -> Self {
    Self {
      previous_integration_message_id: MessageId::null(),
      previous_diff_message_id: MessageId::null(),
    }
  }

  /// Returns the previous integration message id of the identity.
  pub fn previous_integration_message_id(&self) -> &MessageId {
    &self.previous_integration_message_id
  }

  /// Returns the previous diff message id, or the previous integration message id.
  pub fn diff_message_id(&self) -> &MessageId {
    if self.previous_diff_message_id.is_null() {
      &self.previous_integration_message_id
    } else {
      &self.previous_diff_message_id
    }
  }

  /// Sets the current integration message id of the identity.
  pub fn set_integration_message_id(&mut self, message: MessageId) {
    self.previous_integration_message_id = message;

    // Clear the diff message id
    self.previous_diff_message_id = MessageId::null();
  }

  /// Sets the current Tangle diff message id of the identity.
  pub fn set_diff_message_id(&mut self, message: MessageId) {
    self.previous_diff_message_id = message;
  }

  /// Returns whether the identity has been published before.
  pub fn is_new_identity(&self) -> bool {
    self.previous_integration_message_id == MessageId::null()
  }
}

impl Default for ChainState {
  fn default() -> Self {
    Self::new()
  }
}
