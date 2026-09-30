// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::future::Future;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;

use futures::stream::FuturesUnordered;
use futures::Stream;
use futures::StreamExt as _;
use futures::TryStreamExt as _;
use identity_core::common::Url;
use identity_did::DID as _;
use iota_sdk::graphql_client::error::Error as IotaClientError;
use iota_sdk::graphql_client::query_types::ObjectFilter;
use iota_sdk::graphql_client::Client as IotaClient;
use iota_sdk::transaction_builder::TransactionBuilder;
use iota_sdk::types::Address;
use iota_sdk::types::ObjectId;
use iota_sdk::types::TransactionEffects;
use product_core::move_type::MoveType;
use product_core::network::Network;
use product_core::operation::Operation;
use product_core::operation::OperationBuilder;
use product_core::product_client::ProductClient;
use product_core::type_origin_table::TypeOriginTable;
use product_core::type_origin_table::TypeOriginTableCreationError;
use secret_storage::iota::TransactionSigner;
use tokio::sync::RwLock;

use crate::rebased::iota::package::identity_package_registry;
use crate::rebased::migration::get_identity;
use crate::rebased::migration::get_identity_impl;
use crate::rebased::migration::ControllerCap;
use crate::rebased::migration::ControllerToken;
use crate::rebased::migration::DelegationToken;
use crate::rebased::migration::Identity;
use crate::rebased::migration::IdentityBuilder;
use crate::rebased::migration::IdentityResolutionError;
use crate::rebased::migration::InsufficientControllerVotingPower;
use crate::rebased::migration::NotAController;
use crate::rebased::migration::OnChainIdentity;
use crate::rebased::proposals::ProposalResult;
use crate::rebased::Error;
use crate::IotaDID;
use crate::IotaDocument;
use crate::StateMetadataDocument;
use crate::StateMetadataEncoding;

/// A client for interacting with IOTA Identity on the IOTA network.
#[derive(Debug, Clone)]
pub struct IdentityClient {
  iota_client: IotaClient,
  network: Network,
  package: ObjectId,
  type_origin_table: Arc<TypeOriginTable>,
}

impl AsRef<IotaClient> for IdentityClient {
  fn as_ref(&self) -> &IotaClient {
    &self.iota_client
  }
}

impl Deref for IdentityClient {
  type Target = IotaClient;
  fn deref(&self) -> &Self::Target {
    &self.iota_client
  }
}

impl ProductClient for IdentityClient {
  fn network(&self) -> Network {
    self.network
  }

  fn package_id(&self) -> ObjectId {
    self.package
  }

  fn type_origin_table(&self) -> &TypeOriginTable {
    &self.type_origin_table
  }
}

impl IdentityClient {
  /// Creates a new [IdentityClient], with **no** signing capabilities, from the given [IotaClient].
  ///
  /// # Warning
  /// Passing a `custom_package_id` is **only** required when connecting to a custom IOTA network.
  ///
  /// Relying on a custom Identity package when connected to an official IOTA network is **highly
  /// discouraged** and is sure to result in compatibility issues when interacting with other official
  /// IOTA Trust Framework's products.
  ///
  /// # Examples
  /// ```
  /// # use identity_iota_core::rebased::client::IdentityClient;
  ///
  /// # #[tokio::main(flavor = "current_thread")]
  /// # async fn main() -> anyhow::Result<()> {
  /// let iota_client = iota_sdk::IotaClientBuilder::default()
  ///   .build_testnet()
  ///   .await?;
  /// // No package ID is required since we are connecting to an official IOTA network.
  /// let identity_client = IdentityClient::from_iota_client(iota_client, None).await?;
  /// # Ok(())
  /// # }
  /// ```
  pub async fn from_iota_client(
    iota_client: IotaClient,
    custom_package_id: impl Into<Option<ObjectId>>,
  ) -> Result<Self, FromIotaClientError> {
    let make_err = |kind| FromIotaClientError { kind };
    let network = get_network(&iota_client)
      .await
      .map_err(|e| make_err(FromIotaClientErrorKind::NetworkResolution(e.into())))?;
    let package_id = if network.is_custom() {
      custom_package_id
        .into()
        .ok_or(make_err(FromIotaClientErrorKind::MissingPackageId))?
    } else {
      identity_package_registry()
        .await
        .package_id(network.as_chain_id())
        .expect("package id for official networks is tracked by this library")
    };
    let type_origin_table = TypeOriginTable::new(package_id, &iota_client)
      .await
      .map(Arc::new)
      .map_err(|e| make_err(e.into()))?;

    Ok(Self {
      iota_client,
      package: package_id,
      network,
      type_origin_table,
    })
  }

