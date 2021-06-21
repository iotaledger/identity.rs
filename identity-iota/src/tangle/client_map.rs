// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use crate::chain::DocumentChain;
use crate::did::DocumentDiff;
use crate::did::IotaDID;
use crate::did::IotaDocument;
use crate::error::Error;
use crate::error::Result;
use crate::tangle::Client;
use crate::tangle::ClientBuilder;
use crate::tangle::MessageId;
use crate::tangle::Network;
use crate::tangle::TangleResolve;

type State = HashMap<Network, Arc<Client>>;

#[derive(Debug)]
pub struct ClientMap {
  data: RwLock<State>,
}

impl ClientMap {
  pub fn new() -> Self {
    Self {
      data: RwLock::new(State::new()),
    }
  }

  pub fn from_client(client: Client) -> Self {
    let mut data: HashMap<Network, Arc<Client>> = HashMap::new();

    data.insert(client.network, Arc::new(client));

    Self {
      data: RwLock::new(data),
    }
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

  pub fn networks(&self) -> Vec<Network> {
    self.data.read().unwrap().keys().copied().collect()
  }

  pub async fn publish_document(&self, document: &IotaDocument) -> Result<MessageId> {
    let network: Network = document.id().network();
    let client: Arc<Client> = self.client(network).await?;

    client.publish_document(document).await
  }

  pub async fn publish_diff(&self, message_id: &MessageId, diff: &DocumentDiff) -> Result<MessageId> {
    let network: Network = diff.id().network();
    let client: Arc<Client> = self.client(network).await?;

    client.publish_diff(message_id, diff).await
  }

  pub async fn resolve(&self, did: &IotaDID) -> Result<IotaDocument> {
    let network: Network = did.network();
    let client: Arc<Client> = self.client(network).await?;

    client.resolve(did).await
  }

  pub async fn read_document_chain(&self, did: &IotaDID) -> Result<DocumentChain> {
    let network: Network = did.network();
    let client: Arc<Client> = self.client(network).await?;

    client.read_document_chain(did).await
  }

  pub async fn client(&self, network: Network) -> Result<Arc<Client>> {
    {
      let client: Option<Arc<Client>> = self
        .data
        .read()
        .map_err(|_| Error::SharedReadPoisoned)
        .map(|guard| guard.get(&network).map(Arc::clone))?;

      if let Some(client) = client {
        return Ok(client);
      }
    }

    let client: Arc<Client> = Client::from_network(network).await.map(Arc::new)?;

    self
      .data
      .write()
      .map_err(|_| Error::SharedWritePoisoned)?
      .insert(network, Arc::clone(&client));

    Ok(client)
  }
}

impl Default for ClientMap {
  fn default() -> Self {
    Self::new()
  }
}

impl From<Client> for ClientMap {
  fn from(other: Client) -> Self {
    Self::from_client(other)
  }
}

#[async_trait::async_trait(?Send)]
impl TangleResolve for ClientMap {
  async fn resolve(&self, did: &IotaDID) -> Result<IotaDocument> {
    self.client(did.network()).await?.resolve(did).await
  }
}
