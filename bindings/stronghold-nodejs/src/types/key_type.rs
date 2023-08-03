// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::KeyType;
use napi::bindgen_prelude::ToNapiValue;
use napi::bindgen_prelude::FromNapiValue;
use napi_derive::napi;

#[napi]
pub enum NapiKeyType {
  Ed25519,
  X25519,
}

impl From<NapiKeyType> for KeyType {
  fn from(other: NapiKeyType) -> Self {
    match other {
      NapiKeyType::Ed25519 => KeyType::Ed25519,
      NapiKeyType::X25519 => KeyType::X25519,
    }
  }
}
