// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// Wrapper around `SecretManager` that implements the storage interfaces.
use std::sync::Arc;

use iota_sdk::client::secret::{stronghold::StrongholdSecretManager, SecretManager};
use iota_stronghold::Stronghold;
use tokio::sync::{Mutex, MutexGuard};

/// Wrapper around `SecretManager` that implements the storage interfaces.
#[derive(Clone)]
pub struct SecretManagerWrapper {
  secret_manager: Arc<Mutex<SecretManager>>,
  stronghold: Arc<Mutex<Stronghold>>,
}

impl SecretManagerWrapper {
  /// Creates a new [`SecretManagerWrapper`].
  pub async fn new(stronghold_secret_manager: StrongholdSecretManager) -> Self {
    let stronghold = (*stronghold_secret_manager.inner().await).clone();
    Self {
      secret_manager: Arc::new(Mutex::new(SecretManager::Stronghold(stronghold_secret_manager))),
      stronghold: Arc::new(Mutex::new(stronghold)),
    }
  }

  /// Acquire lock of the inner [`SecretManager`].
  pub async fn inner(&self) -> MutexGuard<'_, SecretManager> {
    self.secret_manager.lock().await
  }

  /// Acquire lock of the inner [`Stronghold`].
  pub async fn get_stronghold(&self) -> MutexGuard<'_, Stronghold> {
    self.stronghold.lock().await
  }
}
