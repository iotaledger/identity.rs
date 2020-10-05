use identity_account::storage::{CacheFile, HuffmanCodec};
use std::fs::metadata;

// plain text string file.
const RAW_STR: &str = include_str!("compression.txt");
const PLAIN_PATH: &str = "tests/compression.txt";

#[test]
fn test_compression_writing() {
    let expected = RAW_STR;

    let filename = String::from("tests/test");

    let cache_compressed = CacheFile::new(filename);
    let cache_plain = CacheFile::new(PLAIN_PATH.into());

    let compressed = HuffmanCodec::compress(expected.into()).unwrap();

    cache_compressed.write_cache_file(compressed).unwrap();

    let contents = cache_compressed.read_cache_file().unwrap();

    let decompressed = HuffmanCodec::decompress(&contents).unwrap();

    let metadata_compressed = metadata(cache_compressed.get_name()).unwrap();
    let metadata_plain = metadata(cache_plain.get_name()).unwrap();

    let compressed_len = metadata_compressed.len();
    let plain_len = metadata_plain.len();

    assert_eq!(expected, decompressed);
    assert!(plain_len > compressed_len);
}
