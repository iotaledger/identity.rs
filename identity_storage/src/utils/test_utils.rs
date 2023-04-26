// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;

use iota_client::secret::stronghold::StrongholdSecretManager;
use rand::distributions::DistString;

pub(crate) fn create_stronghold_secret_manager() -> StrongholdSecretManager {
  let mut file = std::env::temp_dir();
  file.push("test_strongholds");
  file.push(rand::distributions::Alphanumeric.sample_string(&mut rand::thread_rng(), 32));
  file.set_extension("stronghold");

  StrongholdSecretManager::builder()
    .password("secure_password")
    .build(&file)
    .unwrap()
}

pub(crate) fn create_temp_file() -> PathBuf {
  let mut file = std::env::temp_dir();
  file.push("test_strongholds");
  file.push(rand::distributions::Alphanumeric.sample_string(&mut rand::thread_rng(), 32));
  file.set_extension("stronghold");
  file
}
