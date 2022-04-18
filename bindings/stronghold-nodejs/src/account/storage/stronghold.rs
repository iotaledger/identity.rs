// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_account_storage::storage::Storage;
use identity_account_storage::storage::Stronghold;
use identity_account_storage::types::EncryptedData;
use identity_account_storage::types::KeyLocation;
use identity_core::crypto::PrivateKey;
use identity_core::crypto::PublicKey;
use identity_iota_core::did::IotaDID;
use identity_iota_core::tangle::NetworkName;
use napi::bindgen_prelude::Error;
use napi::Result;
use napi_derive::napi;

use crate::account::identity::NapiDidLocation;
use crate::account::types::NapiEncryptedData;
use crate::account::types::NapiEncryptionAlgorithm;
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
      Stronghold::new(&snapshot, password, dropsave).await.napi_result()?,
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

  /// Creates a new identity for the given `network`.
  ///
  /// - Uses the given Ed25519 `private_key` or generates a new key if it's `None`.
  /// - Returns an error if the DID already exists.
  /// - Adds the newly created DID to a list which can be accessed via `did_list`.
  ///
  /// Returns the generated DID and the location at which the key was stored.
  #[napi]
  pub async fn did_create(
    &self,
    network: String,
    fragment: String,
    private_key: Option<Vec<u32>>,
  ) -> Result<NapiDidLocation> {
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

    Ok(NapiDidLocation::from((did, location)))
  }

  /// Removes the keys and any other state for the given `did`.
  ///
  /// This operation is idempotent: it does not fail if the given `did` does not (or no longer) exist.
  ///
  /// Returns `true` if the did and its associated data was removed, `false` if nothing was done.
  #[napi]
  pub async fn did_purge(&self, did: &NapiDID) -> Result<bool> {
    self.0.did_purge(&did.0).await.napi_result()
  }

  /// Returns `true` if `did` exists in the list of stored DIDs.
  #[napi]
  pub async fn did_exists(&self, did: &NapiDID) -> Result<bool> {
    self.0.did_exists(&did.0).await.napi_result()
  }

  /// Returns the list of stored DIDs.
  #[napi]
  pub async fn did_list(&self) -> Result<Vec<NapiDID>> {
    Ok(
      self
        .0
        .did_list()
        .await
        .napi_result()?
        .into_iter()
        .map(NapiDID)
        .collect(),
    )
  }

  /// Generates a new key for the given `did` with the given `key_type` and `fragment` identifier
  /// and returns the location of the newly generated key.
  #[napi]
  pub async fn key_generate(&self, did: &NapiDID, key_type: NapiKeyType, fragment: String) -> Result<NapiKeyLocation> {
    let location: KeyLocation = self
      .0
      .key_generate(&did.0, key_type.into(), fragment.as_ref())
      .await
      .napi_result()?;

    Ok(NapiKeyLocation(location))
  }

  /// Inserts a private key at the specified `location`.
  ///
  /// If a key at `location` exists, it is overwritten.
  #[napi]
  pub async fn key_insert(&self, did: &NapiDID, location: &NapiKeyLocation, private_key: Vec<u32>) -> Result<()> {
    let private_key: PrivateKey = private_key.try_into_bytes()?.into();
    self.0.key_insert(&did.0, &location.0, private_key).await.napi_result()
  }

  /// Retrieves the public key from `location`.
  #[napi]
  pub async fn key_public(&self, did: &NapiDID, location: &NapiKeyLocation) -> Result<Vec<u32>> {
    let public_key: PublicKey = self.0.key_public(&did.0, &location.0).await.napi_result()?;
    let public_key: Vec<u8> = public_key.as_ref().to_vec();
    Ok(public_key.into_iter().map(u32::from).collect())
  }

  /// Deletes the key at `location`.
  ///
  /// This operation is idempotent: it does not fail if the key does not exist.
  ///
  /// Returns `true` if it removed the key, `false` if nothing was done.
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

  /// Returns `true` if a key exists at the specified `location`.
  #[napi]
  pub async fn key_exists(&self, did: &NapiDID, location: &NapiKeyLocation) -> Result<bool> {
    self.0.key_exists(&did.0, &location.0).await.napi_result()
  }

  /// Encrypts the given `data` with the specified `algorithm`
  ///
  /// Diffie-Helman key exchange will be performed in case an [`KeyType::X25519`] is given.
  #[napi]
  pub async fn encrypt_data(
    &self,
    did: &NapiDID,
    data: Vec<u32>,
    associated_data: Vec<u32>,
    algorithm: &NapiEncryptionAlgorithm,
    private_key: &NapiKeyLocation,
    public_key: Option<Vec<u32>>,
  ) -> Result<NapiEncryptedData> {
    let public_key: Option<Vec<u8>> = public_key.map(|key| key.try_into_bytes()).transpose()?;
    let data: Vec<u8> = data.try_into_bytes()?;
    let associated_data: Vec<u8> = associated_data.try_into_bytes()?;
    let encrypted_data: EncryptedData = self
      .0
      .encrypt_data(
        &did.0,
        data,
        associated_data,
        &algorithm.0,
        &private_key.0,
        public_key.map(|key| key.into()),
      )
      .await
      .napi_result()?;
    Ok(NapiEncryptedData(encrypted_data))
  }

  /// Decrypts the given `data` with the specified `algorithm`
  ///
  /// Diffie-Helman key exchange will be performed in case an [`KeyType::X25519`] is given.
  #[napi]
  pub async fn decrypt_data(
    &self,
    did: &NapiDID,
    data: &NapiEncryptedData,
    algorithm: &NapiEncryptionAlgorithm,
    private_key: &NapiKeyLocation,
    public_key: Option<Vec<u32>>,
  ) -> Result<Vec<u32>> {
    let public_key: Option<Vec<u8>> = public_key.map(|key| key.try_into_bytes()).transpose()?;
    let data: Vec<u8> = self
      .0
      .decrypt_data(
        &did.0,
        data.0.clone(),
        &algorithm.0,
        &private_key.0,
        public_key.map(|key| key.into()),
      )
      .await
      .napi_result()?;
    Ok(data.into_iter().map(u32::from).collect())
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

  /// Returns the document of the identity specified by `did`.
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

  /// Persists any unsaved changes.
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
