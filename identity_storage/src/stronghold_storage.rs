// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Wrapper around [`StrongholdSecretManager`](StrongholdSecretManager).

use iota_sdk::client::secret::stronghold::StrongholdSecretManager;
use iota_sdk::client::secret::SecretManager;
use iota_stronghold::Stronghold;
use std::sync::Arc;
use tokio::sync::MutexGuard;

/// Wrapper around a [`StrongholdSecretManager`] that implements the [`KeyIdStorage`](crate::KeyIdStorage)
/// and [`JwkStorage`](crate::JwkStorage) interfaces.
#[derive(Clone, Debug)]
pub struct StrongholdStorage(Arc<SecretManager>);

impl StrongholdStorage {
  /// Creates a new [`StrongholdStorage`].
  pub fn new(stronghold_secret_manager: StrongholdSecretManager) -> Self {
    Self(Arc::new(SecretManager::Stronghold(stronghold_secret_manager)))
  }

  /// Shared reference to the inner [`SecretManager`].
  pub fn as_secret_manager(&self) -> Arc<SecretManager> {
    self.0.clone()
  }

  /// Acquire lock of the inner [`Stronghold`].
  pub(crate) async fn get_stronghold(&self) -> MutexGuard<'_, Stronghold> {
    match *self.0 {
      SecretManager::Stronghold(ref stronghold) => stronghold.inner().await,
      _ => unreachable!("secret manager can be only constrcuted from stronghold"),
    }
  }
}
