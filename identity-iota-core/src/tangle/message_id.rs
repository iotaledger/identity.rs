// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// Re-export types from bee-message to avoid adding it or iota-client as a dependency for
// downstream crates.
#[doc(inline)]
pub use bee_message::Message;
#[doc(inline)]
pub use bee_message::MessageId;

use crate::Error;
use bee_message::MESSAGE_ID_LENGTH;

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
    hex.parse().map_err(Error::InvalidMessage)
  }
}
