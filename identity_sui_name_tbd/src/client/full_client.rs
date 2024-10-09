// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::Deref;

use async_trait::async_trait;
use fastcrypto::ed25519::Ed25519PublicKey;
use fastcrypto::traits::ToFromBytes;
use identity_iota_core::IotaDID;
use identity_iota_core::IotaDocument;
use identity_iota_core::StateMetadataDocument;
use identity_verification::jwk::Jwk;
use crate::iota_sdk_abstraction::IotaClientTrait;
use crate::iota_sdk_abstraction::rpc_types::IotaObjectData;
use crate::iota_sdk_abstraction::rpc_types::IotaObjectDataFilter;
use crate::iota_sdk_abstraction::rpc_types::IotaObjectResponseQuery;
use crate::iota_sdk_abstraction::IotaTransactionBlockResponseT;
use crate::iota_sdk_abstraction::types::base_types::IotaAddress;
use crate::iota_sdk_abstraction::types::base_types::ObjectRef;
use crate::iota_sdk_abstraction::ProgrammableTransactionBcs;
use crate::iota_sdk_abstraction::move_types::language_storage::StructTag;
use secret_storage::Signer;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::assets::AuthenticatedAssetBuilder;
use crate::migration::Identity;
use crate::migration::IdentityBuilder;
use crate::transaction::Transaction as TransactionT;
use crate::client::IotaKeySignature;
use crate::utils::MoveType;
use crate::Error;

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

#[derive(Clone)]
pub struct IdentityClient<S> {
  read_client: IdentityClientReadOnly,
  address: IotaAddress,
  public_key: Vec<u8>,
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
  S: Signer<IotaKeySignature> + Sync,
{
  pub async fn new(client: IdentityClientReadOnly, signer: S) -> Result<Self, Error> {
    let public_key = signer
      .public_key()
      .await
      .map_err(|e| Error::InvalidKey(e.to_string()))?;
    let address = convert_to_address(&public_key)?;

    Ok(Self {
      public_key,
      address,
      read_client: client,
      signer,
    })
  }

  pub(crate) async fn execute_transaction(
    &self,
    tx_bcs: ProgrammableTransactionBcs,
    gas_budget: Option<u64>,
  ) -> Result<Box<dyn IotaTransactionBlockResponseT<Error=Error>>, Error> {
    self.read_client.execute_transaction(
      self.sender_address(),
      self.sender_public_key(),
      tx_bcs,
      gas_budget,
      self.signer()
    ).await
  }
}

impl<S> IdentityClient<S> {
  pub fn sender_public_key(&self) -> &[u8] {
    &self.public_key
  }

  pub fn sender_address(&self) -> IotaAddress {
    self.address
  }

  pub fn signer(&self) -> &S {
    &self.signer
  }

  /// Creates a new onchain Identity.
  pub fn create_identity(&self, iota_document: &[u8]) -> IdentityBuilder {
    IdentityBuilder::new(iota_document)
  }

  pub fn create_authenticated_asset<T>(&self, content: T) -> AuthenticatedAssetBuilder<T>
  where
    T: MoveType + Serialize + DeserializeOwned,
  {
    AuthenticatedAssetBuilder::new(content)
  }

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
  S: Signer<IotaKeySignature> + Sync,
{
  pub fn publish_did_document(&self, document: IotaDocument) -> PublishDidTx {
    PublishDidTx(document)
  }

  // TODO: define what happens for (legacy|migrated|new) documents
  pub async fn publish_did_document_update(
    &self,
    document: IotaDocument,
    gas_budget: u64,
  ) -> Result<IotaDocument, Error> {
    let Identity::FullFledged(oci) = self.get_identity(get_object_id_from_did(document.id())?).await? else {
      return Err(Error::Identity("only new identities can be updated".to_string()));
    };

    oci
      .update_did_document(document.clone())
      .finish()
      .execute_with_gas(gas_budget, self)
      .await?;

    Ok(document)
  }

  pub async fn deactivate_did_output(&self, did: &IotaDID, gas_budget: u64) -> Result<(), Error> {
    let oci = if let Identity::FullFledged(value) = self.get_identity(get_object_id_from_did(did)?).await? {
      value
    } else {
      return Err(Error::Identity("only new identities can be deactivated".to_string()));
    };

    oci.deactivate_did().finish().execute_with_gas(gas_budget, self).await?;

    Ok(())
  }
}

pub fn get_sender_public_key(sender_public_jwk: &Jwk) -> Result<Vec<u8>, Error> {
  let public_key_base_64 = &sender_public_jwk
    .try_okp_params()
    .map_err(|err| Error::InvalidKey(format!("key not of type `Okp`; {err}")))?
    .x;

  identity_jose::jwu::decode_b64(public_key_base_64)
    .map_err(|err| Error::InvalidKey(format!("could not decode base64 public key; {err}")))
}

pub fn convert_to_address(sender_public_key: &[u8]) -> Result<IotaAddress, Error> {
  let public_key = Ed25519PublicKey::from_bytes(sender_public_key)
    .map_err(|err| Error::InvalidKey(format!("could not parse public key to Ed25519 public key; {err}")))?;

  Ok(IotaAddress::from(&public_key))
}

#[derive(Debug)]
pub struct PublishDidTx(IotaDocument);

#[async_trait]
impl TransactionT for PublishDidTx {
  type Output = IotaDocument;
  async fn execute_with_opt_gas<S>(
    self,
    gas_budget: Option<u64>,
    client: &IdentityClient<S>,
  ) -> Result<Self::Output, Error>
  where
    S: Signer<IotaKeySignature> + Sync,
  {
    let packed = self
      .0
      .clone()
      .pack()
      .map_err(|err| Error::DidDocSerialization(format!("could not pack DID document: {err}")))?;

    let oci = client
      .create_identity(&packed)
      .finish()
      .execute_with_opt_gas(gas_budget, client)
      .await?;

    // replace placeholders in document
    let did: IotaDID = IotaDID::new(&oci.id(), client.network());
    let metadata_document: StateMetadataDocument = self.0.into();
    let document_without_placeholders = metadata_document.into_iota_document(&did).map_err(|err| {
      Error::DidDocParsingFailed(format!(
        "could not replace placeholders in published DID document {did}; {err}"
      ))
    })?;

    Ok(document_without_placeholders)
  }
}
