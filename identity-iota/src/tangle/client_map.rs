// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use dashmap::DashMap;

use crate::chain::DocumentChain;
use crate::did::DocumentDiff;
use crate::did::IotaDID;
use crate::did::IotaDocument;
use crate::error::Result;
use crate::tangle::Client;
use crate::tangle::ClientBuilder;
use crate::tangle::MessageId;
use crate::tangle::Network;
use crate::tangle::NetworkName;
use crate::tangle::Receipt;
use crate::tangle::TangleResolve;

type State = DashMap<NetworkName, Arc<Client>>;

#[derive(Debug)]
pub struct ClientMap {
  data: State,
}

impl ClientMap {
  pub fn new() -> Self {
    Self { data: State::new() }
  }

  pub fn from_client(client: Client) -> Self {
    let data: State = State::new();

    data.insert(client.network.name(), Arc::new(client));

    Self { data }
  }

  pub fn builder() -> ClientBuilder {
    ClientBuilder::new()
  }

  pub async fn from_network(network: Network) -> Result<Self> {
    Client::from_network(network).await.map(Self::from_client)
  }

  pub async fn from_builder(builder: ClientBuilder) -> Result<Self> {
    Client::from_builder(builder).await.map(Self::from_client)
  }

  pub fn insert(&self, client: Client) {
    self.data.insert(client.network.name(), Arc::new(client));
  }

  pub async fn publish_document(&self, document: &IotaDocument) -> Result<Receipt> {
    let network: Network = document.id().network()?;
    let client: Arc<Client> = self.client(network).await?;

    client.publish_document(document).await
  }

  pub async fn publish_diff(&self, message_id: &MessageId, diff: &DocumentDiff) -> Result<Receipt> {
    let network: Network = diff.id().network()?;
    let client: Arc<Client> = self.client(network).await?;

    client.publish_diff(message_id, diff).await
  }

  pub async fn read_document(&self, did: &IotaDID) -> Result<IotaDocument> {
    let network: Network = did.network()?;
    let client: Arc<Client> = self.client(network).await?;

    client.read_document(did).await
  }

  pub async fn read_document_chain(&self, did: &IotaDID) -> Result<DocumentChain> {
    let network: Network = did.network()?;
    let client: Arc<Client> = self.client(network).await?;

    client.read_document_chain(did).await
  }

  pub async fn client(&self, network: Network) -> Result<Arc<Client>> {
    let network_name = network.name();
    if let Some(client) = self.data.get(&network_name) {
      return Ok(Arc::clone(&client));
    }

    let client: Arc<Client> = Client::from_network(network.clone()).await.map(Arc::new)?;

    self.data.insert(network_name, Arc::clone(&client));

    Ok(client)
  }
}

impl Default for ClientMap {
  fn default() -> Self {
    Self::new()
  }
}

#[async_trait::async_trait(?Send)]
impl TangleResolve for ClientMap {
  async fn resolve(&self, did: &IotaDID) -> Result<IotaDocument> {
    self.read_document(did).await
  }
}