  /// Sets the migration registry ID for the current network.
  /// # Notes
  /// This is only needed when automatic retrieval of MigrationRegistry's ID fails.
  pub fn set_migration_registry_id(&mut self, id: ObjectId) {
    crate::rebased::migration::set_migration_registry_id(self.network.as_chain_id(), id);
  }

  /// Queries an [`IotaDocument`] DID Document through its `did`.
  pub async fn resolve_did(&self, did: &IotaDID) -> Result<IotaDocument, Error> {
    // Make sure `did` references a DID Document on the network
    // this client is connected to.
    let did_network = did.network_str();
    let client_network = self.network.as_ref();
    if did_network != client_network && did_network != self.network.as_chain_id() {
      return Err(Error::DIDResolutionError(format!(
        "provided DID `{did}` \
        references a DID Document on network `{did_network}`, \
        but this client is connected to network `{client_network}`"
      )));
    }
    let identity = self.get_identity(did.to_object_id()).await?;
    let did_doc = identity.did_document(self.network())?;

    match identity {
      Identity::FullFledged(identity) if identity.has_deleted_did() => {
        Err(Error::DIDResolutionError(format!("could not find DID Document {did}")))
      }
      _ => Ok(did_doc),
    }
  }

  /// Resolves an [`Identity`] from its ID `object_id`.
  pub async fn get_identity(&self, object_id: ObjectId) -> Result<Identity, Error> {
    // spawn all checks
    cfg_if::cfg_if! {
      // Unfortunately the compiler runs into lifetime problems if we try to use a 'type ='
      // instead of the below ugly platform specific code
      if #[cfg(feature = "send-sync")] {
        let all_futures = FuturesUnordered::<Pin<Box<dyn Future<Output = Result<Option<Identity>, Error>> + Send>>>::new();
      } else {
        let all_futures = FuturesUnordered::<Pin<Box<dyn Future<Output = Result<Option<Identity>, Error>>>>>::new();
      }
    }
    all_futures.push(Box::pin(resolve_new(self, object_id)));
    all_futures.push(Box::pin(resolve_migrated(self, object_id)));
    all_futures.push(Box::pin(resolve_unmigrated(self, object_id)));

