// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;
use std::sync::Arc;

use dashmap::DashMap;

use identity_core::common::Url;
use identity_credential::credential::Credential;
use identity_credential::presentation::Presentation;

use crate::chain::ChainHistory;
use crate::chain::DocumentHistory;
use crate::did::IotaDID;
use crate::diff::DiffMessage;
use crate::document::ResolvedIotaDocument;
use crate::error::Error;
use crate::error::Result;
use crate::tangle::Client;
use crate::tangle::ClientBuilder;
use crate::tangle::NetworkName;
use crate::tangle::TangleResolve;

/// TODO: description
#[derive(Debug)]
pub struct Resolver {
  // TODO: use Rc internally for Wasm? Typedef?
  client_map: DashMap<NetworkName, Arc<Client>>,
}

impl Resolver {
  /// Constructs a new [`Resolver`] with a default [`Client`] for
  /// the [`Mainnet`](crate::tangle::Network::Mainnet).
  ///
  /// See also [`Resolver::builder`].
  pub async fn new() -> Result<Self> {
    let client_map: DashMap<NetworkName, Arc<Client>> = DashMap::new();
    let client: Client = Client::new().await?;
    client_map.insert(client.network.name(), Arc::new(client));

    Ok(Self { client_map })
  }

  /// Constructs a new [`Resolver`] with no configured [`Clients`](Client).
  ///
  /// See [`Resolver::client`] and [`Resolver::client_builder`].
  // TODO: make a proper ResolverBuilder or rename this empty()/new_empty()?
  #[must_use]
  pub fn builder() -> Self {
    Self {
      client_map: DashMap::new(),
    }
  }

  /// Inserts a [`Client`]. Use with [`Resolver::builder`].
  ///
  /// NOTE: replaces any existing [`Client`] entry with the same [`NetworkName`].
  #[must_use]
  pub fn client(self, client: Client) -> Self {
    self.set_client(Arc::new(client));
    self
  }

  /// Creates and inserts a new [`Client`] from the given builder. Use with [`Resolver::builder`].
  ///
  /// NOTE: replaces any existing [`Client`] entry with the same [`NetworkName`].
  pub async fn client_builder(self, builder: ClientBuilder) -> Result<Self> {
    Client::from_builder(builder).await.map(|client| self.client(client))
  }

  /// Inserts a [`Client`].
  ///
  /// NOTE: replaces any existing [`Client`] entry with the same [`NetworkName`].
  pub fn set_client(&self, client: Arc<Client>) {
    self.client_map.insert(client.network.name(), client);
  }

  /// Removes the [`Client`] corresponding to the given [`NetworkName`] and returns it
  /// if one exists.
  pub fn remove_client(&self, network_name: &NetworkName) -> Option<Arc<Client>> {
    self.client_map.remove(network_name).map(|(_, client)| client)
  }

  /// Returns the [`Client`] corresponding to the given [`NetworkName`] if one exists.
  pub fn get_client(&self, network_name: &NetworkName) -> Option<Arc<Client>> {
    self
      .client_map
      .get(network_name)
      .map(|client_ref| Arc::clone(client_ref.value()))
  }

  /// Returns the [`Client`] corresponding to the [`NetworkName`] on the given ['IotaDID'].
  fn get_client_for_did(&self, did: &IotaDID) -> Result<Arc<Client>> {
    self.get_client(&did.network()?.name()).ok_or_else(|| {
      Error::DIDNotFound(format!(
        "DID network '{}' does not match any resolver client network",
        did.network_str(),
      ))
    })
  }

  /// Fetches the [`IotaDocument`] of the given [`IotaDID`].
  pub async fn resolve(&self, did: &IotaDID) -> Result<ResolvedIotaDocument> {
    let client: Arc<Client> = self.get_client_for_did(did)?;
    client.read_document(did).await
  }

  /// Fetches the [`DocumentHistory`] of the given [`IotaDID`].
  pub async fn resolve_history(&self, did: &IotaDID) -> Result<DocumentHistory> {
    let client: Arc<Client> = self.get_client_for_did(did)?;
    client.resolve_history(did).await
  }

  /// Fetches the [`ChainHistory`] of a diff chain starting from an [`IotaDocument`] on the
  /// integration chain.
  ///
  /// NOTE: the document must have been published to the Tangle and have a valid message id.
  pub async fn resolve_diff_history(&self, document: &ResolvedIotaDocument) -> Result<ChainHistory<DiffMessage>> {
    let client: Arc<Client> = self.get_client_for_did(document.document.id())?;
    client.resolve_diff_history(document).await
  }

  /// Fetches the DID Document of the issuer on a [`Credential`].
  ///
  /// # Errors
  ///
  /// Errors if the issuer URL is not a valid [`IotaDID`] or DID resolution fails.
  pub async fn resolve_credential_issuer(&self, credential: &Credential) -> Result<ResolvedIotaDocument> {
    let issuer: IotaDID = IotaDID::parse(credential.issuer.url().as_str())?;
    self.resolve(&issuer).await
  }

  /// Fetches all DID Documents of [`Credential`] issuers contained in a [`Presentation`].
  /// Issuer documents are returned in arbitrary order.
  ///
  /// # Errors
  ///
  /// Errors if any issuer URL is not a valid [`IotaDID`] or DID resolution fails.
  pub async fn resolve_presentation_issuers(&self, presentation: &Presentation) -> Result<Vec<ResolvedIotaDocument>> {
    // Extract unique issuers.
    let mut issuers: HashSet<IotaDID> = HashSet::new();
    for credential in presentation.verifiable_credential.iter() {
      let issuer: IotaDID = IotaDID::parse(credential.issuer.url().as_str())?;
      issuers.insert(issuer);
    }

    // Resolve issuers concurrently.
    futures::future::try_join_all(issuers.iter().map(|issuer| self.resolve(issuer)).collect::<Vec<_>>()).await
  }

  /// Fetches the DID Document of the holder of a [`Presentation`].
  ///
  /// # Errors
  ///
  /// Errors if the holder URL is missing, is not a valid [`IotaDID`], or DID resolution fails.
  pub async fn resolve_presentation_holder(&self, presentation: &Presentation) -> Result<ResolvedIotaDocument> {
    let holder_url: &Url = presentation.holder.as_ref().ok_or(Error::InvalidPresentationHolder)?;
    let holder: IotaDID = IotaDID::parse(holder_url.as_str())?;
    self.resolve(&holder).await
  }
}

#[async_trait::async_trait(? Send)]
impl TangleResolve for Resolver {
  async fn resolve(&self, did: &IotaDID) -> Result<ResolvedIotaDocument> {
    self.resolve(did).await
  }
}
