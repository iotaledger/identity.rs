// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::KeyType;
use identity_did::verification::MethodType;

pub const fn key_to_method(type_: KeyType) -> MethodType {
  match type_ {
    KeyType::Ed25519 => MethodType::Ed25519VerificationKey2018,
  }
}
