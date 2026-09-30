// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod number;

use iota_sdk::types::ObjectId;
pub(crate) use number::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Bag {
  pub id: ObjectId,
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  pub size: u64,
}

impl Default for Bag {
  fn default() -> Self {
    Self {
      id: ObjectId::ZERO,
      size: 0,
    }
  }
}
