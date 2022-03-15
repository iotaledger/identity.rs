// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::KeyCollection;
use identity_core::crypto::PrivateKey;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub enum MethodSecret {
  Ed25519(PrivateKey),
  MerkleKeyCollection(KeyCollection),
}
