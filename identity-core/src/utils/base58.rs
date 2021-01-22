// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::error::{Error, Result};

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
