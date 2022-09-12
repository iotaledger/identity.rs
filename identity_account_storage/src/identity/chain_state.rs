// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota_core_legacy::tangle::MessageId;
use identity_iota_core_legacy::tangle::MessageIdExt;
use serde::Deserialize;
use serde::Serialize;

/// Holds the last published message ids of the integration and diff chains.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ChainState {
  #[serde(default = "MessageId::null", skip_serializing_if = "MessageId::is_null")]
  last_integration_message_id: MessageId,
  #[deprecated(since = "0.5.0", note = "diff chain features are slated for removal")]
  #[serde(default = "MessageId::null", skip_serializing_if = "MessageId::is_null")]
  last_diff_message_id: MessageId,
}

impl ChainState {
  pub fn new() -> Self {
    Self {
      last_integration_message_id: MessageId::null(),
      last_diff_message_id: MessageId::null(),
    }
  }

  /// Returns the integration message id of the last published update.
  ///
  /// Note: [`MessageId`] has a built-in `null` variant that needs to be checked for.
  pub fn last_integration_message_id(&self) -> &MessageId {
    &self.last_integration_message_id
  }

  /// Returns the diff message id of the last published update.
  ///
  /// Note: [`MessageId`] has a built-in `null` variant that needs to be checked for.
  #[deprecated(since = "0.5.0", note = "diff chain features are slated for removal")]
  pub fn last_diff_message_id(&self) -> &MessageId {
    &self.last_diff_message_id
  }

  /// Sets the last integration message id and resets the
  /// last diff message id to [`MessageId::null()`].
  pub fn set_last_integration_message_id(&mut self, message: MessageId) {
    self.last_integration_message_id = message;

    // Clear the diff message id
    self.last_diff_message_id = MessageId::null();
  }

  /// Sets the last diff message id.
  #[deprecated(since = "0.5.0", note = "diff chain features are slated for removal")]
  pub fn set_last_diff_message_id(&mut self, message: MessageId) {
    self.last_diff_message_id = message;
  }

  /// Returns whether the identity has been published before.
  pub fn is_new_identity(&self) -> bool {
    self.last_integration_message_id.is_null()
  }
}

impl Default for ChainState {
  fn default() -> Self {
    Self::new()
  }
}
