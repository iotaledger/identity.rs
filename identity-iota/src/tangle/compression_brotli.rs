// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::error::Error;
use crate::error::Error::CompressionError;
use std::io::prelude::*;

pub(crate) fn compress_brotli<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>, Error> {
  let mut result = Vec::new();
  let mut compressor = brotli::CompressorReader::new(input.as_ref(), 4096, 5, 22);
  compressor
    .read_to_end(&mut result)
    .map_err(|_| Error::CompressionError)?;
  Ok(result)
}

pub(crate) fn decompress_brotli<T: AsRef<[u8]> + ?Sized>(input: &T) -> Result<String, Error> {
  let mut decompressor = brotli::Decompressor::new(input.as_ref(), 4096 /* buffer size */);
  let mut s = String::new();
  decompressor.read_to_string(&mut s).map_err(|_| CompressionError)?;
  Ok(s)
}

#[cfg(test)]
mod test {
  use crate::did::IotaDocument;
  use crate::tangle::compression_brotli::compress_brotli;
  use crate::tangle::compression_brotli::decompress_brotli;
  use identity_core::convert::ToJson;
  use identity_core::crypto::KeyPair;

  #[test]
  fn test_compression_algorithm() {
    let keypair: KeyPair = KeyPair::new_ed25519().unwrap();
    let mut document: IotaDocument = IotaDocument::new(&keypair).unwrap();
    document.sign(keypair.private()).unwrap();

    let data = document.to_json().unwrap();
    let compressed = compress_brotli(data.as_str()).unwrap();
    let decompressed = decompress_brotli(&compressed).unwrap();

    assert_eq!(decompressed, data);
  }
}
