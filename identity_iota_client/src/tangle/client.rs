// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use bee_rest_api::types::dtos::LedgerInclusionStateDto;
use futures::stream::FuturesUnordered;
use futures::stream::TryStreamExt;
use identity_core::convert::ToJson;
use identity_iota_core::did::IotaDID;
use identity_iota_core::diff::DiffMessage;
use identity_iota_core::document::IotaDocument;
use identity_iota_core::tangle::Message;
use identity_iota_core::tangle::MessageId;
use identity_iota_core::tangle::Network;
use iota_client::Client as IotaClient;
use iota_client::Error as IotaClientError;

use crate::chain::ChainHistory;
use crate::chain::DiffChain;
use crate::chain::DocumentChain;
use crate::chain::DocumentHistory;
use crate::chain::IntegrationChain;
use crate::document::ResolvedIotaDocument;
use crate::error::Error;
use crate::error::Result;
use crate::tangle::ClientBuilder;
use crate::tangle::DIDMessageEncoding;
use crate::tangle::Receipt;
use crate::tangle::TangleRef;
use crate::tangle::TangleResolve;

/// Client for performing IOTA Identity operations on the Tangle.
#[derive(Debug)]
pub struct Client {
  pub(crate) client: IotaClient,
  pub(crate) network: Network,
  pub(crate) encoding: DIDMessageEncoding,
  pub(crate) retry_until_included: bool,
}

impl Client {
  /// Creates a new [`Client`] with default settings.
  pub async fn new() -> Result<Self> {
    Self::builder().build().await
  }

  /// Creates a [`ClientBuilder`] to configure a new [`Client`].
  ///
  /// This is the same as [`ClientBuilder::new`].
  pub fn builder() -> ClientBuilder {
    ClientBuilder::new()
  }

  /// Creates a new [`Client`] based on the [`ClientBuilder`] configuration.
  pub async fn from_builder(builder: ClientBuilder) -> Result<Self> {
    let mut client: iota_client::ClientBuilder = builder.builder;

    if !builder.nodeset {
      if let Some(network_url) = builder.network.default_node_url() {
        client = client.with_node(network_url.as_str())?;
      } else {
        return Err(Error::NoClientNodesProvided);
      }
    }

    Ok(Self {
      client: client.finish().await?,
      network: builder.network,
      encoding: builder.encoding,
      retry_until_included: builder.retry_until_included,
    })
  }

  /// Returns the IOTA [`Network`] that the [`Client`] is configured to use.
  pub fn network(&self) -> Network {
    self.network.clone()
  }

  /// Publishes an [`IotaDocument`] to the Tangle.
  /// If `retry_until_included` is true, this method calls `publish_json_with_retry` with its default `interval`
  /// and `max_attempts` values to increase the probability that the message will be referenced by a milestone.
  pub async fn publish_document(&self, document: &IotaDocument) -> Result<Receipt> {
    if document.id().network_str() != self.network.name_str() {
      return Err(Error::IncompatibleNetwork(format!(
        "DID network '{}' does not match client network '{}'",
        document.id().network_str(),
        self.network.name_str()
      )));
    }
    if self.retry_until_included {
      self
        .publish_json_with_retry(document.integration_index(), document, None, None)
        .await
    } else {
      self.publish_json(document.integration_index(), document).await
    }
  }

  /// Publishes a [`DiffMessage`] to the Tangle to form part of the diff chain for the integration.
  /// chain message specified by the given [`MessageId`].
  ///
  /// This method calls `publish_json_with_retry` with its default `interval` and `max_attempts`
  /// values for increasing the probability that the message will be referenced by a milestone.
  #[deprecated(since = "0.5.0", note = "diff chain features are slated for removal")]
  pub async fn publish_diff(&self, message_id: &MessageId, diff: &DiffMessage) -> Result<Receipt> {
    if diff.id().network_str() != self.network.name_str() {
      return Err(Error::IncompatibleNetwork(format!(
        "DID network '{}' does not match client network '{}'",
        diff.id().network_str(),
        self.network.name_str()
      )));
    }

    let diff_index = &IotaDocument::diff_index(message_id)?;
    if self.retry_until_included {
      self.publish_json_with_retry(diff_index, diff, None, None).await
    } else {
      self.publish_json(diff_index, diff).await
    }
  }

  /// Compresses and publishes arbitrary JSON data to the specified index on the Tangle.
  pub async fn publish_json<T: ToJson>(&self, index: &str, data: &T) -> Result<Receipt> {
    let message_data: Vec<u8> = crate::tangle::pack_did_message(data, self.encoding)?;
    self
      .client
      .message()
      .with_index(index)
      .with_data(message_data)
      .finish()
      .await
      .map_err(Into::into)
      .map(|message| Receipt::new(self.network.clone(), message))
  }

  /// Publishes arbitrary JSON data to the specified index on the Tangle.
  /// Retries (promotes or reattaches) the message until it’s included (referenced by a milestone).
  /// Default interval is 5 seconds and max attempts is 40.
  pub async fn publish_json_with_retry<T: ToJson>(
    &self,
    index: &str,
    data: &T,
    interval: Option<u64>,
    max_attempts: Option<u64>,
  ) -> Result<Receipt> {
    let receipt: Receipt = self.publish_json(index, data).await?;
    let retry_result: Result<Vec<(MessageId, Message)>, IotaClientError> = self
      .client
      .retry_until_included(receipt.message_id(), interval, max_attempts)
      .await;
    let reattached_messages: Vec<(MessageId, Message)> = match retry_result {
      Ok(reattached_messages) => reattached_messages,
      Err(inclusion_error @ IotaClientError::TangleInclusionError(_)) => {
        if self.is_message_included(receipt.message_id()).await? {
          return Ok(receipt);
        } else {
          return Err(Error::from(inclusion_error));
        }
      }
      Err(error) => {
        return Err(Error::from(error));
      }
    };
    match reattached_messages.into_iter().next() {
      Some((_, message)) => Ok(Receipt::new(self.network.clone(), message)),
      None => Err(Error::from(IotaClientError::TangleInclusionError(
        receipt.message_id().to_string(),
      ))),
    }
  }

