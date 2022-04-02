// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::KeyLocation;

use identity_core::crypto::KeyType;
use identity_iota_core::did::IotaDID;
use rand::Rng;

pub fn random_temporary_path() -> String {
  let mut file = std::env::temp_dir();
  file.push("test_strongholds");
  file.push(random_string(32));
  file.set_extension("stronghold");
  file.to_str().unwrap().to_owned()
}

pub fn random_password() -> String {
  random_string(32)
}

pub fn random_did() -> IotaDID {
  let public_key: [u8; 32] = rand::thread_rng().gen();
  IotaDID::new(&public_key).unwrap()
}

pub fn random_key_location() -> KeyLocation {
  let mut thread_rng: rand::rngs::ThreadRng = rand::thread_rng();
  let fragment: String = random_string(32);
  let public_key: [u8; 32] = rand::Rng::gen(&mut thread_rng);

  KeyLocation::new(KeyType::Ed25519, fragment, &public_key)
}

pub fn random_string(len: usize) -> String {
  rand::thread_rng()
    .sample_iter(rand::distributions::Alphanumeric)
    .take(len)
    .map(char::from)
    .collect::<String>()
}

// pub fn random_bytes(len: usize) -> [u8; 32] {
//   rand::thread_rng().gen()
// }