    all_futures
      .filter_map(|res| Box::pin(async move { res.ok().flatten() }))
      .next()
      .await
      .ok_or_else(|| Error::DIDResolutionError(format!("could not find DID document for {object_id}")))
  }

  /// Returns a stream yielding the unique DIDs the given address can access as a controller.
  /// # Notes
  /// This is a streaming version of [dids_controlled_by](Self::dids_controlled_by).
  /// # Errors
  /// This stream might return a [QueryControlledDidsError] when the underlying RPC call fails.
  /// When an error occurs, the stream might successfully yield a value if polled again, depending
  /// on the actual RPC error.
  /// [QueryControlledDidsError]'s source can be downcasted to [SDK's Error](iota_interaction::error::Error).
  /// # Example
  /// ```ignore
  /// # use std::pin::pin;
  /// # use identity_iota_core::rebased::client::IdentityClientReadOnly;
  /// # use identity_iota_core::IotaDID;
  /// # use iota_sdk::IotaClientBuilder;
  /// # use futures::{Stream, StreamExt};
  /// #
  /// # #[tokio::main]
  /// # async fn main() -> anyhow::Result<()> {
  /// # let iota_client = IotaClientBuilder::default().build_testnet().await?;
  /// # let identity_client = IdentityClientReadOnly::new(iota_client).await?;
  /// #
  /// let address = "0x666638f5118b8f894c4e60052f9bc47d6fcfb04fdb990c9afbb988848b79c475".parse()?;
  /// let mut controlled_dids = pin!(identity_client.streamed_dids_controlled_by(address));
  /// assert_eq!(
  ///   controlled_dids.next().await.unwrap()?,
  ///   IotaDID::parse(
  ///     "did:iota:testnet:0x052cfb920024f7a640dc17f7f44c6042ea0038d26972c2cff5c7ba31c82fbb08"
  ///   )?,
  /// );
  /// # Ok(())
  /// # }
  /// ```
  pub(crate) fn streamed_dids_controlled_by(
    &self,
    address: Address,
  ) -> impl Stream<Item = Result<IotaDID, QueryControlledDidsError>> + use<'_> {
    macro_rules! make_stream {
      ($cap_type: ident) => {
        self
          .objects_content_stream::<$cap_type>(
            ObjectFilter::default()
              .with_type($cap_type::move_type(self).to_string())
              .with_owner(address),
          )
          .map_ok(|cap| cap.controller_of())
      };
    }

    make_stream!(ControllerCap)
      .chain(make_stream!(DelegationToken))
      .map_ok(|id| IotaDID::from_object_id(id, self.network))
      .map_err(move |e| QueryControlledDidsError {
        address,
        source: e.into(),
      })
  }

  /// Returns the list of **all** unique DIDs the given address has access to as a controller.
  /// # Notes
  /// For a streaming version of this API see [dids_controlled_by_streamed](Self::dids_controlled_by_streamed).
  /// # Errors
  /// This method might return a [QueryControlledDidsError] when the underlying RPC call fails.
  /// [QueryControlledDidsError]'s source can be downcasted to [SDK's Error](iota_interaction::error::Error)
  /// in order to check whether calling this method again might return a successful result.
  /// # Example
  /// ```
  /// # use identity_iota_core::rebased::client::IdentityClientReadOnly;
  /// # use identity_iota_core::IotaDID;
  /// # use iota_sdk::IotaClientBuilder;
  /// #
  /// # #[tokio::main]
  /// # async fn main() -> anyhow::Result<()> {
  /// # let iota_client = IotaClientBuilder::default().build_testnet().await?;
  /// # let identity_client = IdentityClientReadOnly::new(iota_client).await?;
  /// #
  /// let address = "0x666638f5118b8f894c4e60052f9bc47d6fcfb04fdb990c9afbb988848b79c475".parse()?;
  /// let controlled_dids = identity_client.dids_controlled_by(address).await?;
  /// assert_eq!(
  ///   controlled_dids,
  ///   vec![IotaDID::parse(
  ///     "did:iota:testnet:0x052cfb920024f7a640dc17f7f44c6042ea0038d26972c2cff5c7ba31c82fbb08"
  ///   )?]
  /// );
  /// # Ok(())
  /// # }
  /// ```
  pub async fn dids_controlled_by(&self, address: Address) -> Result<Vec<IotaDID>, QueryControlledDidsError> {
    self.streamed_dids_controlled_by(address).try_collect().await
  }
}

impl IdentityClient {
  /// Returns a new [`IdentityBuilder`] in order to build a new [`crate::rebased::migration::OnChainIdentity`].
  pub fn create_identity(&self, iota_document: IotaDocument) -> IdentityBuilder {
    IdentityBuilder::new(iota_document)
  }

  /// Returns an [Operation] to publish the given DID Document on-chain.
  pub fn publish_did_document(
    &self,
    document: IotaDocument,
    signer: &impl TransactionSigner,
  ) -> OperationBuilder<PublishDidDocument> {
    OperationBuilder::new(PublishDidDocument::new(document, signer.address()))
  }

  /// Deactivates a DID document.
  pub async fn deactivate_did_output(&self, did: &IotaDID, signer: &impl TransactionSigner) -> Result<(), Error> {
    let mut oci = if let Identity::FullFledged(value) = self.get_identity(did.to_object_id()).await? {
      value
    } else {
      return Err(Error::Identity("only new identities can be deactivated".to_string()));
    };

    let controller_token = oci.get_controller_token(signer.address(), self).await?.ok_or_else(|| {
      Error::Identity(format!(
        "address {} has no control over Identity {}",
        signer.address(),
        oci.id()
      ))
    })?;

    oci
      .deactivate_did(&controller_token)
      .finish(self)
      .await?
      .execute(signer, self)
      .await
      .map_err(|e| Error::TransactionUnexpectedResponse(e.to_string()))?;

    Ok(())
  }

