// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use rand::distributions::DistString;
use rand::rngs::OsRng;

pub(crate) fn random_string() -> String {
  rand::distributions::Alphanumeric.sample_string(&mut OsRng, 32)
}
