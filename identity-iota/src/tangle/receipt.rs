// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Url;

use crate::tangle::Message;
use crate::tangle::MessageId;
use crate::tangle::Network;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Receipt {
  network: Network,
  #[serde(rename = "messageId")]
  message_id: MessageId,
  #[serde(rename = "networkId")]
  network_id: u64,
  nonce: u64,
}

impl Receipt {
  pub(crate) fn new(network: Network, message: Message) -> Self {
    Self {
      network,
      message_id: message.id().0,
      network_id: message.network_id(),
      nonce: message.nonce(),
    }
  }

  /// Returns the associated IOTA Tangle `Network`.
  pub fn network(&self) -> Network {
    self.network
  }

  /// Returns the message `id`.
  pub fn message_id(&self) -> &MessageId {
    &self.message_id
  }

  /// Returns the message `network_id`.
  pub fn network_id(&self) -> u64 {
    self.network_id
  }

  /// Returns the message `nonce`.
  pub fn nonce(&self) -> u64 {
    self.nonce
  }

  /// Returns the web explorer URL of the associated `message`.
  pub fn message_url(&self) -> Url {
    self.network.message_url(&self.message_id.to_string())
  }
}

impl From<Receipt> for MessageId {
  fn from(other: Receipt) -> MessageId {
    other.message_id
  }
}
