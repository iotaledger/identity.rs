// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota_core::did::IotaDID;
use identity_iota_core::diff::DiffMessage;
use identity_iota_core::message::MessageId;

use crate::tangle::TangleRef;

impl TangleRef for DiffMessage {
  fn did(&self) -> &IotaDID {
    self.id()
  }

  fn message_id(&self) -> &MessageId {
    self.message_id()
  }

  fn set_message_id(&mut self, message_id: MessageId) {
    self.set_message_id(message_id);
  }

  fn previous_message_id(&self) -> &MessageId {
    self.previous_message_id()
  }

  fn set_previous_message_id(&mut self, message_id: MessageId) {
    self.set_previous_message_id(message_id);
  }
}
