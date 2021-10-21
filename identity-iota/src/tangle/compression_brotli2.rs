use brotli2::read::{BrotliDecoder, BrotliEncoder};
use std::io::prelude::*;

pub fn compress_brotli2(input: &str) -> Vec<u8> {
  let mut result = Vec::new();
  let mut e = BrotliEncoder::new(input.as_bytes(), 6);
  e.read_to_end(&mut result);
  return result;
}

pub fn decompress_brotli2<T: AsRef<[u8]>>(input: &T) -> String {
  let mut z = BrotliDecoder::new(input.as_ref());
  let mut s = String::new();
  z.read_to_string(&mut s).unwrap();
  return s;
}
