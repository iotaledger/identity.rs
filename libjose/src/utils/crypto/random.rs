// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::utils::rand;

use crate::error::Result;
use crate::lib::*;

pub fn random_bytes(size: usize) -> Result<Vec<u8>> {
  let mut bytes: Vec<u8> = vec![0; size];

  rand::fill(&mut bytes)?;

  Ok(bytes)
}
