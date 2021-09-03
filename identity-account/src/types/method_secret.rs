// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::{KeyCollection, SecretKey};

#[derive(Clone, Debug)]
pub enum MethodSecret {
  Ed25519(SecretKey),
  MerkleKeyCollection(KeyCollection),
}
