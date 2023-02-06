// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use base64::engine::general_purpose;
use base64::Engine;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::Error;
use crate::error::Result;

/// Encode the given slice in url-safe base64.
pub fn encode_b64(data: impl AsRef<[u8]>) -> String {
  general_purpose::URL_SAFE_NO_PAD.encode(data)
}

/// Decode the given url-safe base64-encoded slice into its raw bytes.
pub fn decode_b64(data: impl AsRef<[u8]>) -> Result<Vec<u8>> {
  general_purpose::URL_SAFE_NO_PAD
    .decode(data)
    .map_err(Error::InvalidBase64)
}

/// Serialize the given data into JSON and encode the result in url-safe base64.
pub fn encode_b64_json<T>(data: &T) -> Result<String>
where
  T: Serialize,
{
  serde_json::to_vec(data).map(encode_b64).map_err(Error::InvalidJson)
}

/// Decode the given url-safe base64-encoded slice into its raw bytes and try to deserialize it into `T`.
pub fn decode_b64_json<T>(data: impl AsRef<[u8]>) -> Result<T>
where
  T: DeserializeOwned,
{
  decode_b64(data).and_then(|data| serde_json::from_slice(&data).map_err(Error::InvalidJson))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn smoke() {
    assert!(decode_b64(encode_b64(b"libjose")).is_ok());
  }
}
