// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use uuid::Uuid;

use crate::message::Timing;

/// A DIDComm `report` Message
///
/// [Reference](https://github.com/iotaledger/identity.rs/blob/dev/docs/DID%20Communications%20Research%20and%20Specification/Standalone_Messages.md#report)
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Report {
  context: String,
  thread: Uuid,
  reference: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  error: Option<u32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  comment: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  timing: Option<Timing>,
}

impl Report {
  /// Creates a new `Report` message.
  pub fn new(context: String, thread: Uuid, reference: String) -> Self {
    Self {
      context,
      thread,
      reference,
      error: None,
      comment: None,
      timing: None,
    }
  }

  impl_message_accessor!(context => String);
  impl_message_accessor!(thread => Uuid);
  impl_message_accessor!(reference => String);
  impl_message_accessor!(error => Option<u32>);
  impl_message_accessor!(comment => Option<String>);
  impl_message_accessor!(timing => Option<Timing>);
}
