// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
pub(crate) fn random_temporary_path() -> String {
  let mut file = std::env::temp_dir();
  file.push("test_strongholds");
  file.push(random_string());
  file.set_extension("stronghold");
  file.to_str().unwrap().to_owned()
}

#[cfg(test)]
pub(crate) fn random_password() -> String {
  random_string()
}

#[cfg(test)]
pub(crate) fn random_did() -> identity_iota_core::did::IotaDID {
  identity_iota_core::did::IotaDID::new(&random_bytes()).unwrap()
}

#[cfg(test)]
pub(crate) fn random_key_location() -> crate::types::KeyLocation {
  let mut thread_rng: rand::rngs::ThreadRng = rand::thread_rng();
  let fragment: String = random_string();
  let public_key: [u8; 32] = rand::Rng::gen(&mut thread_rng);

  crate::types::KeyLocation::new(identity_core::crypto::KeyType::Ed25519, fragment, &public_key)
}

pub(crate) fn random_string() -> String {
  random_bytes().into_iter().map(char::from).collect::<String>()
}

pub(crate) fn random_bytes() -> [u8; 32] {
  let mut dest: [u8; 32] = Default::default();
  getrandom::getrandom(&mut dest).unwrap();
  dest
}
