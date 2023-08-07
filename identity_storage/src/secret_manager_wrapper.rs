// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use iota_sdk::client::secret::stronghold::StrongholdSecretManager;
use iota_sdk::client::secret::SecretManager;
use iota_stronghold::Stronghold;
use tokio::sync::MutexGuard;

/// Wrapper around `SecretManager` that implements the storage interfaces.
#[derive(Clone)]
pub struct SecretManagerWrapper(Arc<SecretManager>);

impl SecretManagerWrapper {
  /// Creates a new [`SecretManagerWrapper`].
  pub fn new(stronghold_secret_manager: StrongholdSecretManager) -> Self {
    Self(Arc::new(SecretManager::Stronghold(stronghold_secret_manager)))
  }

  /// Shared reference to the inner [`SecretManager`].
  pub fn inner(&self) -> Arc<SecretManager> {
    self.0.clone()
  }

  ///Acquire lock of the inner [`Stronghold`].
  pub async fn get_stronghold(&self) -> MutexGuard<'_, Stronghold> {
    match *self.0 {
      SecretManager::Stronghold(ref stronghold) => stronghold.inner().await,
      _ => unreachable!("secret manager can be only constrcuted from stronghold"),
    }
  }
}
