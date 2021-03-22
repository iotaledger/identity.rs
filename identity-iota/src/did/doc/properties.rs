// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Object;
use identity_core::common::Timestamp;

use crate::tangle::MessageId;

/// Additional properties stored in an IOTA DID Document.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Properties {
  pub(crate) created: Timestamp,
  pub(crate) updated: Timestamp,
  #[serde(default, skip_serializing_if = "MessageId::is_none")]
  pub(crate) previous_message_id: MessageId,
  #[serde(flatten)]
  pub(crate) properties: Object,
}

impl Properties {
  pub fn new() -> Self {
    Self {
      created: Timestamp::now(),
      updated: Timestamp::now(),
      previous_message_id: MessageId::NONE,
      properties: Object::new(),
    }
  }
}

impl Default for Properties {
  fn default() -> Self {
    Self::new()
  }
}
