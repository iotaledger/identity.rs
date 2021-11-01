#[cfg(test)]
mod test {
  use super::*;
  use crate::did::{IotaDocument, IotaVerificationMethod};
  use crate::tangle::compression_brotli2::{compress_brotli2, decompress_brotli2};
  use crate::tangle::TangleRef;
  use brotli2::read::{BrotliDecoder, BrotliEncoder};
  use identity_core::convert::{FromJson, ToJson};
  use identity_core::crypto::KeyPair;
  use identity_core::json;
  use identity_did::service::Service;
  use identity_did::verification::MethodScope;
  use serde::Serialize;
  use std::fs::File;
  use std::io::prelude::*;
  use std::time::Instant;


  fn test_brotli() {
    // test_compression_algorithm("BROTLI", compress_brotli2, decompress_brotli2)
  }

  #[test]
  fn test_compression_algorithm() {
    println!(">>>>> algorithm: Brotli <<<<<");
    let data = String::from("{\"id\":\"did:iota:U1zDCC47hjcXupixrw5kbKCxV3ZCUn3RomJ5VkX1wHv\",\"verificationMethod\":[{\"id\":\"did:iota:U1zDCC47hjcXupixrw5kbKCxV3ZCUn3RomJ5VkX1wHv#authentication\",\"controller\":\"did:iota:U1zDCC47hjcXupixrw5kbKCxV3ZCUn3RomJ5VkX1wHv\",\"type\":\"Ed25519VerificationKey2018\",\"publicKeyBase58\":\"Mamni7dcVHSDvyAHy4MLFaYTDqh2upJJWca4yqF82yb\"}],\"authentication\":[\"did:iota:U1zDCC47hjcXupixrw5kbKCxV3ZCUn3RomJ5VkX1wHv#authentication\"],\"created\":\"2021-10-21T09:37:56Z\",\"updated\":\"2021-10-21T09:37:56Z\",\"proof\":{\"type\":\"JcsEd25519Signature2020\",\"verificationMethod\":\"#authentication\",\"signatureValue\":\"M4qQSeVBRXCumiCqA6hNNcb6xBVmtdGD7tgAvDfTPfPjwRfYof8ZUBihwdEhmD6jDTcWpSanWDWJKsxc1yK5gUo\"}}");

    // compression time
    let before = Instant::now();
    for i in 0..10000 {
      let compressed = compress_brotli2(data.as_str());
    }
    println!("compression time: {:.2?}", before.elapsed());

    // compression ratio
    let compressed = compress_brotli2(data.as_str()).unwrap();
    let size_before = data.as_str().as_bytes().len();
    let size_after = compressed.len();

    print_ratio(size_before, size_after);

    let before = Instant::now();
    for i in 0..10000 {
      let decompressed = decompress_brotli2(&compressed);
    }
    println!("decompression finished in {:.2?}", before.elapsed());

    let decompressed = decompress_brotli2(&compressed).unwrap();
    assert_eq!(decompressed, data);
    println!("ــــــــــــــــــــــــــــــــــــ")
  }


  fn print_ratio(size_before: usize, size_after: usize) {
    let ratio: f64 = size_after as f64 / size_before as f64;
    let compressed_ratio: f64 = 1.0 - ratio;
    println!(
      "Before: {}\nAfter: {}\nRatio: {}\nCompressed Ratio: {}",
      size_before, size_after, ratio, compressed_ratio
    );
  }
}
