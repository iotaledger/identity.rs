// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

use identity_core::common::Url;
use identity_credential::credential::Credential;
use identity_credential::presentation::Presentation;
use identity_iota_core::did::IotaDID;
use identity_iota_core::diff::DiffMessage;
use identity_iota_core::tangle::NetworkName;
use serde::Serialize;

use crate::chain::ChainHistory;
use crate::chain::DocumentHistory;
use crate::credential::FailFast;
use crate::credential::PresentationValidationOptions;
use crate::credential::PresentationValidator;
use crate::document::ResolvedIotaDocument;
use crate::error::Error;
use crate::error::Result;
use crate::tangle::Client;
use crate::tangle::ClientBuilder;
use crate::tangle::SharedPtr;
use crate::tangle::TangleResolve;

/// A `Resolver` supports resolving DID Documents across different Tangle networks using
/// multiple [`Clients`][Client].
///
/// Also provides convenience functions for resolving DID Documents associated with
/// verifiable [`Credentials`][Credential] and [`Presentations`][Presentation].
#[derive(Debug)]
pub struct Resolver<C = Arc<Client>>
where
  C: SharedPtr<Client>,
{
  client_map: HashMap<NetworkName, C>,
}

impl<C> Resolver<C>
where
  C: SharedPtr<Client>,
{
  /// Constructs a new [`Resolver`] with a default [`Client`] for
  /// the [`Mainnet`](identity_iota_core::tangle::Network::Mainnet).
  ///
  /// See also [`Resolver::builder`].
  pub async fn new() -> Result<Self> {
    let client: Client = Client::new().await?;

    let mut client_map: HashMap<NetworkName, C> = HashMap::new();
    client_map.insert(client.network.name(), C::from(client));
    Ok(Self { client_map })
  }

  /// Returns a new [`ResolverBuilder`] with no configured [`Clients`](Client).
  pub fn builder() -> ResolverBuilder {
    ResolverBuilder::new()
  }

  /// Returns the [`Client`] corresponding to the given [`NetworkName`] if one exists.
  pub fn get_client(&self, network_name: &NetworkName) -> Option<&C> {
    self.client_map.get(network_name)
  }

  /// Returns the [`Client`] corresponding to the [`NetworkName`] on the given ['IotaDID'].
  fn get_client_for_did(&self, did: &IotaDID) -> Result<&C> {
    self.get_client(&did.network()?.name()).ok_or_else(|| {
      Error::DIDNotFound(format!(
        "DID network '{}' does not match any resolver client network",
        did.network_str(),
      ))
    })
  }

  /// Fetches the [`ResolvedIotaDocument`] of the given [`IotaDID`].
  pub async fn resolve(&self, did: &IotaDID) -> Result<ResolvedIotaDocument> {
    let client: &Client = self.get_client_for_did(did)?.deref();
    client.read_document(did).await
  }

  /// Fetches the [`DocumentHistory`] of the given [`IotaDID`].
  pub async fn resolve_history(&self, did: &IotaDID) -> Result<DocumentHistory> {
    let client: &Client = self.get_client_for_did(did)?.deref();
    client.resolve_history(did).await
  }

  /// Fetches the [`ChainHistory`] of a diff chain starting from a [`ResolvedIotaDocument`] on the
  /// integration chain.
  ///
  /// NOTE: the document must have been published to the Tangle and have a valid message id.
  #[deprecated(since = "0.5.0", note = "diff chain features are slated for removal")]
  pub async fn resolve_diff_history(&self, document: &ResolvedIotaDocument) -> Result<ChainHistory<DiffMessage>> {
    let client: &Client = self.get_client_for_did(document.document.id())?.deref();
    client.resolve_diff_history(document).await
  }

  /// Fetches the DID Document of the issuer on a [`Credential`].
  ///
  /// # Errors
  ///
  /// Errors if the issuer URL is not a valid [`IotaDID`] or DID resolution fails.
  pub async fn resolve_credential_issuer<U: Serialize>(
    &self,
    credential: &Credential<U>,
  ) -> Result<ResolvedIotaDocument> {
    let issuer: IotaDID = IotaDID::parse(credential.issuer.url().as_str()).map_err(|error| {
      Error::IsolatedValidationError(crate::credential::ValidationError::SignerUrl {
        signer_ctx: crate::credential::SignerContext::Issuer,
        source: error.into(),
      })
    })?;
    self.resolve(&issuer).await
  }

  /// Fetches all DID Documents of [`Credential`] issuers contained in a [`Presentation`].
  /// Issuer documents are returned in arbitrary order.
  ///
  /// # Errors
  ///
  /// Errors if any issuer URL is not a valid [`IotaDID`] or DID resolution fails.
  pub async fn resolve_presentation_issuers<U, V: Serialize>(
    &self,
    presentation: &Presentation<U, V>,
  ) -> Result<Vec<ResolvedIotaDocument>> {
    // Extract unique issuers.
    let issuers: HashSet<IotaDID> = presentation
      .verifiable_credential
      .iter()
      .map(|credential| IotaDID::parse(credential.issuer.url().as_str()))
      .map(|url_result| {
        url_result.map_err(|error| {
          Error::IsolatedValidationError(crate::credential::ValidationError::SignerUrl {
            signer_ctx: crate::credential::SignerContext::Issuer,
            source: error.into(),
          })
        })
      })
      .collect::<Result<_>>()?;

    // Resolve issuers concurrently.
    futures::future::try_join_all(issuers.iter().map(|issuer| self.resolve(issuer)).collect::<Vec<_>>()).await
  }

  /// Fetches the DID Document of the holder of a [`Presentation`].
  ///
  /// # Errors
  ///
  /// Errors if the holder URL is missing, is not a valid [`IotaDID`], or DID resolution fails.
  pub async fn resolve_presentation_holder<U, V>(
    &self,
    presentation: &Presentation<U, V>,
  ) -> Result<ResolvedIotaDocument> {
    let holder_url: &Url = presentation.holder.as_ref().ok_or(Error::IsolatedValidationError(
      crate::credential::ValidationError::MissingPresentationHolder,
    ))?;
    let holder: IotaDID = IotaDID::parse(holder_url.as_str()).map_err(|error| {
      Error::IsolatedValidationError(crate::credential::ValidationError::SignerUrl {
        signer_ctx: crate::credential::SignerContext::Holder,
        source: error.into(),
      })
    })?;
    self.resolve(&holder).await
  }

  /// Verifies a [`Presentation`].
  ///
  /// # Important
  /// See [`PresentationValidator::validate`](PresentationValidator::validate()) for information about which properties
  /// get validated and what is expected of the optional arguments `holder` and `issuer`.
  ///
  /// # Resolution
  /// The DID Documents for the `holder` and `issuers` are optionally resolved if not given.
  /// If you already have up-to-date versions of these DID Documents, you may want
  /// to use [`PresentationValidator::validate`].
  /// See also [`Resolver::resolve_presentation_issuers`] and [`Resolver::resolve_presentation_holder`].
  ///
  /// # Errors
  /// Errors from resolving the holder and issuer DID Documents, if not provided, will be returned immediately.
  /// Otherwise, errors from validating the presentation and its credentials will be returned
  /// according to the `fail_fast` parameter.
  pub async fn verify_presentation<U: Serialize, V: Serialize>(
    &self,
    presentation: &Presentation<U, V>,
    options: &PresentationValidationOptions,
    fail_fast: FailFast,
    holder: Option<&ResolvedIotaDocument>,
    issuers: Option<&[ResolvedIotaDocument]>,
  ) -> Result<()> {
    match (holder, issuers) {
      (Some(holder), Some(issuers)) => {
        PresentationValidator::validate(presentation, holder, issuers, options, fail_fast)
      }
      (Some(holder), None) => {
        let issuers: Vec<ResolvedIotaDocument> = self.resolve_presentation_issuers(presentation).await?;
        PresentationValidator::validate(presentation, holder, &issuers, options, fail_fast)
      }
      (None, Some(issuers)) => {
        let holder: ResolvedIotaDocument = self.resolve_presentation_holder(presentation).await?;
        PresentationValidator::validate(presentation, &holder, issuers, options, fail_fast)
      }
      (None, None) => {
        let (holder, issuers): (ResolvedIotaDocument, Vec<ResolvedIotaDocument>) = futures::future::try_join(
          self.resolve_presentation_holder(presentation),
          self.resolve_presentation_issuers(presentation),
        )
        .await?;
        PresentationValidator::validate(presentation, &holder, &issuers, options, fail_fast)
      }
    }
    .map_err(Into::into)
  }
}

