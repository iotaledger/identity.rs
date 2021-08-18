// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use futures::stream::FuturesUnordered;
use futures::stream::TryStreamExt;
use identity_core::convert::ToJson;
use iota_client::Client as IotaClient;

use crate::chain::DiffChain;
use crate::chain::DocumentChain;
use crate::chain::IntegrationChain;
use crate::did::DocumentDiff;
use crate::did::IotaDID;
use crate::did::IotaDocument;
use crate::error::Error;
use crate::error::Result;
use crate::tangle::ClientBuilder;
use crate::tangle::Message;
use crate::tangle::MessageId;
use crate::tangle::Network;
use crate::tangle::Receipt;
use crate::tangle::TangleResolve;

#[derive(Debug)]
pub struct Client {
  pub(crate) client: IotaClient,
  pub(crate) network: Network,
}

impl Client {
  /// Creates a new `Client` with default settings.
  pub async fn new() -> Result<Self> {
    Self::builder().build().await
  }

  /// Creates a `ClientBuilder` to configure a new `Client`.
  ///
  /// This is the same as [ClientBuilder::new]`.
  pub fn builder() -> ClientBuilder {
    ClientBuilder::new()
  }

  /// Creates a new `Client` with default settings for the given `Network`.
  pub async fn from_network(network: Network) -> Result<Self> {
    Self::builder().network(network).build().await
  }

  /// Creates a new `Client` based on the `ClientBuilder` configuration.
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
    })
  }

  /// Returns the `Client` Tangle network.
  pub fn network(&self) -> Network {
    self.network.clone()
  }

  /// Publishes an `IotaDocument` to the Tangle.
  pub async fn publish_document(&self, document: &IotaDocument) -> Result<Receipt> {
    self
      .client
      .message()
      .with_index(document.id().tag())
      .with_data(document.to_json_vec()?)
      .finish()
      .await
      .map_err(Into::into)
      .map(|message| Receipt::new(self.network.clone(), message))
  }

  /// Publishes a `DocumentDiff` to the Tangle.
  pub async fn publish_diff(&self, message_id: &MessageId, diff: &DocumentDiff) -> Result<Receipt> {
    self
      .client
      .message()
      .with_index(&IotaDocument::diff_address(message_id)?)
      .with_data(diff.to_json_vec()?)
      .finish()
      .await
      .map_err(Into::into)
      .map(|message| Receipt::new(self.network.clone(), message))
  }

  pub async fn read_document(&self, did: &IotaDID) -> Result<IotaDocument> {
    self.read_document_chain(did).await.and_then(DocumentChain::fold)
  }

  pub async fn read_document_chain(&self, did: &IotaDID) -> Result<DocumentChain> {
    trace!("Read Document Chain: {}", did);
    trace!("Integration Chain Address: {}", did.tag());

    // Fetch all messages for the integration chain.
    let messages: Vec<Message> = Self::read_messages(&self.client, did.tag()).await?;
    let integration_chain: IntegrationChain = IntegrationChain::try_from_messages(did, &messages)?;

    // Check if there is any query given and return
    let skip_diff: bool = did.query_pairs().any(|(key, value)| key == "diff" && value == "false");

    let diff: DiffChain = if skip_diff {
      DiffChain::new()
    } else {
      // Fetch all messages for the diff chain.
      let address: String = IotaDocument::diff_address(integration_chain.current_message_id())?;
      let messages: Vec<Message> = Self::read_messages(&self.client, &address).await?;

      trace!("Diff Messages: {:#?}", messages);

      DiffChain::try_from_messages(&integration_chain, &messages)?
    };

    DocumentChain::with_diff_chain(integration_chain, diff)
  }

  async fn read_messages(client: &IotaClient, address: &str) -> Result<Vec<Message>> {
    let message_ids: Box<[MessageId]> = Self::read_message_index(client, address).await?;
    let messages: Vec<Message> = Self::read_message_data(client, &message_ids).await?;

    Ok(messages)
  }

  async fn read_message_index(client: &IotaClient, address: &str) -> Result<Box<[MessageId]>> {
    client.get_message().index(address).await.map_err(Into::into)
  }

  async fn read_message_data(client: &IotaClient, messages: &[MessageId]) -> Result<Vec<Message>> {
    messages
      .iter()
      .map(|message| client.get_message().data(message))
      .collect::<FuturesUnordered<_>>()
      .try_collect()
      .await
      .map_err(Into::into)
  }
}

#[async_trait::async_trait(?Send)]
impl TangleResolve for Client {
  async fn resolve(&self, did: &IotaDID) -> Result<IotaDocument> {
    self.read_document(did).await
  }
}
