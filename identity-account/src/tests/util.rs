// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use identity_account_storage::storage::MemStore;
use identity_account_storage::storage::Storage;
use identity_account_storage::storage::Stronghold;
use identity_iota::tangle::ClientBuilder;
use identity_iota_core::tangle::Network;
use rand::Rng;

use crate::account::AccountConfig;
use crate::account::AccountSetup;

// There's a bug in our stronghold wrapper that makes multiple `Stronghold`s not work concurrently,
// so we're using a static instance as a temporary workaround, until we've upgraded.
pub(super) static TEST_STRONGHOLD: once_cell::sync::Lazy<Arc<dyn Storage>> = once_cell::sync::Lazy::new(|| {
  let temp_file = temporary_random_path();
  let stronghold: Stronghold = futures::executor::block_on(Stronghold::new(&temp_file, "password", None)).unwrap();
  Arc::new(stronghold)
});

pub(super) async fn account_setup(network: Network) -> AccountSetup {
  account_setup_storage(Arc::new(MemStore::new()), network).await
}

pub(super) async fn account_setup_storage(storage: Arc<dyn Storage>, network: Network) -> AccountSetup {
  AccountSetup::new(
    storage,
    Arc::new(
      ClientBuilder::new()
        .network(network)
        .node_sync_disabled()
        .build()
        .await
        .unwrap(),
    ),
    AccountConfig::new().testmode(true),
  )
}

fn temporary_random_path() -> String {
  let mut file = std::env::temp_dir();
  file.push("test_strongholds/");
  file.push(
    rand::thread_rng()
      .sample_iter(rand::distributions::Alphanumeric)
      .take(32)
      .map(char::from)
      .collect::<String>(),
  );
  file.set_extension("stronghold");
  file.to_str().unwrap().to_owned()
}

pub(super) async fn storages() -> [Arc<dyn Storage>; 2] {
  [Arc::new(MemStore::new()), Arc::clone(&TEST_STRONGHOLD)]
}
