// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub use self::client::*;

#[cfg(not(any(test, feature = "mem-client")))]
mod client {
  pub type Client = identity_iota::client::Client;
}

#[cfg(any(test, feature = "mem-client"))]
mod client {
  use crypto::utils::rand;
  use hashbrown::HashMap;
  use identity_core::convert::ToJson;
  use identity_iota::client::Network;
  use identity_iota::did::Document;
  use identity_iota::did::DocumentDiff;
  use identity_iota::tangle::MessageId;

  use crate::error::Result;
  use crate::utils::Shared;

  #[derive(Debug)]
  pub struct Client {
    network: Network,
    auth_messages: Shared<HashMap<MessageId, Vec<u8>>>,
    diff_messages: Shared<HashMap<MessageId, Vec<u8>>>,
  }

  impl Client {
    pub async fn from_network(network: Network) -> Result<Self> {
      Ok(Self {
        network,
        auth_messages: Shared::new(HashMap::new()),
        diff_messages: Shared::new(HashMap::new()),
      })
    }

    pub async fn publish_document(&self, data: &Document) -> Result<MessageId> {
      let key: MessageId = Self::random_id()?;
      let val: Vec<u8> = data.to_json_vec()?;

      self.auth_messages.write()?.insert(key, val);

      Ok(key)
    }

    pub async fn publish_diff(&self, _previous: &MessageId, data: &DocumentDiff) -> Result<MessageId> {
      let key: MessageId = Self::random_id()?;
      let val: Vec<u8> = data.to_json_vec()?;

      self.diff_messages.write()?.insert(key, val);

      Ok(key)
    }

    fn random_id() -> Result<MessageId> {
      let mut data: [u8; 32] = [0; 32];

      rand::fill(&mut data)?;

      Ok(MessageId::new(data))
    }
  }
}
