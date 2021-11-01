use brotli2::read::{BrotliDecoder, BrotliEncoder};
use std::io::prelude::*;
use crate::error::Error;
use crate::error::Error::CompressionError;

pub(crate) fn compress_brotli2<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>, Error> {
  let mut result = Vec::new();
  let mut e = BrotliEncoder::new(input.as_ref(), 6);
  e.read_to_end(&mut result).map_err(|_|Error::CompressionError)?;
  Ok(result)
}

pub(crate) fn decompress_brotli2<T: AsRef<[u8]> + ?Sized>(input: &T) -> Result<String, Error> {
  let mut z = BrotliDecoder::new(input.as_ref());
  let mut s = String::new();
  z.read_to_string(&mut s).map_err(|_| CompressionError)?;
  return Ok(s);
}
