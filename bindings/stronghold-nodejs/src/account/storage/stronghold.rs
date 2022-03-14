// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_account_storage::storage::Storage;
use identity_account_storage::storage::Stronghold;
use identity_core::crypto::PrivateKey;
use identity_core::crypto::PublicKey;
use identity_core::utils::decode_b58;
use identity_core::utils::encode_b58;
use napi::bindgen_prelude::Error;
use napi::Result;
use napi_derive::napi;

use crate::account::NapiChainState;
use crate::account::NapiIdentityState;
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

  /// Sets the account password.
  #[napi]
  pub async fn set_password(&self, password: Vec<u32>) -> Result<()> {
    let password: [u8; 32] = password
      .into_iter()
      .filter_map(|n| u8::try_from(n).ok())
      .collect::<Vec<u8>>()
      .try_into()
      .map_err(|_| Error::from_reason("Invalid password type. Expected [u8; 32]".to_owned()))?;
    self.0.set_password(password).await.napi_result()
  }

  /// Write any unsaved changes to disk.
  #[napi]
  pub async fn flush_changes(&self) -> Result<()> {
    self.0.flush_changes().await.napi_result()
  }

  /// Creates a new keypair at the specified `location`
  #[napi]
  pub async fn key_new(&self, did: &NapiDID, location: &NapiKeyLocation) -> Result<String> {
    let public_key: PublicKey = self.0.key_new(&did.0, &location.0).await.napi_result()?;
    Ok(encode_b58(&public_key))
  }

  /// Inserts a private key at the specified `location`.
  #[napi]
  pub async fn key_insert(&self, did: &NapiDID, location: &NapiKeyLocation, private_key: String) -> Result<String> {
    let private_key: PrivateKey = decode_b58(&private_key).napi_result()?.into();
    let public_key: PublicKey = self
      .0
      .key_insert(&did.0, &location.0, private_key)
      .await
      .napi_result()?;
    Ok(encode_b58(&public_key))
  }

  /// Retrieves the public key at the specified `location`.
  #[napi]
  pub async fn key_get(&self, did: &NapiDID, location: &NapiKeyLocation) -> Result<String> {
    let public_key: PublicKey = self.0.key_get(&did.0, &location.0).await.napi_result()?;
    Ok(encode_b58(&public_key))
  }

  /// Deletes the keypair specified by `location`.
  #[napi]
  pub async fn key_del(&self, did: &NapiDID, location: &NapiKeyLocation) -> Result<()> {
    self.0.key_del(&did.0, &location.0).await.napi_result()
  }

  /// Signs `data` with the private key at the specified `location`.
  #[napi]
  pub async fn key_sign(&self, did: &NapiDID, location: &NapiKeyLocation, data: Vec<u32>) -> Result<NapiSignature> {
    let data_u8: Vec<u8> = data.iter().filter_map(|n| u8::try_from(*n).ok()).collect();
    if data_u8.len() != data.len() {
      return Err(Error::from_reason(String::from("Invalid data type. Expected Vec<u8>")));
    }
    self
      .0
      .key_sign(&did.0, &location.0, data_u8)
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
  pub async fn chain_state(&self, did: &NapiDID) -> Result<Option<NapiChainState>> {
    self
      .0
      .chain_state(&did.0)
      .await
      .napi_result()
      .map(|opt_chain_state| opt_chain_state.map(|chain_state| chain_state.into()))
  }

  /// Set the chain state of the identity specified by `did`.
  #[napi]
  pub async fn set_chain_state(&self, did: &NapiDID, chain_state: &NapiChainState) -> Result<()> {
    self.0.set_chain_state(&did.0, &chain_state.0).await.napi_result()
  }

  /// Returns the state of the identity specified by `did`.
  #[napi]
  pub async fn state(&self, did: &NapiDID) -> Result<Option<NapiIdentityState>> {
    self
      .0
      .state(&did.0)
      .await
      .napi_result()
      .map(|opt_id_state| opt_id_state.map(|id_state| id_state.into()))
  }

  /// Sets a new state for the identity specified by `did`.
  #[napi]
  pub async fn set_state(&self, did: &NapiDID, state: &NapiIdentityState) -> Result<()> {
    self.0.set_state(&did.0, &state.0).await.napi_result()
  }

  /// Removes the keys and any state for the identity specified by `did`.
  #[napi]
  pub async fn purge(&self, did: &NapiDID) -> Result<()> {
    self.0.purge(&did.0).await.napi_result()
  }
}
