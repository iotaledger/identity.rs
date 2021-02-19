// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::error::Error;
use crate::error::Result;

/// Decodes the given `data` as base58-btc.
pub fn decode_b58<T>(data: &T) -> Result<Vec<u8>>
where
  T: AsRef<[u8]> + ?Sized,
{
  bs58::decode(data)
    .with_alphabet(bs58::Alphabet::BITCOIN)
    .into_vec()
    .map_err(Error::DecodeBase58)
}

/// Encodes the given `data` as base58-btc.
pub fn encode_b58<T>(data: &T) -> String
where
  T: AsRef<[u8]> + ?Sized,
{
  bs58::encode(data).with_alphabet(bs58::Alphabet::BITCOIN).into_string()
}

/// Decodes the given `data` as base16 (hex).
pub fn decode_b16<T>(data: &T) -> Result<Vec<u8>>
where
  T: AsRef<[u8]> + ?Sized,
{
  hex::decode(data).map_err(Error::DecodeBase16)
}

/// Encodes the given `data` as base16 (hex).
pub fn encode_b16<T>(data: &T) -> String
where
  T: AsRef<[u8]> + ?Sized,
{
  hex::encode(data)
}

/// Decodes the given `data` as base64.
pub fn decode_b64<T>(data: &T) -> Result<Vec<u8>>
where
  T: AsRef<[u8]> + ?Sized,
{
  base64::decode_config(data.as_ref(), base64::URL_SAFE).map_err(Error::DecodeBase64)
}

/// Encodes the given `data` as base64.
pub fn encode_b64<T>(data: &T) -> String
where
  T: AsRef<[u8]> + ?Sized,
{
  base64::encode_config(data.as_ref(), base64::URL_SAFE)
}
