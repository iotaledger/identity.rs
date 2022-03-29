// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_account_storage::storage::Storage;
use identity_account_storage::storage::Stronghold;
use identity_account_storage::types::KeyLocation;
use identity_core::crypto::PrivateKey;
use identity_core::crypto::PublicKey;
use identity_iota_core::did::IotaDID;
use identity_iota_core::tangle::NetworkName;
use napi::bindgen_prelude::Error;
use napi::Result;
use napi_derive::napi;

use crate::account::identity::NapiDIDLocation;
use crate::account::types::NapiKeyType;
use crate::account::NapiChainState;
use crate::account::NapiDocument;
use crate::account::NapiKeyLocation;
use crate::account::NapiSignature;
use crate::did::NapiDID;
use crate::error::NapiResult;

#[napi]
#[derive(Debug)]
pub struct NapiStronghold(pub(crate) Stronghold);

#[napi]
impl NapiStronghold {
  /// Workaround for Napi not generating code when no factory or constructor is present, and
  /// async constructors are not possible.
  #[napi(factory)]
  pub fn _f() -> Result<NapiStronghold> {
    Err(Error::from_reason(
      "use NapiStronghold.new to instantiate instances".to_owned(),
    ))
  }

  /// Creates an instance of `Stronghold`.
  #[napi]
  pub async fn new(snapshot: String, password: String, dropsave: Option<bool>) -> Result<NapiStronghold> {
    Ok(NapiStronghold(
      Stronghold::new(&snapshot, &*password, dropsave).await.napi_result()?,
    ))
  }

  /// Returns whether save-on-drop is enabled.
  #[napi(getter)]
  pub fn dropsave(&self) -> bool {
    self.0.dropsave()
  }

  /// Set whether to save the storage changes on drop.
  /// Default: true
  #[napi(setter)]
  pub fn set_dropsave(&mut self, dropsave: bool) {
    self.0.set_dropsave(dropsave);
  }

  #[napi]
  pub async fn did_create(
    &self,
    network: String,
    fragment: String,
    private_key: Option<Vec<u32>>,
  ) -> Result<NapiDIDLocation> {
    let network: NetworkName = NetworkName::try_from(network).napi_result()?;
    let private_key: Option<PrivateKey> = match private_key {
      Some(private_key) => Some(private_key.try_into_bytes()?.into()),
      None => None,
    };

    let (did, location): (IotaDID, KeyLocation) = self
      .0
      .did_create(network, fragment.as_ref(), private_key)
      .await
      .napi_result()?;

    Ok(NapiDIDLocation::from((did, location)))
  }

  /// Removes the keys and any state for the identity specified by `did`.
  #[napi]
  pub async fn did_purge(&self, did: &NapiDID) -> Result<bool> {
    self.0.did_purge(&did.0).await.napi_result()
  }

  /// Creates a new keypair at the specified `location`
  #[napi]
  pub async fn key_generate(&self, did: &NapiDID, key_type: &NapiKeyType, fragment: String) -> Result<NapiKeyLocation> {
    let location: KeyLocation = self
      .0
      .key_generate(&did.0, key_type.0, fragment.as_ref())
      .await
      .napi_result()?;

    Ok(NapiKeyLocation(location))
  }

  /// Inserts a private key at the specified `location`.
  #[napi]
  pub async fn key_insert(&self, did: &NapiDID, location: &NapiKeyLocation, private_key: Vec<u32>) -> Result<()> {
    let private_key: PrivateKey = private_key.try_into_bytes()?.into();
    self.0.key_insert(&did.0, &location.0, private_key).await.napi_result()
  }

  /// Retrieves the public key at the specified `location`.
  #[napi]
  pub async fn key_public(&self, did: &NapiDID, location: &NapiKeyLocation) -> Result<Vec<u32>> {
    let public_key: PublicKey = self.0.key_public(&did.0, &location.0).await.napi_result()?;
    let public_key: Vec<u8> = public_key.as_ref().to_vec();
    Ok(public_key.into_iter().map(u32::from).collect())
  }

  /// Deletes the keypair specified by `location`.
  #[napi]
  pub async fn key_delete(&self, did: &NapiDID, location: &NapiKeyLocation) -> Result<bool> {
    self.0.key_delete(&did.0, &location.0).await.napi_result()
  }

  /// Signs `data` with the private key at the specified `location`.
  #[napi]
  pub async fn key_sign(&self, did: &NapiDID, location: &NapiKeyLocation, data: Vec<u32>) -> Result<NapiSignature> {
    let data: Vec<u8> = data.try_into_bytes()?;
    self
      .0
      .key_sign(&did.0, &location.0, data)
      .await
      .napi_result()
      .map(|signature| signature.into())
  }

  /// Returns `true` if a keypair exists at the specified `location`.
  #[napi]
  pub async fn key_exists(&self, did: &NapiDID, location: &NapiKeyLocation) -> Result<bool> {
    self.0.key_exists(&did.0, &location.0).await.napi_result()
  }

  /// Returns the chain state of the identity specified by `did`.
  #[napi]
  pub async fn chain_state_get(&self, did: &NapiDID) -> Result<Option<NapiChainState>> {
    self
      .0
      .chain_state_get(&did.0)
      .await
      .napi_result()
      .map(|opt_chain_state| opt_chain_state.map(|chain_state| chain_state.into()))
  }

  /// Set the chain state of the identity specified by `did`.
  #[napi]
  pub async fn chain_state_set(&self, did: &NapiDID, chain_state: &NapiChainState) -> Result<()> {
    self.0.chain_state_set(&did.0, &chain_state.0).await.napi_result()
  }

  /// Returns the state of the identity specified by `did`.
  #[napi]
  pub async fn document_get(&self, did: &NapiDID) -> Result<Option<NapiDocument>> {
    self
      .0
      .document_get(&did.0)
      .await
      .napi_result()
      .map(|opt_document| opt_document.map(|doc| doc.into()))
  }

  /// Sets a new state for the identity specified by `did`.
  #[napi]
  pub async fn document_set(&self, did: &NapiDID, state: &NapiDocument) -> Result<()> {
    self.0.document_set(&did.0, &state.0).await.napi_result()
  }

  #[napi]
  pub async fn index_has(&self, did: &NapiDID) -> Result<bool> {
    self.0.index_has(&did.0).await.napi_result()
  }

  #[napi]
  pub async fn index(&self) -> Result<Vec<NapiDID>> {
    Ok(self.0.index().await.napi_result()?.into_iter().map(NapiDID).collect())
  }

  /// Write any unsaved changes to disk.
  #[napi]
  pub async fn flush_changes(&self) -> Result<()> {
    self.0.flush_changes().await.napi_result()
  }
}

trait TryIntoBytes {
  fn try_into_bytes(self) -> Result<Vec<u8>>;
}

impl TryIntoBytes for Vec<u32> {
  fn try_into_bytes(self) -> Result<Vec<u8>> {
    self
      .into_iter()
      .map(u8::try_from)
      .collect::<std::result::Result<Vec<u8>, _>>()
      .map_err(|err| Error::from_reason(format!("invalid data type, expected Uint8Array: {}", err)))
  }
}
