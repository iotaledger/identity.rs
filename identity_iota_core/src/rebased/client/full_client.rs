// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::Deref;

use crate::iota_interaction_adapter::IdentityMoveCallsAdapter;
use crate::rebased::migration::CreateIdentity;
use crate::rebased::transaction_builder::Transaction;
use crate::rebased::transaction_builder::TransactionBuilder;
use crate::IotaDID;
use crate::IotaDocument;
use crate::StateMetadataDocument;
use crate::StateMetadataEncoding;
use async_trait::async_trait;
use identity_iota_interaction::move_types::language_storage::StructTag;
use identity_iota_interaction::rpc_types::IotaObjectData;
use identity_iota_interaction::rpc_types::IotaObjectDataFilter;
use identity_iota_interaction::rpc_types::IotaObjectResponseQuery;
use identity_iota_interaction::rpc_types::IotaTransactionBlockEffects;
use identity_iota_interaction::types::base_types::IotaAddress;
use identity_iota_interaction::types::base_types::ObjectRef;
use identity_iota_interaction::types::crypto::PublicKey;
use identity_iota_interaction::types::transaction::ProgrammableTransaction;
use identity_iota_interaction::IdentityMoveCalls as _;
use identity_verification::jwk::Jwk;
use secret_storage::Signer;
use serde::de::DeserializeOwned;
use tokio::sync::OnceCell;

use crate::rebased::assets::AuthenticatedAssetBuilder;
use crate::rebased::migration::Identity;
use crate::rebased::migration::IdentityBuilder;
use crate::rebased::Error;
use identity_iota_interaction::IotaClientTrait;
use identity_iota_interaction::IotaKeySignature;
use identity_iota_interaction::MoveType;
use identity_iota_interaction::OptionalSync;

use super::get_object_id_from_did;
use super::IdentityClientReadOnly;

/// Mirrored types from identity_storage::KeyId
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct KeyId(String);

impl KeyId {
  /// Creates a new key identifier from a string.
  pub fn new(id: impl Into<String>) -> Self {
    Self(id.into())
  }

  /// Returns string representation of the key id.
  pub fn as_str(&self) -> &str {
    &self.0
  }
}

impl std::fmt::Display for KeyId {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.0)
  }
}

impl From<KeyId> for String {
  fn from(value: KeyId) -> Self {
    value.0
  }
}

/// A client for interacting with the IOTA network.
#[derive(Clone)]
pub struct IdentityClient<S> {
  /// [`IdentityClientReadOnly`] instance, used for read-only operations.
  read_client: IdentityClientReadOnly,
  /// The public key of the client.
  public_key: PublicKey,
  /// The signer of the client.
  signer: S,
}

impl<S> Deref for IdentityClient<S> {
  type Target = IdentityClientReadOnly;
  fn deref(&self) -> &Self::Target {
    &self.read_client
  }
}

impl<S> IdentityClient<S>
where
  S: Signer<IotaKeySignature>,
{
  /// Create a new [`IdentityClient`].
  pub async fn new(client: IdentityClientReadOnly, signer: S) -> Result<Self, Error> {
    let public_key = signer
      .public_key()
      .await
      .map_err(|e| Error::InvalidKey(e.to_string()))?;

    Ok(Self {
      public_key,
      read_client: client,
      signer,
    })
  }
}

impl<S> IdentityClient<S> {
  /// Returns the bytes of the sender's public key.
  pub fn sender_public_key(&self) -> &PublicKey {
    &self.public_key
  }

  /// Returns this [`IdentityClient`]'s sender address.
  #[inline(always)]
  pub fn sender_address(&self) -> IotaAddress {
    IotaAddress::from(&self.public_key)
  }

  /// Returns a reference to this [`IdentityClient`]'s [`Signer`].
  pub fn signer(&self) -> &S {
    &self.signer
  }

  /// Returns a new [`IdentityBuilder`] in order to build a new [`crate::rebased::migration::OnChainIdentity`].
  pub fn create_identity(&self, iota_document: IotaDocument) -> IdentityBuilder {
    IdentityBuilder::new(iota_document)
  }

  /// Returns a new [`IdentityBuilder`] in order to build a new [`crate::rebased::migration::OnChainIdentity`].
  pub fn create_authenticated_asset<T>(&self, content: T) -> AuthenticatedAssetBuilder<T>
  where
    T: MoveType + DeserializeOwned + Send + Sync + PartialEq,
  {
    AuthenticatedAssetBuilder::new(content)
  }

  /// Query the objects owned by the address wrapped by this client to find the object of type `tag`
  /// and that satisfies `predicate`.
  pub async fn find_owned_ref<P>(&self, tag: StructTag, predicate: P) -> Result<Option<ObjectRef>, Error>
  where
    P: Fn(&IotaObjectData) -> bool,
  {
    let filter = IotaObjectResponseQuery::new_with_filter(IotaObjectDataFilter::StructType(tag));

    let mut cursor = None;
    loop {
      let mut page = self
        .read_api()
        .get_owned_objects(self.sender_address(), Some(filter.clone()), cursor, None)
        .await?;
      let obj_ref = std::mem::take(&mut page.data)
        .into_iter()
        .filter_map(|res| res.data)
        .find(|obj| predicate(obj))
        .map(|obj_data| obj_data.object_ref());
      cursor = page.next_cursor;

      if obj_ref.is_some() {
        return Ok(obj_ref);
      }
      if !page.has_next_page {
        break;
      }
    }

    Ok(None)
  }
}

