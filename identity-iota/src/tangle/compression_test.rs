#[cfg(test)]
mod test {
  use std::time::Instant;
  use identity_core::convert::ToJson;
  use identity_core::crypto::KeyPair;
  use crate::did::IotaDocument;
  use crate::tangle::compression_brotli2::{compress_brotli2, decompress_brotli2};
  use crate::tangle::compression_deflate::{compress_deflate, decompress_deflate};
  use crate::tangle::compression_snappy::{compress_snappy, decompress_snappy};
  use crate::tangle::compressor::{compress_bzip2, decompress_bzip2};
  use super::*;


  #[test]
  fn test_bzip2() {
    println!(">>>>> algorithm: BZIP2 <<<<<");
    let data = get_basic_iota_document();
    let before = Instant::now();
    for i in 0..10000 {
      let compressed = compress_bzip2(&data).unwrap();
    }
    println!("compression finished in {:.2?}", before.elapsed());

    let compressed = compress_bzip2(&data).unwrap();
    let size_before = data.as_str().as_bytes().len();
    let size_after = compressed.len();
    print_ratio(size_before, size_after);

    let before = Instant::now();
    for i in 0..10000 {
      let decompressed = decompress_bzip2(&compressed);
    }
    println!("decompression finished in {:.2?}", before.elapsed());
    let decompressed = decompress_bzip2(&compressed);
    assert_eq!(data, decompressed);
    println!("ــــــــــــــــــــــــــــــــــــــــ")
  }

  #[test]
  fn test_snappy() {
    println!(">>>>> algorithm: SNAPPY <<<<<");
    let data = get_basic_iota_document();
    let before = Instant::now();
    for i in 0..10000 {
      let compressed = compress_snappy(data.as_str());
    }
    println!("snappy finished in {:.2?}", before.elapsed());
    let compressed = compress_snappy(data.as_str());
    let size_before = data.as_str().as_bytes().len();
    let size_after = compressed.len();
    print_ratio(size_before, size_after);

    let before = Instant::now();
    for i in 0..10000 {
      let decompressed = decompress_snappy(&compressed);
    }
    println!("decompression finished in {:.2?}", before.elapsed());

    let decompressed = decompress_snappy(&compressed);
    assert_eq!(data, decompressed);
    println!("ــــــــــــــــــــــــــــــــــــــــ")
  }

  #[test]
  fn test_deflate() {
    println!(">>>>> algorithm: DEFLATE <<<<<");

    let data = get_basic_iota_document();

    // compression time
    let before = Instant::now();
    for i in 0..10000 {
      let compressed = compress_deflate(data.as_str());
    }
    println!("compression time: {:.2?}", before.elapsed());

    // compression ratio
    let compressed = compress_deflate(data.as_str());
    let size_before = data.as_str().as_bytes().len();
    let size_after = compressed.len();
    print_ratio(size_before, size_after);

    // decompression time
    let before = Instant::now();
    for i in 0..10000 {
      let decompressed = decompress_deflate(&compressed);
    }
    println!("decompression finished in {:.2?}", before.elapsed());

    // compare with origin
    let decompressed = decompress_deflate(&compressed);
    assert_eq!(data, decompressed);
    println!("ــــــــــــــــــــــــــــــــــــــــ")
  }

  #[test]
  fn test_brotli() {
    test_compression_algorithm("BROTLI", compress_brotli2, decompress_brotli2)
    // println!(">>>>> algorithm: Brotli <<<<<");
    // let data = get_basic_iota_document();
    //
    // // compression time
    // let before = Instant::now();
    // for i in 0..10000 {
    //   let compressed = compress_brotli2(data.as_str());
    // }
    // println!("compression time: {:.2?}", before.elapsed());
    //
    // // compression ratio
    // let compressed = compress_brotli2(data.as_str());
    // let size_before = data.as_str().as_bytes().len();
    // let size_after = compressed.len();
    // print_ratio(size_before, size_after);
    //
    //
    // let before = Instant::now();
    // for i in 0..10000 {
    //   let decompressed = decompress_brotli2(&compressed);
    // }
    // println!("decompression finished in {:.2?}", before.elapsed());
    //
    // let decompressed = decompress_brotli2(&compressed);
    // assert_eq!(decompressed, data);
    // println!("ــــــــــــــــــــــــــــــــــــــــ")
  }


  fn test_compression_algorithm(
    algorithm_name: &str,
    compress: fn(input: &str) -> Vec<u8>,
    decompress: fn(input: &Vec<u8>) -> String,
  ) {
    println!(">>>>> algorithm: {} <<<<<", algorithm_name);
    let data = get_basic_iota_document();

    // compression time
    let before = Instant::now();
    for i in 0..10000 {
      let compressed = compress(data.as_str());
    }
    println!("compression time: {:.2?}", before.elapsed());

    // compression ratio
    let compressed = compress(data.as_str());
    let size_before = data.as_str().as_bytes().len();
    let size_after = compressed.len();
    print_ratio(size_before, size_after);


    let before = Instant::now();
    for i in 0..10000 {
      let decompressed = decompress(&compressed);
    }
    println!("decompression finished in {:.2?}", before.elapsed());

    let decompressed = decompress(&compressed);
    assert_eq!(decompressed, data);
    println!("ــــــــــــــــــــــــــــــــــــــــ")
  }

  fn get_basic_iota_document() -> String {
    let keypair: KeyPair = KeyPair::new_ed25519().unwrap();
    let mut document: IotaDocument = IotaDocument::new(&keypair).unwrap();
    document.sign(&keypair.private()).unwrap();
    return document.to_json().unwrap();
  }

  fn print_ratio(size_before: usize, size_after: usize) {
    let ratio: f64 = size_after as f64 / size_before as f64;
    let compressed_ratio: f64 = 1.0 - ratio;
    println!("Before: {}\nAfter: {}\nRatio: {}\nCompressed Ratio: {}", size_before, size_after, ratio, compressed_ratio);
  }
}
