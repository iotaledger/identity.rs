use crate::Result;
use std::io;
use std::io::{Cursor, Read};

pub fn compress_snappy(input: &str) -> Vec<u8> {
  let mut bytes: Vec<u8> = Vec::new();
  let reference = &bytes;
  let mut cursor = Cursor::new(input);
  let mut cursoroutput = Cursor::new(bytes);

  let mut wtr = snap::write::FrameEncoder::new(cursoroutput);
  let x = io::copy(&mut cursor, &mut wtr).expect("I/O operation failed");
  return wtr.into_inner().unwrap().into_inner();
}

pub fn decompress_snappy(input: &Vec<u8>) -> String {
  let mut cursor_input = Cursor::new(input);
  let mut cursor_output = Cursor::new(Vec::new());
  let mut rdr2 = snap::read::FrameDecoder::new(cursor_input);

  io::copy(&mut rdr2, &mut cursor_output).expect("I/O operation failed");
  let result = cursor_output.into_inner();
  return String::from_utf8(result).unwrap();
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_compress() {
    let compressed = compress_snappy(String::from("hello from here").as_str());
    let decompressed = decompress_snappy(&compressed);

    println!("{}", decompressed);
  }
}