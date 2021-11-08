// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::error::Error::CompressionError;
use crate::tangle::compression_brotli::compress_brotli;
use crate::tangle::compression_brotli::decompress_brotli;
use crate::Error;

#[derive(Debug, Copy, Clone)]
pub(crate) enum DIDMessageEncoding {
  Json = 0,
  JsonBrotli = 1,
}

/// Decompresses a message depending on the encoding flag.
pub(crate) fn decompress_message(encoding_flag: &u8, data: &[u8]) -> Result<Vec<u8>, Error> {
  if *encoding_flag == DIDMessageEncoding::JsonBrotli as u8 {
    decompress_brotli(data)
  } else if *encoding_flag == DIDMessageEncoding::Json as u8 {
    Ok(data.to_vec()) //todo prevent copying the slice.
  } else {
    Err(Error::InvalidMessageFlags)
  }
}

/// Compresses a message depending on current encoding version.
pub(crate) fn compress_message<T: AsRef<[u8]>>(message: T, encoding: DIDMessageEncoding) -> Result<Vec<u8>, Error> {
  match encoding {
    DIDMessageEncoding::JsonBrotli => compress_brotli(message),
    DIDMessageEncoding::Json => Err(CompressionError),
  }
}
