// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::secret::stronghold::StrongholdSecretManager;

pub(crate) fn create_stronghold_secret_manager() -> StrongholdSecretManager {
  use rand::distributions::DistString;

  let mut file = std::env::temp_dir();
  file.push("test_strongholds");
  file.push(rand::distributions::Alphanumeric.sample_string(&mut rand::thread_rng(), 32));
  file.set_extension("stronghold");

  StrongholdSecretManager::builder()
    .password("secure_password")
    .build(&file)
    .unwrap()
}
