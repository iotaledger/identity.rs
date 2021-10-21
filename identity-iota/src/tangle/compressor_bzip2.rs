use bzip2::read::{BzDecoder, BzEncoder};
use bzip2::Compression;

use crate::Error;
use crate::Result;
use std::io::Read;

// String, &str, Vec<u8>, &[u8]
pub fn compress_bzip2(input: &str) -> Vec<u8> {
  let mut compressor = BzEncoder::new(input.as_bytes(), Compression::best());
  let mut bytes: Vec<u8> = Vec::new();
  let res = compressor.read_to_end(&mut bytes).map_err(|e| {
    return Error::CompressionError;
  });
  return bytes;
}

pub fn decompress_bzip2<T: AsRef<[u8]>>(input: &T) -> String {
  let mut decompressor = BzDecoder::new(input.as_ref());
  let mut s = String::new();
  decompressor.read_to_string(&mut s).unwrap();
  return s;
}
