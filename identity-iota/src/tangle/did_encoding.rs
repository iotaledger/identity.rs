// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::error::Error::CompressionError;
use crate::tangle::compression_brotli::compress_brotli;
use crate::tangle::compression_brotli::decompress_brotli;
use crate::Error;

#[derive(Copy, Clone)]
pub(crate) enum DIDMessageEncoding {
  Json = 0,
  JsonBrotli = 1,
}

/// Adds the current encoding flag at the beginning of arbitrary data.
pub(crate) fn add_encoding_version_flag(mut data: Vec<u8>, encoding: DIDMessageEncoding) -> Vec<u8> {
  let encoding_version = encoding as u8;

  data.splice(0..0, [encoding_version].iter().cloned());
  data
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

#[cfg(test)]
mod test {
  use crate::tangle::did_encoding::DIDMessageEncoding;

  #[test]
  fn test_add_version_flag() {
    let message: Vec<u8> = vec![10, 4, 5, 5];
    let message_with_flag = DIDMessageEncoding::add_encoding_version_flag(message, DIDMessageEncoding::JsonBrotli);
    assert_eq!(message_with_flag, [DIDMessageEncoding::JsonBrotli as u8, 10, 4, 5, 5])
  }
}
