// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::actor::Endpoint;
use crate::actor::RequestMode;

use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestMessage {
  pub endpoint: Endpoint,
  pub request_mode: RequestMode,
  pub data: Vec<u8>,
}

impl RequestMessage {
  pub fn new(endpoint: Endpoint, request_mode: RequestMode, data: Vec<u8>) -> Self {
    Self {
      endpoint,
      request_mode,
      data,
    }
  }

  pub fn from_bytes(bytes: &[u8]) -> std::io::Result<Self> {
    serde_json::from_slice::<'_, Self>(bytes)
      .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err.to_string()))
  }

  pub fn to_bytes(&self) -> std::io::Result<Vec<u8>> {
    serde_json::to_vec(self).map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err.to_string()))
  }
}

#[derive(Debug)]
pub struct ResponseMessage(pub Vec<u8>);