impl<S> IdentityClient<S>
where
  S: Signer<IotaKeySignature> + OptionalSync,
{
  /// Returns a [PublishDidDocument] transaction wrapped by a [TransactionBuilder].
  pub fn publish_did_document(&self, document: IotaDocument) -> TransactionBuilder<PublishDidDocument> {
    TransactionBuilder::new(PublishDidDocument::new(document, self.sender_address()))
  }

  // TODO: define what happens for (legacy|migrated|new) documents
  /// Updates a DID Document.
  pub async fn publish_did_document_update(
    &self,
    document: IotaDocument,
    gas_budget: u64,
  ) -> Result<IotaDocument, Error> {
    let mut oci =
      if let Identity::FullFledged(value) = self.get_identity(get_object_id_from_did(document.id())?).await? {
        value
      } else {
        return Err(Error::Identity("only new identities can be updated".to_string()));
      };

    let controller_token = oci.get_controller_token(self).await?.ok_or_else(|| {
      Error::Identity(format!(
        "address {} has no control over Identity {}",
        self.sender_address(),
        oci.id()
      ))
    })?;

    oci
      .update_did_document(document.clone(), &controller_token)
      .finish(self)
      .await?
      .with_gas_budget(gas_budget)
      .build_and_execute(self)
      .await?;

    Ok(document)
  }

  /// Deactivates a DID document.
  pub async fn deactivate_did_output(&self, did: &IotaDID, gas_budget: u64) -> Result<(), Error> {
    let mut oci = if let Identity::FullFledged(value) = self.get_identity(get_object_id_from_did(did)?).await? {
      value
    } else {
      return Err(Error::Identity("only new identities can be deactivated".to_string()));
    };

    let controller_token = oci.get_controller_token(self).await?.ok_or_else(|| {
      Error::Identity(format!(
        "address {} has no control over Identity {}",
        self.sender_address(),
        oci.id()
      ))
    })?;

    oci
      .deactivate_did(&controller_token)
      .finish(self)
      .await?
      .with_gas_budget(gas_budget)
      .build_and_execute(self)
      .await?;

    Ok(())
  }
}

/// Utility function that returns the key's bytes of a JWK encoded public ed25519 key.
pub fn get_sender_public_key(sender_public_jwk: &Jwk) -> Result<Vec<u8>, Error> {
  let public_key_base_64 = &sender_public_jwk
    .try_okp_params()
    .map_err(|err| Error::InvalidKey(format!("key not of type `Okp`; {err}")))?
    .x;

  identity_jose::jwu::decode_b64(public_key_base_64)
    .map_err(|err| Error::InvalidKey(format!("could not decode base64 public key; {err}")))
}

/// Publishes a new DID Document on-chain. An [`crate::rebased::migration::OnChainIdentity`] will be created to contain
/// the provided document.
#[derive(Debug, Clone)]
pub struct PublishDidDocument {
  did_document: IotaDocument,
  controller: IotaAddress,
  cached_ptb: OnceCell<ProgrammableTransaction>,
}

impl PublishDidDocument {
  /// Creates a new [PublishDidDocument] transaction.
  pub fn new(did_document: IotaDocument, controller: IotaAddress) -> Self {
    Self {
      did_document,
      controller,
      cached_ptb: OnceCell::new(),
    }
  }

  async fn make_ptb(&self, client: &IdentityClientReadOnly) -> Result<ProgrammableTransaction, Error> {
    let did_doc = StateMetadataDocument::from(self.did_document.clone())
      .pack(StateMetadataEncoding::Json)
      .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))?;
    let programmable_tx_bcs = IdentityMoveCallsAdapter::new_with_controllers(
      Some(&did_doc),
      [(self.controller, 1, false)],
      1,
      client.package_id(),
    )
    .await?;
    Ok(bcs::from_bytes(&programmable_tx_bcs)?)
  }
}

#[cfg_attr(not(feature = "send-sync"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync", async_trait)]
impl Transaction for PublishDidDocument {
  type Output = IotaDocument;

  async fn build_programmable_transaction(
    &self,
    client: &IdentityClientReadOnly,
  ) -> Result<ProgrammableTransaction, Error> {
    self.cached_ptb.get_or_try_init(|| self.make_ptb(client)).await.cloned()
  }

  async fn apply(
    self,
    effects: IotaTransactionBlockEffects,
    client: &IdentityClientReadOnly,
  ) -> (Result<Self::Output, Error>, IotaTransactionBlockEffects) {
    let tx = {
      let builder = IdentityBuilder::new(self.did_document)
        .threshold(1)
        .controller(self.controller, 1);
      CreateIdentity::new(builder)
    };

    let (application_result, remaining_effects) = tx.apply(effects, client).await;
    (application_result.map(IotaDocument::from), remaining_effects)
  }
}
