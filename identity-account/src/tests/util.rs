// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use identity_account_storage::storage::MemStore;
use identity_account_storage::storage::Storage;
use identity_account_storage::storage::Stronghold;
use identity_iota::tangle::ClientBuilder;
use identity_iota_core::tangle::Network;
use rand::distributions::DistString;
use rand::rngs::OsRng;

use crate::account::AccountConfig;
use crate::account::AccountSetup;

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

pub(super) fn temporary_random_path() -> String {
  let mut file = std::env::temp_dir();
  file.push("test_strongholds");
  file.push(rand::distributions::Alphanumeric.sample_string(&mut OsRng, 32));
  file.set_extension("stronghold");
  file.to_str().unwrap().to_owned()
}

pub(super) async fn storages() -> [Arc<dyn Storage>; 2] {
  [
    Arc::new(MemStore::new()),
    Arc::new(
      Stronghold::new(&temporary_random_path(), "password".to_owned(), Some(false))
        .await
        .unwrap(),
    ),
  ]
}
