// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::collections::HashSet;
#[cfg(not(feature = "arc"))]
use std::rc::Rc;
#[cfg(feature = "arc")]
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

#[cfg(not(feature = "arc"))]
type ClientRc = Rc<Client>;

#[cfg(feature = "arc")]
type ClientRc = Arc<Client>;

/// A `Resolver` supports resolving DID Documents across different Tangle networks using
/// multiple [`Clients`][Client].
///
/// Also provides convenience functions for resolving DID Documents associated with
/// verifiable [`Credentials`][Credential] and [`Presentations`][Presentation].
#[derive(Debug)]
pub struct Resolver {
  // TODO: is DashMap really necessary here?
  //  We can avoid mutation if we disallow adding/removing Clients post-creation.
  client_map: DashMap<NetworkName, ClientRc>,
}

impl Resolver {
  /// Constructs a new [`Resolver`] with a default [`Client`] for
  /// the [`Mainnet`](crate::tangle::Network::Mainnet).
  ///
  /// See also [`Resolver::builder`].
  pub async fn new() -> Result<Self> {
    let client_map: DashMap<NetworkName, ClientRc> = DashMap::new();
    let client: Client = Client::new().await?;
    client_map.insert(client.network.name(), ClientRc::new(client));

    Ok(Self { client_map })
  }

  /// Returns a new [`ResolverBuilder`] with no configured [`Clients`](Client).
  pub fn builder() -> ResolverBuilder {
    ResolverBuilder::new()
  }

  /// Inserts a [`Client`].
  ///
  /// NOTE: replaces any existing [`Client`] entry with the same [`NetworkName`].
  // TODO: remove this?
  pub fn set_client(&self, client: ClientRc) {
    self.client_map.insert(client.network.name(), client);
  }

  /// Removes the [`Client`] corresponding to the given [`NetworkName`] and returns it
  /// if one exists.
  // TODO: remove this?
  pub fn remove_client(&self, network_name: &NetworkName) -> Option<ClientRc> {
    self.client_map.remove(network_name).map(|(_, client)| client)
  }

  /// Returns the [`Client`] corresponding to the given [`NetworkName`] if one exists.
  pub fn get_client(&self, network_name: &NetworkName) -> Option<ClientRc> {
    self
      .client_map
      .get(network_name)
      .map(|client_ref| Arc::clone(client_ref.value()))
  }

  /// Returns the [`Client`] corresponding to the [`NetworkName`] on the given ['IotaDID'].
  fn get_client_for_did(&self, did: &IotaDID) -> Result<ClientRc> {
    self.get_client(&did.network()?.name()).ok_or_else(|| {
      Error::DIDNotFound(format!(
        "DID network '{}' does not match any resolver client network",
        did.network_str(),
      ))
    })
  }

  /// Fetches the [`IotaDocument`] of the given [`IotaDID`].
  pub async fn resolve(&self, did: &IotaDID) -> Result<ResolvedIotaDocument> {
    let client: ClientRc = self.get_client_for_did(did)?;
    client.read_document(did).await
  }

  /// Fetches the [`DocumentHistory`] of the given [`IotaDID`].
  pub async fn resolve_history(&self, did: &IotaDID) -> Result<DocumentHistory> {
    let client: ClientRc = self.get_client_for_did(did)?;
    client.resolve_history(did).await
  }

  /// Fetches the [`ChainHistory`] of a diff chain starting from an [`IotaDocument`] on the
  /// integration chain.
  ///
  /// NOTE: the document must have been published to the Tangle and have a valid message id.
  pub async fn resolve_diff_history(&self, document: &ResolvedIotaDocument) -> Result<ChainHistory<DiffMessage>> {
    let client: ClientRc = self.get_client_for_did(document.document.id())?;
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

/// Builder for configuring [`Clients`][Client] when constructing a [`Resolver`].
#[derive(Default)]
pub struct ResolverBuilder {
  clients: HashMap<NetworkName, ClientOrBuilder>,
}

#[allow(clippy::large_enum_variant)]
enum ClientOrBuilder {
  Client(ClientRc),
  Builder(ClientBuilder),
}

impl ResolverBuilder {
  /// Constructs a new [`ResolverBuilder`] with no [`Clients`][Client] configured.
  pub fn new() -> Self {
    Self {
      clients: Default::default(),
    }
  }

  /// Inserts a [`Client`].
  ///
  /// NOTE: replaces any previous [`Client`] or [`ClientBuilder`] with the same [`NetworkName`].
  #[must_use]
  pub fn client(mut self, client: ClientRc) -> Self {
    self
      .clients
      .insert(client.network.name(), ClientOrBuilder::Client(client));
    self
  }

  /// Inserts a [`ClientBuilder`].
  ///
  /// NOTE: replaces any previous [`Client`] or [`ClientBuilder`] with the same [`NetworkName`].
  pub fn client_builder(mut self, builder: ClientBuilder) -> Self {
    self
      .clients
      .insert(builder.network.name(), ClientOrBuilder::Builder(builder));
    self
  }

  /// Constructs a new [`Resolver`] based on the builder configuration.
  pub async fn build(self) -> Result<Resolver> {
    let client_map: DashMap<NetworkName, ClientRc> = DashMap::new();
    for (network_name, client_or_builder) in self.clients {
      let client: ClientRc = match client_or_builder {
        ClientOrBuilder::Client(client) => client,
        ClientOrBuilder::Builder(builder) => ClientRc::new(builder.build().await?),
      };
      client_map.insert(network_name, client);
    }

    Ok(Resolver { client_map })
  }
}

#[async_trait::async_trait(? Send)]
impl TangleResolve for Resolver {
  async fn resolve(&self, did: &IotaDID) -> Result<ResolvedIotaDocument> {
    self.resolve(did).await
  }
}
