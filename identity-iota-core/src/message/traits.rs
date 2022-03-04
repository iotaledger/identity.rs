// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::bee_message::MessageId;
use iota_client::bee_message::MESSAGE_ID_LENGTH;

use crate::did::IotaDID;
use crate::error::Result;

// TODO: Use MessageId when it has a const ctor
static NULL: &[u8; MESSAGE_ID_LENGTH] = &[0; MESSAGE_ID_LENGTH];

pub trait MessageIdExt: Sized {
  fn is_null(&self) -> bool;

  fn encode_hex(&self) -> String;

  fn decode_hex(hex: &str) -> Result<Self>;
}

impl MessageIdExt for MessageId {
  fn is_null(&self) -> bool {
    self.as_ref() == NULL
  }

  fn encode_hex(&self) -> String {
    self.to_string()
  }

  fn decode_hex(hex: &str) -> Result<Self> {
    hex.parse().map_err(Into::into)
  }
}

pub trait TangleRef {
  fn did(&self) -> &IotaDID;

  fn message_id(&self) -> &MessageId;

  fn set_message_id(&mut self, message_id: MessageId);

  fn previous_message_id(&self) -> &MessageId;

  fn set_previous_message_id(&mut self, message_id: MessageId);
}
