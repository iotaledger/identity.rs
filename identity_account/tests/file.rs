use identity_account::storage::{read_cache_file, write_cache_file, HuffmanCodec};
use std::fs::metadata;

// plain text string file.
const RAW_STR: &str = include_str!("compression.txt");
const PLAIN_PATH: &str = "tests/compression.txt";

#[test]
fn test_compression_writing() {
    let expected = RAW_STR;

    let filename = String::from("tests/test");

    let compressed = HuffmanCodec::compress(expected.into()).unwrap();

    write_cache_file(compressed, &filename).unwrap();

    let contents = read_cache_file(&filename).unwrap();

    let decompressed = HuffmanCodec::decompress(&contents).unwrap();

    let metadata_compressed = metadata(filename).unwrap();
    let metadata_plain = metadata(PLAIN_PATH).unwrap();

    let compressed_len = metadata_compressed.len();
    let plain_len = metadata_plain.len();

    assert_eq!(expected, decompressed);
    assert!(plain_len > compressed_len);
}
