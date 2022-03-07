// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub use network::Network;
pub use network::NetworkName;

mod network;

use iota_client::bee_message::MessageId;
use iota_client::bee_message::MESSAGE_ID_LENGTH;

static NULL: &[u8; MESSAGE_ID_LENGTH] = &[0; MESSAGE_ID_LENGTH];

pub fn message_id_is_null(message_id: &MessageId) -> bool {
  message_id.as_ref() == NULL
}
