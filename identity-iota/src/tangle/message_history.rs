// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::chain::DiffChain;
use crate::chain::IntegrationChain;
use crate::did::DocumentDiff;
use crate::did::IotaDID;
use crate::did::IotaDocument;
use crate::error::Result;
use crate::tangle::Client;
use crate::tangle::Message;
use crate::tangle::MessageId;
use crate::tangle::MessageSet;

/// A representation of a DID Document message history.
#[derive(Clone, Debug, Serialize)]
pub struct MessageHistory {
  #[serde(rename = "intChainData")]
  pub int_chain_data: IntegrationChain,
  #[serde(rename = "intChainSpam")]
  pub int_chain_spam: Option<Vec<MessageId>>,
  #[serde(rename = "diffChainData")]
  pub diff_chain_data: DiffChain,
  #[serde(rename = "diffChainSpam")]
  pub diff_chain_spam: Option<Vec<MessageId>>,
}

impl MessageHistory {
  /// Read the message history of the DID Document identified by the given DID.
  pub async fn read(client: &Client, did: &IotaDID) -> Result<Self> {
    let int_messages: Vec<Message> = client.read_messages(did.tag()).await?;
    let int_message_set: MessageSet<IotaDocument> = MessageSet::new(did, &int_messages);
    let int_chain_data: IntegrationChain = IntegrationChain::try_from_index(int_message_set.to_index())?;
    let int_message_id: &MessageId = int_chain_data.current_message_id();

    let diff_address: String = IotaDocument::diff_address(int_message_id)?;
    let diff_messages: Vec<Message> = client.read_messages(&diff_address).await?;
    let diff_message_set: MessageSet<DocumentDiff> = MessageSet::new(did, &diff_messages);
    let diff_chain_data: DiffChain = DiffChain::try_from_index(&int_chain_data, diff_message_set.to_index())?;

    Ok(MessageHistory {
      int_chain_data,
      int_chain_spam: int_message_set.spam().map(<[_]>::to_vec),
      diff_chain_data,
      diff_chain_spam: diff_message_set.spam().map(<[_]>::to_vec),
    })
  }
}
