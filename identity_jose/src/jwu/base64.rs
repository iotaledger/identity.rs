// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::from_slice;
use serde_json::to_vec;

use crate::error::Result;

pub fn encode_b64(data: impl AsRef<[u8]>) -> String {
  base64::encode_config(data.as_ref(), base64::URL_SAFE_NO_PAD)
}

pub fn decode_b64(data: impl AsRef<[u8]>) -> Result<Vec<u8>> {
  base64::decode_config(data.as_ref(), base64::URL_SAFE_NO_PAD).map_err(Into::into)
}

pub fn encode_b64_json<T>(data: &T) -> Result<String>
where
  T: Serialize,
{
  to_vec(data).map(encode_b64).map_err(Into::into)
}

pub fn decode_b64_json<T>(data: impl AsRef<[u8]>) -> Result<T>
where
  T: DeserializeOwned,
{
  decode_b64(data).and_then(|data| from_slice(&data).map_err(Into::into))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn smoke() {
    assert!(decode_b64(encode_b64(b"libjose")).is_ok());
  }
}
