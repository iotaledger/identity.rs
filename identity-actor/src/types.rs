// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestMessage {
  pub name: String,
  pub data: Vec<u8>,
}

impl RequestMessage {
  pub fn new<S: Into<String>>(name: S, data: Vec<u8>) -> Self {
    Self {
      name: name.into(),
      data,
    }
  }
}

pub type ResponseMessage = Vec<u8>;
