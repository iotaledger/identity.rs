// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde;
use serde::Deserialize;
use serde::Serialize;

use identity_core::common::Object;
use identity_core::common::Timestamp;

use crate::tangle::MessageId;
use crate::tangle::MessageIdExt;

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
      created: Timestamp::now_utc(),
      updated: Timestamp::now_utc(),
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
