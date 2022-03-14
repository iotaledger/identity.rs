// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::keys::pbkdf::PBKDF2_HMAC_SHA512;

const PBKDF_ITER: usize = 100;
const PBKDF_SALT: &[u8] = b"identity.rs";

pub type EncryptionKey = [u8; 32];

pub fn derive_encryption_key(password: &str) -> EncryptionKey {
  let mut output: EncryptionKey = Default::default();

  // safe to unwrap (rounds > 0)
  PBKDF2_HMAC_SHA512(password.as_bytes(), PBKDF_SALT, PBKDF_ITER, &mut output).unwrap();

  output
}
