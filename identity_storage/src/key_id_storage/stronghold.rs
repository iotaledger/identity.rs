// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::KeyId;
use crate::KeyIdStorage;
use crate::KeyIdStorageError;
use crate::KeyIdStorageErrorKind;
use crate::KeyIdStorageResult;
use crate::MethodDigest;
use crate::CLIENT_PATH;
use async_trait::async_trait;
use iota_client::secret::stronghold::StrongholdSecretManager;
use iota_stronghold::Client;
use iota_stronghold::ClientError;
use iota_stronghold::Stronghold;
use tokio::sync::MutexGuard;

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
impl KeyIdStorage for StrongholdSecretManager {
  async fn insert_key_id(&self, method_digest: MethodDigest, key_id: KeyId) -> KeyIdStorageResult<()> {
    let stronghold = self.get_stronghold().await;
    let client = get_client(stronghold)?;
    let store = client.store();
    let key = method_digest.pack();
    let key_exists = store
      .contains_key(key.as_ref())
      .map_err(|err| KeyIdStorageError::new(crate::KeyIdStorageErrorKind::Unspecified).with_source(err))?;

    if key_exists {
      return Err(KeyIdStorageError::new(KeyIdStorageErrorKind::KeyIdAlreadyExists));
    }
    let value = key_id.as_str().as_bytes();
    client
      .store()
      .insert(method_digest.pack(), value.to_vec(), None)
      .map_err(|err| KeyIdStorageError::new(crate::KeyIdStorageErrorKind::Unspecified).with_source(err))?;
    Ok(())
  }

  async fn get_key_id(&self, method_digest: &MethodDigest) -> KeyIdStorageResult<KeyId> {
    let stronghold = self.get_stronghold().await;
    let store = get_client(stronghold)?.store();
    let key: Vec<u8> = method_digest.pack();
    let value: Vec<u8> = store
      .get(key.as_ref())
      .map_err(|err| KeyIdStorageError::new(KeyIdStorageErrorKind::Unspecified).with_source(err))?
      .ok_or(KeyIdStorageError::new(KeyIdStorageErrorKind::KeyIdNotFound))?;

    let key_id: KeyId = KeyId::new(
      String::from_utf8(value)
        .map_err(|err| KeyIdStorageError::new(KeyIdStorageErrorKind::Unspecified).with_source(err))?,
    );
    Ok(key_id)
  }

  async fn delete_key_id(&self, method_digest: &MethodDigest) -> KeyIdStorageResult<()> {
    let stronghold = self.get_stronghold().await;
    let store = get_client(stronghold)?.store();
    let key: Vec<u8> = method_digest.pack();

    let _ = store
      .delete(key.as_ref())
      .map_err(|err| KeyIdStorageError::new(KeyIdStorageErrorKind::Unspecified).with_source(err))?
      .ok_or(KeyIdStorageError::new(KeyIdStorageErrorKind::KeyIdNotFound))?;

    Ok(())
  }
}

pub fn get_client(stronghold: MutexGuard<'_, Stronghold>) -> KeyIdStorageResult<Client> {
  if let Ok(client) = stronghold.get_client(CLIENT_PATH) {
    return Ok(client);
  }
  let client = stronghold.get_client(CLIENT_PATH);
  match client {
    Ok(client) => Ok(client),
    Err(ClientError::ClientDataNotPresent) => load_or_create_client(stronghold),
    Err(err) => Err(KeyIdStorageError::new(KeyIdStorageErrorKind::Unspecified).with_source(err)),
  }
}
fn load_or_create_client(stronghold: MutexGuard<'_, Stronghold>) -> KeyIdStorageResult<Client> {
  match stronghold.load_client(CLIENT_PATH) {
    Ok(client) => Ok(client),
    Err(ClientError::ClientDataNotPresent) => stronghold
      .create_client(CLIENT_PATH)
      .map_err(|err| KeyIdStorageError::new(KeyIdStorageErrorKind::Unspecified).with_source(err)),
    Err(err) => Err(KeyIdStorageError::new(KeyIdStorageErrorKind::Unspecified).with_source(err)),
  }
}
