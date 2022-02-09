// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use futures::executor;
use identity::account::DIDLease;
use identity::account::Storage;
use identity::account::Stronghold;
use identity::core::decode_b58;
use identity::core::encode_b58;
use identity::crypto::PublicKey;
use napi::bindgen_prelude::Error;
use napi::Result;

use crate::account::JsChainState;
use crate::account::JsDIDLease;
use crate::account::JsGeneration;
use crate::account::JsIdentityState;
use crate::account::JsKeyLocation;
use crate::account::JsSignature;
use crate::did::JsDID;
use crate::error::NapiResult;

#[napi(js_name = Stronghold)]
#[derive(Debug)]
pub struct JsStronghold(pub(crate) Stronghold);

#[napi]
impl JsStronghold {
  /// Creates an instance of `Stronghold`.
  /// Prefer to use Stronghold.new(snapshot, password, dropsave)
  #[napi(factory)]
  pub fn create(snapshot: String, password: String, dropsave: Option<bool>) -> JsStronghold {
    let js_stronghold_result = executor::block_on(JsStronghold::new(snapshot, password, dropsave));
    match js_stronghold_result {
      Ok(stronghold) => stronghold,
      Err(e) => panic!("Unable to create Stronghold: {:?}", e),
    }
  }

  /// Creates an instance of `Stronghold`.
  #[napi]
  pub async fn new(
    snapshot: String,
    password: String,
    dropsave: Option<bool>,
  ) -> Result<JsStronghold> {
    let stronghold: Stronghold = Stronghold::new(&snapshot, &*password, dropsave)
      .await
      .napi_result()?;
    Ok(JsStronghold(stronghold))
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
    let password: Vec<u8> = password
      .into_iter()
      .filter_map(|n| u8::try_from(n).ok())
      .collect();
    self
      .0
      .set_password(password.try_into().map_err(|_| {
        Error::from_reason(String::from("Invalid password type. Expected [u8; 32]"))
      })?)
      .await
      .napi_result()
  }

  /// Write any unsaved changes to disk.
  #[napi]
  pub async fn flush_changes(&self) -> Result<()> {
    self.0.flush_changes().await.napi_result()
  }

  /// Attempt to obtain the exclusive permission to modify the given `did`.
  /// The caller is expected to make no more modifications after the lease has been dropped.
  /// Returns an IdentityInUse error if already leased.
  #[napi]
  pub async fn lease_did(&self, did: &JsDID) -> Result<JsDIDLease> {
    let did_lease: DIDLease = self.0.lease_did(&did.0).await.napi_result()?;
    Ok(did_lease.into())
  }

  /// Creates a new keypair at the specified `location`
  #[napi]
  pub async fn key_new(&self, did: &JsDID, location: &JsKeyLocation) -> Result<String> {
    let public_key: PublicKey = self.0.key_new(&did.0, &location.0).await.napi_result()?;
    Ok(encode_b58(&public_key))
  }

  /// Inserts a private key at the specified `location`.
  #[napi]
  pub async fn key_insert(
    &self,
    did: &JsDID,
    location: &JsKeyLocation,
    private_key: String,
  ) -> Result<String> {
    let public_key: PublicKey = self
      .0
      .key_insert(
        &did.0,
        &location.0,
        decode_b58(&private_key).napi_result()?.into(),
      )
      .await
      .napi_result()?;
    Ok(encode_b58(&public_key))
  }

  /// Retrieves the public key at the specified `location`.
  #[napi]
  pub async fn key_get(&self, did: &JsDID, location: &JsKeyLocation) -> Result<String> {
    let public_key: PublicKey = self.0.key_get(&did.0, &location.0).await.napi_result()?;
    Ok(encode_b58(&public_key))
  }

  /// Deletes the keypair specified by `location`.
  #[napi]
  pub async fn key_del(&self, did: &JsDID, location: &JsKeyLocation) -> Result<()> {
    self.0.key_del(&did.0, &location.0).await.napi_result()
  }

  /// Signs `data` with the private key at the specified `location`.
  #[napi]
  pub async fn key_sign(
    &self,
    did: &JsDID,
    location: &JsKeyLocation,
    data: Vec<u32>,
  ) -> Result<JsSignature> {
    let mut data_u8: Vec<u8> = Vec::new();
    for v in data {
      match u8::try_from(v) {
        Ok(v) => data_u8.push(v),
        Err(_) => {
          return Err(Error::from_reason(String::from(
            "Invalid data type. Expected Vec<u8>",
          )))
        }
      }
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
  pub async fn key_exists(&self, did: &JsDID, location: &JsKeyLocation) -> Result<bool> {
    self.0.key_exists(&did.0, &location.0).await.napi_result()
  }

  /// Returns the last generation that has been published to the tangle for the given `did`.
  #[napi]
  pub async fn published_generation(&self, did: &JsDID) -> Result<Option<JsGeneration>> {
    self
      .0
      .published_generation(&did.0)
      .await
      .napi_result()
      .map(|opt_generation| opt_generation.map(|generation| generation.into()))
  }

  /// Sets the last generation that has been published to the tangle for the given `did`.
  #[napi]
  pub async fn set_published_generation(&self, did: &JsDID, index: &JsGeneration) -> Result<()> {
    self
      .0
      .set_published_generation(&did.0, index.0)
      .await
      .napi_result()
  }

  /// Returns the chain state of the identity specified by `did`.
  #[napi]
  pub async fn chain_state(&self, did: &JsDID) -> Result<Option<JsChainState>> {
    self
      .0
      .chain_state(&did.0)
      .await
      .napi_result()
      .map(|opt_chain_state| opt_chain_state.map(|chain_state| chain_state.into()))
  }

  /// Set the chain state of the identity specified by `did`.
  #[napi]
  pub async fn set_chain_state(&self, did: &JsDID, chain_state: &JsChainState) -> Result<()> {
    self
      .0
      .set_chain_state(&did.0, &chain_state.0)
      .await
      .napi_result()
  }

  /// Returns the state of the identity specified by `did`.
  #[napi]
  pub async fn state(&self, did: &JsDID) -> Result<Option<JsIdentityState>> {
    self
      .0
      .state(&did.0)
      .await
      .napi_result()
      .map(|opt_id_state| opt_id_state.map(|id_state| id_state.into()))
  }

  /// Sets a new state for the identity specified by `did`.
  #[napi]
  pub async fn set_state(&self, did: &JsDID, state: &JsIdentityState) -> Result<()> {
    self.0.set_state(&did.0, &state.0).await.napi_result()
  }

  /// Removes the keys and any state for the identity specified by `did`.
  #[napi]
  pub async fn purge(&self, did: &JsDID) -> Result<()> {
    self.0.purge(&did.0).await.napi_result()
  }
}