/// Builder for configuring [`Clients`][Client] when constructing a [`Resolver`].
#[derive(Default)]
pub struct ResolverBuilder<C = Arc<Client>>
where
  C: SharedPtr<Client>,
{
  clients: HashMap<NetworkName, ClientOrBuilder<C>>,
}

#[allow(clippy::large_enum_variant)]
enum ClientOrBuilder<C> {
  Client(C),
  Builder(ClientBuilder),
}

impl<C> ResolverBuilder<C>
where
  C: SharedPtr<Client>,
{
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
  pub fn client(mut self, client: C) -> Self {
    self
      .clients
      .insert(client.deref().network.name(), ClientOrBuilder::Client(client));
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
  pub async fn build(self) -> Result<Resolver<C>> {
    let mut client_map: HashMap<NetworkName, C> = HashMap::new();
    for (network_name, client_or_builder) in self.clients {
      let client: C = match client_or_builder {
        ClientOrBuilder::Client(client) => client,
        ClientOrBuilder::Builder(builder) => C::from(builder.build().await?),
      };
      client_map.insert(network_name, client);
    }

    Ok(Resolver { client_map })
  }
}

#[async_trait::async_trait(?Send)]
impl TangleResolve for Resolver {
  async fn resolve(&self, did: &IotaDID) -> Result<ResolvedIotaDocument> {
    self.resolve(did).await
  }
}
