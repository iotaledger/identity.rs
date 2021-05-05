// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::tangle::MessageIdExt;
use identity_core::common::Object;
use identity_core::common::Timestamp;
use iota_client::bee_message::MessageId;

/// Additional properties stored in an IOTA DID Document.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Properties {
  pub(crate) created: Timestamp,
  pub(crate) updated: Timestamp,
  #[serde(
    rename = "previousMessageId",
    default = "MessageId::null",
    skip_serializing_if = "MessageIdExt::is_null"
  )]
  pub(crate) previous_message_id: MessageId,
  #[serde(flatten)]
  pub(crate) properties: Object,
}

impl Properties {
  pub fn new() -> Self {
    Self {
      created: Timestamp::now(),
      updated: Timestamp::now(),
      previous_message_id: MessageId::null(),
      properties: Object::new(),
    }
  }
}

impl Default for Properties {
  fn default() -> Self {
    Self::new()
  }
}
