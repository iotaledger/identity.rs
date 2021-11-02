// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::tangle::compression_brotli::compress_brotli;
use crate::tangle::compression_brotli::decompress_brotli;
use crate::Error;

#[derive(Copy, Clone)]
pub(crate) enum MessageEncodingVersion {
  JsonBrotli = 1,
}

static CURRENT_ENCODING_VERSION: MessageEncodingVersion = MessageEncodingVersion::JsonBrotli;

/// Adds the current encoding flag at the beginning of arbitrary data.
pub(crate) fn add_encoding_version_flag(mut data: Vec<u8>) -> Vec<u8> {
  let encoding_version = CURRENT_ENCODING_VERSION as u8;

  data.splice(0..0, [encoding_version].iter().cloned());
  data
}

/// Decompresses a message depending on the encoding flag.
pub(crate) fn get_decompressed_message_data<T: AsRef<[u8]>>(encoding_flag: &u8, data: T) -> Result<String, Error> {
  return if *encoding_flag == MessageEncodingVersion::JsonBrotli as u8 {
    decompress_brotli(data.as_ref())
  } else {
    Err(Error::InvalidMessageFlags)
  };
}

/// Compresses a message depending on current encoding version.
pub(crate) fn compress_message<T: AsRef<[u8]>>(message: T) -> Result<Vec<u8>, Error> {
  match CURRENT_ENCODING_VERSION {
    MessageEncodingVersion::JsonBrotli => compress_brotli(message),
  }
}

#[test]
fn test_add_version_flag() {
  let message: Vec<u8> = vec![10, 4, 5, 5];
  let message_with_flag = add_encoding_version_flag(message);
  assert_eq!(message_with_flag, [CURRENT_ENCODING_VERSION as u8, 10, 4, 5, 5])
}
