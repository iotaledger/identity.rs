use std::hash::Hasher;

use identity_verification::VerificationMethod;
use seahash::SeaHasher;

// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MethodDigest {
  pub version: u8,
  pub value: u64,
}

impl MethodDigest {
  pub fn new(verification_method: &VerificationMethod) -> identity_verification::Result<Self> {
    let mut hasher: SeaHasher = SeaHasher::new();
    let fragment = verification_method
      .id()
      .fragment()
      .ok_or(identity_verification::Error::MissingIdFragment)?;

    let method_data: Vec<u8> = verification_method.data().try_decode()?;
    hasher.write(fragment.as_bytes());
    hasher.write(&method_data);
    let key_hash: u64 = hasher.finish();
    Ok(Self {
      version: 0,
      value: key_hash,
    })
  }
}
