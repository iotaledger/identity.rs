// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::chain::DiffChain;
use crate::chain::DocumentChain;
use crate::chain::IntegrationChain;
use crate::client::ClientBuilder;
use crate::client::Network;
use crate::did::DocumentDiff;
use crate::did::IotaDID;
use crate::did::IotaDocument;
use crate::error::Error;
use crate::error::Result;
use futures::stream::FuturesUnordered;
use futures::stream::TryStreamExt;
use identity_core::common::Url;
use identity_core::convert::ToJson;
use iota_client::bee_message::Message;
use iota_client::bee_message::MessageId;

#[derive(Clone, Debug)]
pub struct Messages {
  message_ids: Box<[MessageId]>,
  messages: Vec<Message>,
}

#[derive(Debug)]
pub struct Client {
  pub(crate) client: iota_client::Client,
  pub(crate) network: Network,
}

impl Client {
  /// Creates a new `Client`  with default settings.
  pub async fn new() -> Result<Self> {
    Self::from_builder(Self::builder()).await
  }

  /// Creates a `ClientBuilder` to configure a new `Client`.
  ///
  /// This is the same as [ClientBuilder::new]`.
  pub fn builder() -> ClientBuilder {
    ClientBuilder::new()
  }

  /// Creates a new `Client` with default settings for the given `Network`.
  pub async fn from_network(network: Network) -> Result<Self> {
    Self::builder()
      .node(network.node_url().as_str())
      .network(network)
      .build()
      .await
  }

  /// Creates a new `Client` based on the `ClientBuilder` configuration.
  pub async fn from_builder(builder: ClientBuilder) -> Result<Self> {
    let mut client: iota_client::ClientBuilder = iota_client::ClientBuilder::new();

    if builder.nodes.is_empty() {
      client = client.with_node(builder.network.node_url().as_str())?;
    } else {
      let nodes: Vec<&str> = builder.nodes.iter().map(|node| node.as_str()).collect();
      client = client.with_nodes(&nodes)?;
    }
    if !builder.node_sync_enabled {
      client = client.with_node_sync_disabled();
    }

    client = client.with_network(builder.network.as_str());

    Ok(Self {
      client: client.finish().await?,
      network: builder.network,
    })
  }

  /// Returns the `Client` Tangle network.
  pub fn network(&self) -> Network {
    self.network
  }

  /// Returns the default node URL of the `Client` network.
  pub fn default_node_url(&self) -> &'static Url {
    self.network.node_url()
  }

  /// Returns the web explorer URL of the `Client` network.
  pub fn explorer_url(&self) -> &'static Url {
    self.network.explorer_url()
  }

  /// Returns the web explorer URL of the given `transaction`.
  pub fn transaction_url(&self, message_id: &str) -> Url {
    format!("{}/message/{}", self.network.explorer_url(), message_id)
      .parse()
      .unwrap()
  }

  /// Publishes an `Document` to the Tangle.
  ///
  /// Note: The only validation performed is to ensure the correct Tangle
  /// network is selected.
  pub async fn publish_document(&self, document: &IotaDocument) -> Result<MessageId> {
    debug!("Publish Document: {}", document.id());
    debug!("Document JSON = {:#}", document);

    debug_assert!(document.verify().is_ok());

    self.check_network(document.id())?;

    let message_builder = self
      .client
      .message()
      .with_index(document.id().tag())
      .with_data(document.to_json()?.into_bytes());

    let message = message_builder.finish().await?;

    Ok(message.id().0)
  }

  /// Publishes a `DocumentDiff` to the Tangle.
  ///
  /// Note: The only validation performed is to ensure the correct Tangle
  /// network is selected.
  pub async fn publish_diff(&self, message_id: &MessageId, diff: &DocumentDiff) -> Result<MessageId> {
    debug!("Publish Diff: {}", diff.id());
    debug!("Diff JSON = {}", diff.to_json_pretty()?);

    self.check_network(diff.id())?;

    let message: Message = self
      .client
      .message()
      .with_index(&IotaDocument::diff_address(message_id)?)
      .with_data(diff.to_json()?.into_bytes())
      .finish()
      .await?;

    Ok(message.id().0)
  }

  pub async fn read_document(&self, did: &IotaDID) -> Result<IotaDocument> {
    self.read_document_chain(did).await.and_then(DocumentChain::fold)
  }

  pub async fn read_document_chain(&self, did: &IotaDID) -> Result<DocumentChain> {
    trace!("Read Document Chain: {}", did);
    trace!("Integration Chain Address: {}", did.tag());

    // Fetch all messages for the integration chain.
    let messages: Messages = self.read_messages(did.tag()).await?;
    let integration_chain: IntegrationChain = IntegrationChain::try_from_messages(did, &messages.messages)?;

    // Check if there is any query given and return
    let skip_diff: bool = did.query_pairs().any(|(key, value)| key == "diff" && value == "false");

    let diff: DiffChain = if skip_diff {
      DiffChain::new()
    } else {
      // Fetch all messages for the diff chain.
      let address: String = IotaDocument::diff_address(integration_chain.current_message_id())?;
      let messages: Messages = self.read_messages(&address).await?;

      trace!("Tangle Messages: {:?}", messages);

      DiffChain::try_from_messages(&integration_chain, &messages.messages)?
    };

    DocumentChain::with_diff_chain(integration_chain, diff)
  }

  pub async fn read_messages(&self, address: &str) -> Result<Messages> {
    let message_ids: Box<[MessageId]> = self.client.get_message().index(address).await?;

    let messages: Vec<Message> = message_ids
      .iter()
      .map(|message| self.client.get_message().data(message))
      .collect::<FuturesUnordered<_>>()
      .try_collect()
      .await?;

    Ok(Messages { message_ids, messages })
  }

  pub fn check_network(&self, did: &IotaDID) -> Result<()> {
    if !self.network.matches_did(did) {
      return Err(Error::InvalidDIDNetwork);
    }

    Ok(())
  }
}
