// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::errors::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use crate::endpoint::Endpoint;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestMessage {
  pub name: Endpoint,
  pub data: Vec<u8>,
}

impl RequestMessage {
  pub fn new(name: impl AsRef<str>, data: Vec<u8>) -> Result<Self> {
    Ok(Self {
      name: Endpoint::new(name)?,
      data,
    })
  }
}

pub type ResponseMessage = Vec<u8>;
