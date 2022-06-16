// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use rand::distributions::DistString;
use rand::rngs::OsRng;
use rand::Rng;

pub(crate) fn random_temporary_path() -> String {
  let mut file = std::env::temp_dir();
  file.push("test_strongholds");
  file.push(random_string());
  file.set_extension("stronghold");
  file.to_str().unwrap().to_owned()
}

pub(crate) fn random_did() -> identity_iota_core::did::IotaDID {
  identity_iota_core::did::IotaDID::new(&random_bytes()).unwrap()
}

pub(crate) fn random_key_location() -> crate::types::KeyLocation {
  let fragment: String = random_string();
  let public_key: [u8; 32] = OsRng.gen();

  crate::types::KeyLocation::new(identity_core::crypto::KeyType::Ed25519, fragment, &public_key)
}

pub(crate) fn random_bytes() -> [u8; 32] {
  OsRng.gen()
}

pub(crate) fn random_string() -> String {
  rand::distributions::Alphanumeric.sample_string(&mut OsRng, 32)
}
