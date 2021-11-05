// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::error::Error;
use crate::error::Error::CompressionError;
use std::io::prelude::*;

const BUFFER_SIZE: usize = 4096;
const QUALITY: u32 = 5; // compression level
const WINDOWS_SIZE: u32 = 22;

pub(crate) fn compress_brotli<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>, Error> {
  let mut result = Vec::new();
  let mut compressor = brotli::CompressorReader::new(input.as_ref(), BUFFER_SIZE, QUALITY, WINDOWS_SIZE);
  compressor
    .read_to_end(&mut result)
    .map_err(|_| Error::CompressionError)?;
  Ok(result)
}

pub(crate) fn decompress_brotli<T: AsRef<[u8]> + ?Sized>(input: &T) -> Result<Vec<u8>, Error> {
  let mut decompressor = brotli::Decompressor::new(input.as_ref(), BUFFER_SIZE);
  let mut buf = Vec::new();
  decompressor.read_to_end(&mut buf).map_err(|_| CompressionError)?;
  Ok(buf)
}

#[cfg(test)]
mod test {
  use crate::did::IotaDocument;
  use crate::tangle::compression_brotli::compress_brotli;
  use crate::tangle::compression_brotli::decompress_brotli;
  use identity_core::convert::ToJson;
  use identity_core::crypto::KeyPair;

  #[test]
  fn test_brotli() {
    let keypair: KeyPair = KeyPair::new_ed25519().unwrap();
    let mut document: IotaDocument = IotaDocument::new(&keypair).unwrap();
    document
      .sign_self(keypair.private(), &document.default_signing_method().unwrap().id())
      .unwrap();

    let data = document.to_json().unwrap();
    let compressed = compress_brotli(data.as_str()).unwrap();
    let decompressed = decompress_brotli(&compressed).unwrap();

    assert_eq!(decompressed, data.as_bytes());
  }
}