  /// A shorthand for
  /// [OnChainIdentity::update_did_document](crate::rebased::migration::OnChainIdentity::update_did_document)'s DID
  /// Document.
  ///
  /// This method makes the following assumptions:
  /// - The given `did_document` has already been published on-chain within an Identity.
  /// - This [IdentityClient] is a controller of the corresponding Identity with enough voting power to execute the
  ///   transaction without any other controller approval.
  pub async fn publish_did_update(
    &self,
    did_document: IotaDocument,
    signer: &impl TransactionSigner,
  ) -> Result<OperationBuilder<ShorthandDidUpdate>, MakeUpdateDidDocTxError> {
    use MakeUpdateDidDocTxError as Error;
    use MakeUpdateDidDocTxErrorKind as ErrorKind;

    let make_err = |kind| Error {
      did_document: did_document.clone(),
      kind,
    };

    let identity_id = did_document.id().to_object_id();
    let identity = get_identity_impl(self, identity_id)
      .await
      .map_err(|e| make_err(e.into()))?;

    if identity.has_deleted_did() {
      return Err(make_err(ErrorKind::DeletedIdentityDocument));
    }

    let controller_token = identity
      .get_controller_token(signer.address(), self)
      .await
      .map_err(|e| make_err(ErrorKind::RpcError(e.into())))?
      .ok_or_else(|| {
        make_err(
          NotAController {
            address: signer.address(),
            identity: did_document.id().clone(),
          }
          .into(),
        )
      })?;

    let vp = identity
      .controller_voting_power(controller_token.controller_id())
      .expect("is a controller");
    let threshold = identity.threshold();
    if vp < threshold {
      return Err(make_err(
        InsufficientControllerVotingPower {
          controller_token_id: controller_token.controller_id(),
          controller_voting_power: vp,
          required: threshold,
        }
        .into(),
      ));
    }

    Ok(OperationBuilder::new(ShorthandDidUpdate {
      identity: RwLock::new(identity),
      controller_token,
      did_document,
    }))
  }
}

/// Publishes a new DID Document on-chain. An [`OnChainIdentity`](crate::rebased::migration::OnChainIdentity)
/// will be created to contain the provided document.
#[derive(Debug, Clone)]
pub struct PublishDidDocument {
  did_document: IotaDocument,
  controller: Address,
}

impl PublishDidDocument {
  /// Creates a new [PublishDidDocument] transaction.
  pub fn new(did_document: IotaDocument, controller: Address) -> Self {
    Self {
      did_document,
      controller,
    }
  }
}

impl Operation for PublishDidDocument {
  type Client = IdentityClient;
  type Output = IotaDocument;
  type Error = Error;

  async fn to_transaction(
    &self,
    client: &IdentityClient,
    mut ptb: TransactionBuilder<IotaClient>,
  ) -> Result<TransactionBuilder<IotaClient>, Self::Error> {
    let package = client.package_id();
    let did_document_bytes = StateMetadataDocument::from(self.did_document.clone())
      .pack(StateMetadataEncoding::Json)
      .map_err(|e| Error::DidDocSerialization(e.to_string()))?;

    super::iota::move_calls::identity::new_with_controllers(
      &mut ptb,
      Some(&did_document_bytes),
      [(self.controller, 1, false)],
      1,
      package,
    );

    Ok(ptb)
  }

  async fn apply_effects(
    self,
    client: &IdentityClient,
    tx_effects: &mut TransactionEffects,
  ) -> Result<Self::Output, Self::Error> {
    if tx_effects.as_v1().status.is_failure() {
      return Err(tx_effects.as_v1().status.clone().unwrap_err().0.into());
    }

    let target_did_bytes = StateMetadataDocument::from(self.did_document)
      .pack(StateMetadataEncoding::Json)
      .map_err(|e| Error::DidDocSerialization(e.to_string()))?;

    let is_target_identity = |identity: &OnChainIdentity| -> bool {
      let did_bytes = identity
        .multicontroller()
        .controlled_value()
        .as_deref()
        .unwrap_or_default();
      target_did_bytes == did_bytes && identity.threshold() == 1
    };

    let create_identity_candidates = tx_effects
      .as_v1()
      .changed_objects
      .iter()
      .filter(|obj| {
        obj.id_operation.is_created()
          && obj
            .output_state
            .opt_object_owner()
            .is_some_and(|owner| owner.is_shared())
      })
      .map(|obj| obj.object_id);

    let mut target_identity = None;
    for id in create_identity_candidates {
      let Some(identity) = get_identity(client, id)
        .await
        .map_err(|e| Error::Identity(e.to_string()))?
      else {
        continue;
      };

      if is_target_identity(&identity) {
        target_identity = Some(identity);
      }
    }

    if let Some(identity) = target_identity {
      // tx_effects
      //   .as_mut_v1()
      //   .changed_objects
      //   .retain(|obj| obj.object_id != identity.id().to_object_id());
      Ok(identity.did_doc)
    } else {
      Err(Error::TransactionUnexpectedResponse(
        "failed to find the correct identity in this transaction's effects".to_owned(),
      ))
    }
  }
}

