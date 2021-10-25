use std::io;
use std::io::prelude::*;
use std::io::{Cursor, Read};

pub fn compress_zstd(input: &str) -> Vec<u8> {
  let output = Cursor::new(Vec::new());
  let mut encoder = zstd::stream::Encoder::new(output, 0).unwrap();
  io::copy(&mut Cursor::new(input), &mut encoder).unwrap();
  return encoder.finish().unwrap().into_inner();
}

pub fn decompress_zstd(input: &Vec<u8>) -> String {
  let mut cursor_input = Cursor::new(input);
  let mut cursor_output = Cursor::new(Vec::new());
  zstd::stream::copy_decode(&mut cursor_input, &mut cursor_output).unwrap();
  let result = cursor_output.into_inner();
  return String::from_utf8(result).unwrap();
}
