#[cfg(test)]
mod test {
  use super::*;
  use crate::did::{IotaDocument, IotaVerificationMethod};
  use crate::tangle::compress_zstd::{compress_zstd, decompress_zstd};
  use crate::tangle::compression_brotli2::{compress_brotli2, decompress_brotli2};
  use crate::tangle::compression_deflate::{compress_deflate, decompress_deflate};
  use crate::tangle::compression_snappy::{compress_snappy, decompress_snappy};
  use crate::tangle::compressor_bzip2::{compress_bzip2, decompress_bzip2};
  use crate::tangle::TangleRef;
  use brotli2::read::{BrotliDecoder, BrotliEncoder};
  use identity_core::convert::{FromJson, ToJson};
  use identity_core::crypto::KeyPair;
  use identity_core::json;
  use identity_did::service::Service;
  use identity_did::verification::MethodScope;
  use rmps::{Deserializer, Serializer};
  use serde::Serialize;
  use std::fs::File;
  use std::io::prelude::*;
  use std::time::Instant;

  extern crate rmp_serde as rmps;

  #[test]
  fn test_bzip2() {
    test_compression_algorithm("BZIP2", compress_bzip2, decompress_bzip2);
  }

  #[test]
  fn test_snappy() {
    test_compression_algorithm("SNAPPY", compress_snappy, decompress_snappy)
  }

  #[test]
  fn test_deflate() {
    test_compression_algorithm("ZLIB (DEFLATE)", compress_deflate, decompress_deflate)
  }

  #[test]
  fn test_zstd() {
    test_compression_algorithm("ZSTD", compress_zstd, decompress_zstd)
  }

  #[test]
  fn test_brotli() {
    // test_compression_algorithm("BROTLI", compress_brotli2, decompress_brotli2)
  }

  fn test_compression_algorithm(
    algorithm_name: &str,
    compress: fn(input: &str) -> Vec<u8>,
    decompress: fn(input: &Vec<u8>) -> String,
  ) {
    println!(">>>>> algorithm: {} <<<<<", algorithm_name);
    let (document, data) = get_basic_iota_document();

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
    println!("ــــــــــــــــــــــــــــــــــــ")
  }

  #[test]
  fn test_message_pack() {
    println!(">>>>> Serialization:  Message Pack<<<<<");
    let (document, data) = get_basic_iota_document();
    let mut buf = Vec::new();
    document.serialize(&mut Serializer::new(&mut buf));
    print_ratio(document.to_string().as_bytes().len(), buf.len());
    println!("ــــــــــــــــــــــــــــــــــــ")
  }

  #[test]
  fn test_message_pack_brotli() {
    println!(">>>>> Serialization + Compression:  Message Pack + Brotli <<<<<");
    let (document, data) = get_basic_iota_document();
    let mut buf = Vec::new();
    document.serialize(&mut Serializer::new(&mut buf));
    let mut result = Vec::new();
    let mut e = BrotliEncoder::new(buf.as_slice(), 6);
    e.read_to_end(&mut result);

    print_ratio(document.to_string().as_bytes().len(), result.len());
    println!("ــــــــــــــــــــــــــــــــــــ")
  }

  #[test]
  fn test_message_cbor() {
    println!(">>>>> Serialization:  Cbor<<<<<");
    let (document, data) = get_basic_iota_document();
    let mut buf = Vec::new();

    document.serialize(&mut serde_cbor::Serializer::new(&mut buf));

    print_ratio(document.to_string().as_bytes().len(), buf.len());
    println!("ــــــــــــــــــــــــــــــــــــ")
  }

  fn get_basic_iota_document() -> (IotaDocument, String) {
    let keypair: KeyPair = KeyPair::new_ed25519().unwrap();
    let mut document: IotaDocument = IotaDocument::new(&keypair).unwrap();
    document.sign(&keypair.private()).unwrap();

    let new_key = KeyPair::new_ed25519().unwrap();
    let method: IotaVerificationMethod =
      IotaVerificationMethod::from_did(document.did().clone(), &new_key, "newKey").unwrap();
    assert!(document.insert_method(MethodScope::VerificationMethod, method));

    let new_key: KeyPair = KeyPair::new_ed25519().unwrap();
    let method: IotaVerificationMethod =
      IotaVerificationMethod::from_did(document.did().clone(), &new_key, "newKey2").unwrap();
    assert!(document.insert_method(MethodScope::VerificationMethod, method));

    let new_key: KeyPair = KeyPair::new_ed25519().unwrap();
    let method: IotaVerificationMethod =
      IotaVerificationMethod::from_did(document.did().clone(), &new_key, "newKey3").unwrap();
    assert!(document.insert_method(MethodScope::VerificationMethod, method));

    let service: Service = Service::from_json_value(json!({
      "id": document.id().join("#linked-domain").unwrap(),
      "type": "LinkedDomains",
      "serviceEndpoint": "https://iota.org"
    }))
    .unwrap();
    assert!(document.insert_service(service));

    let service: Service = Service::from_json_value(json!({
      "id": document.id().join("#linked-domain2").unwrap(),
      "type": "LinkedDomain2s",
      "serviceEndpoint": "https://iota2.org"
    }))
    .unwrap();
    assert!(document.insert_service(service));

    let service: Service = Service::from_json_value(json!({
      "id": document.id().join("#linked-domain3").unwrap(),
      "type": "LinkedDomain242332s",
      "serviceEndpoint": "https://example.example"
    }))
    .unwrap();
    assert!(document.insert_service(service));

    let res = document.to_json().unwrap();
    println!("{}", res);

    return (document, res);
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
