// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::bee_message::MessageId;

pub trait TangleRef {
  fn message_id(&self) -> &MessageId;

  fn set_message_id(&mut self, message_id: MessageId);

  fn previous_message_id(&self) -> &MessageId;

  fn set_previous_message_id(&mut self, message_id: MessageId);
}
