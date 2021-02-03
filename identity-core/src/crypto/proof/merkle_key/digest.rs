// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use sha2::Sha256;

use crate::crypto::MerkleKeyDigest;

impl MerkleKeyDigest for Sha256 {
  const TAG: u8 = 0;
}
