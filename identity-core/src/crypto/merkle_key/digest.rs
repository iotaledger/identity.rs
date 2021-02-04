// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use sha2::Sha256;

use crate::crypto::merkle_key::Digest;

impl Digest for Sha256 {
  const TAG: u8 = 0;
}