/// The actual Transaction type returned by [IdentityClient::publish_did_update].
#[derive(Debug)]
pub struct ShorthandDidUpdate {
  identity: RwLock<OnChainIdentity>,
  controller_token: ControllerToken,
  did_document: IotaDocument,
}

impl Operation for ShorthandDidUpdate {
  type Client = IdentityClient;
  type Error = MakeUpdateDidDocTxError;
  type Output = IotaDocument;

  async fn to_transaction(
    &self,
    client: &Self::Client,
    tx_builder: TransactionBuilder<IotaClient>,
  ) -> Result<TransactionBuilder<IotaClient>, Self::Error> {
    let mut identity = self.identity.write().await;
    identity
      .update_did_document(self.did_document.clone(), &self.controller_token)
      .finish(client)
      .await
      .map_err(|e| MakeUpdateDidDocTxError {
        did_document: self.did_document.clone(),
        kind: MakeUpdateDidDocTxErrorKind::RpcError(e.into()),
      })?
      .to_transaction(client, tx_builder)
      .await
      .map_err(|e| MakeUpdateDidDocTxError {
        did_document: self.did_document.clone(),
        kind: MakeUpdateDidDocTxErrorKind::RpcError(e.into()),
      })
  }

  async fn apply_effects(
    self,
    client: &Self::Client,
    tx_effects: &mut TransactionEffects,
  ) -> Result<Self::Output, Self::Error> {
    let mut identity = self.identity.write().await;
    let ProposalResult::Executed(_) = identity
      .update_did_document(self.did_document.clone(), &self.controller_token)
      .finish(client)
      .await
      .map_err(|e| MakeUpdateDidDocTxError {
        did_document: self.did_document.clone(),
        kind: MakeUpdateDidDocTxErrorKind::RpcError(e.into()),
      })?
      .apply_effects(client, tx_effects)
      .await
      .map_err(|e| MakeUpdateDidDocTxError {
        did_document: self.did_document.clone(),
        kind: MakeUpdateDidDocTxErrorKind::RpcError(e.into()),
      })?
    else {
      unreachable!("all precondition have been checked before creating this operation")
    };

    // Re-sync identity.
    let updated_identity = get_identity(client, identity.id())
      .await
      .map_err(|e| MakeUpdateDidDocTxError {
        did_document: self.did_document.clone(),
        kind: MakeUpdateDidDocTxErrorKind::RpcError(e.into()),
      })?
      .expect("identity exists");

    *identity = updated_identity;

    Ok(identity.did_doc.clone())
  }
}

/// [IdentityClient::publish_did_update] error.
#[derive(Debug, thiserror::Error)]
#[error("failed to prepare transaction to update DID '{}'", did_document.id())]
#[non_exhaustive]
pub struct MakeUpdateDidDocTxError {
  /// The DID document that was being published.
  pub did_document: IotaDocument,
  /// Specific type of failure for this error.
  pub kind: MakeUpdateDidDocTxErrorKind,
}

