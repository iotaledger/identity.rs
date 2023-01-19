// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_account_storage::storage::Storage;
use identity_account_storage::storage::Stronghold;
use identity_account_storage::types::EncryptedData;
use identity_account_storage::types::KeyLocation;
use identity_core::crypto::PrivateKey;
use identity_core::crypto::PublicKey;
use identity_did::CoreDID;
use identity_iota_core_legacy::tangle::NetworkName;
use napi::bindgen_prelude::Error;
use napi::Result;
use napi_derive::napi;

use crate::error::NapiResult;
use crate::types::NapiCekAlgorithm;
use crate::types::NapiCoreDid;
use crate::types::NapiDIDType;
use crate::types::NapiDidLocation;
use crate::types::NapiEncryptedData;
use crate::types::NapiEncryptionAlgorithm;
use crate::types::NapiKeyLocation;
use crate::types::NapiKeyType;
use crate::types::NapiSignature;

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
    did_type: NapiDIDType,
    network: String,
    fragment: String,
    private_key: Option<Vec<u32>>,
  ) -> Result<NapiDidLocation> {
    let network: NetworkName = NetworkName::try_from(network).napi_result()?;
    let private_key: Option<PrivateKey> = match private_key {
      Some(private_key) => Some(private_key.try_into_bytes()?.into()),
      None => None,
    };

    let (did, location): (CoreDID, KeyLocation) = self
      .0
      .did_create(did_type.into(), network, fragment.as_ref(), private_key)
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
  pub async fn did_purge(&self, did: &NapiCoreDid) -> Result<bool> {
    self.0.did_purge(&did.0).await.napi_result()
  }

  /// Returns `true` if `did` exists in the list of stored DIDs.
  #[napi]
  pub async fn did_exists(&self, did: &NapiCoreDid) -> Result<bool> {
    self.0.did_exists(&did.0).await.napi_result()
  }

  /// Returns the list of stored DIDs.
  #[napi]
  pub async fn did_list(&self) -> Result<Vec<NapiCoreDid>> {
    Ok(
      self
        .0
        .did_list()
        .await
        .napi_result()?
        .into_iter()
        .map(NapiCoreDid)
        .collect(),
    )
  }

  /// Generates a new key for the given `did` with the given `key_type` and `fragment` identifier
  /// and returns the location of the newly generated key.
  #[napi]
  pub async fn key_generate(
    &self,
    did: &NapiCoreDid,
    key_type: NapiKeyType,
    fragment: String,
  ) -> Result<NapiKeyLocation> {
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
  pub async fn key_insert(&self, did: &NapiCoreDid, location: &NapiKeyLocation, private_key: Vec<u32>) -> Result<()> {
    let private_key: PrivateKey = private_key.try_into_bytes()?.into();
    self.0.key_insert(&did.0, &location.0, private_key).await.napi_result()
  }

  /// Retrieves the public key from `location`.
  #[napi]
  pub async fn key_public(&self, did: &NapiCoreDid, location: &NapiKeyLocation) -> Result<Vec<u32>> {
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
  pub async fn key_delete(&self, did: &NapiCoreDid, location: &NapiKeyLocation) -> Result<bool> {
    self.0.key_delete(&did.0, &location.0).await.napi_result()
  }

  /// Signs `data` with the private key at the specified `location`.
  #[napi]
  pub async fn key_sign(&self, did: &NapiCoreDid, location: &NapiKeyLocation, data: Vec<u32>) -> Result<NapiSignature> {
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
  pub async fn key_exists(&self, did: &NapiCoreDid, location: &NapiKeyLocation) -> Result<bool> {
    self.0.key_exists(&did.0, &location.0).await.napi_result()
  }

  /// Encrypts the given `plaintext` with the specified `encryption_algorithm` and `cek_algorithm`.
  ///
  /// Returns an [`EncryptedData`] instance.
  #[napi]
  pub async fn data_encrypt(
    &self,
    did: &NapiCoreDid,
    plaintext: Vec<u32>,
    associated_data: Vec<u32>,
    encryption_algorithm: &NapiEncryptionAlgorithm,
    cek_algorithm: &NapiCekAlgorithm,
    public_key: Vec<u32>,
  ) -> Result<NapiEncryptedData> {
    let public_key: Vec<u8> = public_key.try_into_bytes()?;
    let plaintext: Vec<u8> = plaintext.try_into_bytes()?;
    let associated_data: Vec<u8> = associated_data.try_into_bytes()?;
    let encrypted_data: EncryptedData = self
      .0
      .data_encrypt(
        &did.0,
        plaintext,
        associated_data,
        &encryption_algorithm.0,
        &cek_algorithm.0,
        public_key.into(),
      )
      .await
      .napi_result()?;
    Ok(NapiEncryptedData(encrypted_data))
  }

  /// Decrypts the given `data` with the specified `encryption_algorithm` and `cek_algorithm`.
  ///
  /// Returns the decrypted text.
  #[napi]
  pub async fn data_decrypt(
    &self,
    did: &NapiCoreDid,
    data: &NapiEncryptedData,
    encryption_algorithm: &NapiEncryptionAlgorithm,
    cek_algorithm: &NapiCekAlgorithm,
    private_key: &NapiKeyLocation,
  ) -> Result<Vec<u32>> {
    let data: Vec<u8> = self
      .0
      .data_decrypt(
        &did.0,
        data.0.clone(),
        &encryption_algorithm.0,
        &cek_algorithm.0,
        &private_key.0,
      )
      .await
      .napi_result()?;
    Ok(data.into_iter().map(u32::from).collect())
  }

  /// Returns the blob stored by the identity specified by `did`.
  #[napi]
  pub async fn blob_get(&self, did: &NapiCoreDid) -> Result<Option<Vec<u32>>> {
    self
      .0
      .blob_get(&did.0)
      .await
      .napi_result()
      .map(|opt_value| opt_value.map(|value| value.into_iter().map(u32::from).collect()))
  }

  /// Stores an arbitrary blob for the identity specified by `did`.
  #[napi]
  pub async fn blob_set(&self, did: &NapiCoreDid, blob: Vec<u32>) -> Result<()> {
    let blob: Vec<u8> = blob.try_into_bytes()?;
    self.0.blob_set(&did.0, blob).await.napi_result()
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
