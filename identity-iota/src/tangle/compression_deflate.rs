use std::io::Cursor;
use flate2::Compression;
use flate2::read::{GzDecoder, ZlibEncoder};
use std::io::prelude::*;
use flate2::write::ZlibDecoder;

pub fn compress_deflate(input: &str) -> Vec<u8> {
  let mut e = ZlibEncoder::new(input.as_bytes(), Compression::default());
  let mut result = Vec::new();
  let compressed_bytes = e.read_to_end(&mut result);
  return result;
}

pub fn decompress_deflate(input: &Vec<u8>) -> String {
  let mut output = Vec::new();
  let mut z = ZlibDecoder::new(output);
  z.write_all(input);
  let writer = z.finish().unwrap();
  let return_string = String::from_utf8(writer).expect("String parsing error");
  return return_string;
}