/// Types of failure for [MakeUpdateDidDocTxError].
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum MakeUpdateDidDocTxErrorKind {
  /// Node RPC failure.
  #[error(transparent)]
  RpcError(Box<dyn std::error::Error + Send + Sync>),
  /// Failed to resolve the corresponding [OnChainIdentity].
  #[error(transparent)]
  IdentityResolution(#[from] IdentityResolutionError),
  /// The invoking client is not a controller of the given DID document.
  #[error(transparent)]
  NotAController(#[from] NotAController),
  /// The DID document has been deleted and cannot be updated.
  #[error("Identity's DID Document is deleted")]
  DeletedIdentityDocument,
  /// The invoking client is a controller but doesn't have enough voting power
  /// to perform the update.
  #[error(transparent)]
  InsufficientVotingPower(#[from] InsufficientControllerVotingPower),
}

/// Error that might occur when querying an address for its controlled DIDs.
#[derive(Debug, thiserror::Error)]
#[error("failed to query the DIDs controlled by address `{address}`")]
#[non_exhaustive]
pub struct QueryControlledDidsError {
  /// The queried address.
  pub address: Address,
  source: Box<dyn std::error::Error + Send + Sync>,
}

async fn get_network(iota_client: &IotaClient) -> Result<Network, IotaClientError> {
  Ok(
    iota_client
      .chain_id()
      .await?
      .parse()
      .expect("a successful call to chain_id returns a valid Network"),
  )
}

async fn resolve_new(client: &IdentityClient, object_id: ObjectId) -> Result<Option<Identity>, Error> {
  let onchain_identity = get_identity(client, object_id).await.map_err(|err| {
    Error::DIDResolutionError(format!(
      "could not get identity document for object id {object_id}; {err}"
    ))
  })?;
  Ok(onchain_identity.map(Identity::FullFledged))
}

async fn resolve_migrated(client: &IdentityClient, object_id: ObjectId) -> Result<Option<Identity>, Error> {
  let onchain_identity = super::migration::lookup(client, object_id).await.map_err(|err| {
    Error::DIDResolutionError(format!(
      "failed to look up object_id {object_id} in migration registry; {err}"
    ))
  })?;
  let Some(mut onchain_identity) = onchain_identity else {
    return Ok(None);
  };
  let queried_did = IotaDID::from_object_id(object_id, client.network);
  let doc = onchain_identity.did_document_mut();
  let identity_did = doc.id().clone();
  // When querying a migrated identity we obtain a DID document with DID `identity_did` and the `alsoKnownAs`
  // property containing `queried_did`. Since we are resolving `queried_did`, lets replace in the document these
  // values. `queried_id` becomes the DID Document ID.
  *doc.core_document_mut().id_mut_unchecked() = queried_did.clone().into();
  // The DID Document `alsoKnownAs` property is cleaned of its `queried_did` entry,
  // which gets replaced by `identity_did`.
  doc
    .also_known_as_mut()
    .replace::<Url>(&queried_did.into_url().into(), identity_did.into_url().into());

  Ok(Some(Identity::FullFledged(onchain_identity)))
}

async fn resolve_unmigrated(client: &IdentityClient, object_id: ObjectId) -> Result<Option<Identity>, Error> {
  let unmigrated_alias = super::migration::get_alias(client, object_id)
    .await
    .map_err(|err| Error::DIDResolutionError(format!("could not query for object id {object_id}; {err}")))?;
  Ok(unmigrated_alias.map(Identity::Legacy))
}

/// The error that results from a failed attempt at creating an [IdentityClient]
/// from a given [IotaClient].
#[derive(Debug, thiserror::Error)]
#[error("failed to create an 'IdentityClient' from the given 'IotaClient'")]
#[non_exhaustive]
pub struct FromIotaClientError {
  /// Type of failure for this error.
  #[source]
  pub kind: FromIotaClientErrorKind,
}

/// Types of failure for [FromIotaClientError].
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum FromIotaClientErrorKind {
  /// A package ID is required, but was not supplied.
  #[error("an IOTA Identity package ID must be supplied when connecting to an unofficial IOTA network")]
  MissingPackageId,
  /// Network ID resolution through an RPC call failed.
  #[error("failed to resolve the network the given client is connected to")]
  NetworkResolution(#[source] Box<dyn std::error::Error + Send + Sync>),
  /// Failed to create a [TypeOriginTable] for the given package ID.
  #[error(transparent)]
  TypeOriginTable(#[from] TypeOriginTableCreationError),
}

#[cfg(test)]
mod tests {
  use crate::IotaDID;

  use super::IdentityClient;
  use iota_sdk::graphql_client::Client as IotaClient;
  use product_core::network::Network;

  #[tokio::test]
  async fn resolution_of_a_did_for_a_different_network_fails() -> anyhow::Result<()> {
    let iota_client = IotaClient::new_testnet();
    let identity_client = IdentityClient::from_iota_client(iota_client, None).await?;

    let did = IotaDID::new(&[1; 32], Network::Devnet);
    let error = identity_client.resolve_did(&did).await.unwrap_err();

    assert!(matches!(error, crate::rebased::Error::DIDResolutionError(_)));

    Ok(())
  }
}
