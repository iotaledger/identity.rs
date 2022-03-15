// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;
use std::ops::Deref;

use identity_iota_core::did::IotaDID;
use identity_iota_core::diff::DiffMessage;
use identity_iota_core::document::IotaDocument;
use identity_iota_core::tangle::Message;
use identity_iota_core::tangle::MessageId;
use serde;
use serde::Deserialize;
use serde::Serialize;

use crate::chain::DiffChain;
use crate::chain::IntegrationChain;
use crate::document::ResolvedIotaDocument;
use crate::error::Result;
use crate::tangle::Client;
use crate::tangle::MessageExt;
use crate::tangle::MessageIndex;
use crate::tangle::TangleRef;

/// A DID Document's history and current state.
// ChainHistory<T> is not stored directly due to limitations on exporting generics in Wasm bindings.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DocumentHistory {
  /// List of integration chain documents.
  #[serde(rename = "integrationChainData")]
  pub integration_chain_data: Vec<ResolvedIotaDocument>,
  /// List of "spam" messages on the integration chain index.
  #[serde(rename = "integrationChainSpam")]
  pub integration_chain_spam: Vec<MessageId>,
  /// List of diffs for the last integration chain document.
  #[serde(rename = "diffChainData")]
  pub diff_chain_data: Vec<DiffMessage>,
  /// List of "spam" messages on the diff chain index.
  #[serde(rename = "diffChainSpam")]
  pub diff_chain_spam: Vec<MessageId>,
}

impl DocumentHistory {
  /// Read the [`DocumentHistory`] of the DID Document identified by the given [`IotaDID`] from the
  /// Tangle.
  pub async fn read(client: &Client, did: &IotaDID) -> Result<Self> {
    // Fetch and parse the integration chain
    let integration_messages: Vec<Message> = client.read_messages(did.tag()).await?;
    let integration_chain = IntegrationChain::try_from_messages(did, &integration_messages, client).await?;

    // Fetch and parse the diff chain for the last integration message
    let diff_index: String = IotaDocument::diff_index(integration_chain.current_message_id())?;
    let diff_messages: Vec<Message> = client.read_messages(&diff_index).await?;
    let diff_chain: DiffChain = DiffChain::try_from_messages(&integration_chain, &diff_messages, client).await?;

    let integration_chain_history: ChainHistory<ResolvedIotaDocument> =
      ChainHistory::from((integration_chain, integration_messages.deref()));
    let diff_chain_history: ChainHistory<DiffMessage> = ChainHistory::from((diff_chain, diff_messages.deref()));
    Ok(Self {
      integration_chain_data: integration_chain_history.chain_data,
      integration_chain_spam: integration_chain_history.spam,
      diff_chain_data: diff_chain_history.chain_data,
      diff_chain_spam: diff_chain_history.spam,
    })
  }
}

/// A list of messages on an integration chain or diff chain.
///
/// Retains a list of "spam" messages published on the same index that do not form part of the
/// resulting chain.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChainHistory<T> {
  #[serde(rename = "chainData")]
  pub chain_data: Vec<T>,
  pub spam: Vec<MessageId>,
}

impl<T> ChainHistory<T>
where
  T: TangleRef,
{
  /// Constructs a list of `spam` [`MessageIds`](MessageId).
  ///
  /// Assumes any message not in `chain_data` is "spam".
  pub fn separate_spam(chain_data: &[T], messages: &[Message]) -> Vec<MessageId> {
    // This is somewhat less efficient than separating the messages during construction of the chain
    // itself but avoids duplicating or polluting the construction logic.
    let chain_message_id_set: HashSet<MessageId> = chain_data.iter().map(TangleRef::message_id).cloned().collect();

    messages
      .iter()
      .filter_map(|message| Some(message.id().0).filter(|id| !chain_message_id_set.contains(id)))
      .collect()
  }
}

impl ChainHistory<DiffMessage> {
  /// Construct a [`ChainHistory`] of [`DiffMessages`](DiffMessage) for an integration chain
  /// [`ResolvedIotaDocument`].
  ///
  /// This is useful for constructing histories of old diff chains no longer at the end of an
  /// integration chain.
  pub async fn try_from_raw_messages(
    document: &ResolvedIotaDocument,
    messages: &[Message],
    client: &Client,
  ) -> Result<Self> {
    let did: &IotaDID = document.document.id();
    let index: MessageIndex<DiffMessage> = messages
      .iter()
      .flat_map(|message| message.try_extract_diff(did))
      .collect();

    let diff_chain = DiffChain::try_from_index_with_document(document, index, client).await?;
    Ok(Self::from((diff_chain, messages)))
  }
}

/// Construct [`ChainHistory`] from an [`IntegrationChain`].
impl From<(IntegrationChain, &[Message])> for ChainHistory<ResolvedIotaDocument> {
  fn from((integration_chain, messages): (IntegrationChain, &[Message])) -> Self {
    // The list contains the history and current document as the last element.
    let chain_data: Vec<ResolvedIotaDocument> = Vec::from(integration_chain);
    let spam: Vec<MessageId> = ChainHistory::separate_spam(&chain_data, messages);

    Self { chain_data, spam }
  }
}

/// Construct [`ChainHistory`] from a [`DiffChain`].
impl From<(DiffChain, &[Message])> for ChainHistory<DiffMessage> {
  fn from((diff_chain, messages): (DiffChain, &[Message])) -> Self {
    let chain_data: Vec<DiffMessage> = Vec::from(diff_chain);
    let spam: Vec<MessageId> = ChainHistory::separate_spam(&chain_data, messages);

    Self { chain_data, spam }
  }
}