  /// Fetch the [`IotaDocument`] specified by the given [`IotaDID`].
  pub async fn read_document(&self, did: &IotaDID) -> Result<ResolvedIotaDocument> {
    self.read_document_chain(did).await.and_then(DocumentChain::fold)
  }

  /// Fetches a [`DocumentChain`] given an [`IotaDID`].
  pub async fn read_document_chain(&self, did: &IotaDID) -> Result<DocumentChain> {
    log::trace!("Read Document Chain: {}", did);
    if did.network_str() != self.network.name_str() {
      return Err(Error::DIDNotFound(format!(
        "DID network '{}' does not match client network '{}'",
        did.network_str(),
        self.network.name_str()
      )));
    }

    // Fetch all messages for the integration chain.
    log::trace!("Integration Chain Address: {}", did.tag());
    let messages: Vec<Message> = self.read_messages(did.tag()).await?;
    let integration_chain: IntegrationChain = IntegrationChain::try_from_messages(did, &messages, self).await?;

    // TODO: do we still want to support this, replace with ResolutionOptions?
    // // Check if there is any query given and return
    // let skip_diff: bool = did_url.query_pairs().any(|(key, value)| key == "diff" && value == "false");
    //
    // let diff: DiffChain = if skip_diff {
    //   DiffChain::new()
    // } else {
    //   // Fetch all messages for the diff chain.
    //   let index: String = IotaDocument::diff_index(integration_chain.current_message_id())?;
    //   let messages: Vec<Message> = self.read_messages(&index).await?;
    //
    //   log::trace!("Diff Messages: {:#?}", messages);
    //
    //   DiffChain::try_from_messages(&integration_chain, &messages)?
    // };

    // Fetch the latest diff chain.
    let diff_chain: DiffChain = {
      let index: String = IotaDocument::diff_index(integration_chain.current_message_id())?;
      let messages: Vec<Message> = self.read_messages(&index).await?;

      log::trace!("Diff Messages: {:#?}", messages);

      DiffChain::try_from_messages(&integration_chain, &messages, self).await?
    };

    DocumentChain::new_with_diff_chain(integration_chain, diff_chain)
  }

  /// Returns the [`DocumentHistory`] of the given [`IotaDID`].
  pub async fn resolve_history(&self, did: &IotaDID) -> Result<DocumentHistory> {
    DocumentHistory::read(self, did).await
  }

  /// Returns the [`ChainHistory`] of a diff chain starting from an [`IotaDocument`] on the
  /// integration chain.
  ///
  /// NOTE: the document must have been published to the Tangle and have a valid message id.
  #[deprecated(since = "0.5.0", note = "diff chain features are slated for removal")]
  pub async fn resolve_diff_history(&self, document: &ResolvedIotaDocument) -> Result<ChainHistory<DiffMessage>> {
    let diff_index: String = IotaDocument::diff_index(document.message_id())?;
    let diff_messages: Vec<Message> = self.read_messages(&diff_index).await?;
    ChainHistory::try_from_raw_messages(document, &diff_messages, self).await
  }

  /// Fetch all [`Messages`][Message] from the given index on the IOTA Tangle.
  pub(crate) async fn read_messages(&self, index: &str) -> Result<Vec<Message>> {
    let message_ids: Box<[MessageId]> = Self::read_message_index(&self.client, index).await?;
    let messages: Vec<Message> = Self::read_message_data(&self.client, &message_ids).await?;

    Ok(messages)
  }

  /// Read the [`MessageIds`][MessageId] from the given index on the IOTA Tangle.
  ///
  /// NOTE: the number of returned [`MessageIds`][MessageId] is capped at 1000 due to limitations of
  ///       the current API, with no ordering guarantees.
  async fn read_message_index(client: &IotaClient, index: &str) -> Result<Box<[MessageId]>> {
    client.get_message().index(index).await.map_err(Into::into)
  }

  /// Fetch the associated [`Messages`][Message] given a list of [MessageIds][`MessageId`] from
  /// the Tangle.
  async fn read_message_data(client: &IotaClient, messages: &[MessageId]) -> Result<Vec<Message>> {
    messages
      .iter()
      .map(|message_id| client.get_message().data(message_id))
      .collect::<FuturesUnordered<_>>()
      .try_collect()
      .await
      .map_err(Into::into)
  }

  /// Checks if a message is confirmed by a milestone.
  pub async fn is_message_included(&self, message_id: &MessageId) -> Result<bool> {
    match self
      .client
      .get_message()
      .metadata(message_id)
      .await?
      .ledger_inclusion_state
    {
      Some(ledger_inclusion_state) => match ledger_inclusion_state {
        LedgerInclusionStateDto::Included | LedgerInclusionStateDto::NoTransaction => Ok(true),
        LedgerInclusionStateDto::Conflicting => Ok(false),
      },
      None => Ok(false),
    }
  }
}

#[async_trait::async_trait(?Send)]
impl TangleResolve for Client {
  async fn resolve(&self, did: &IotaDID) -> Result<ResolvedIotaDocument> {
    self.read_document(did).await
  }
}
