use identity_account::storage::HuffmanCodec;
use proptest::proptest;

const RAW_STR: &str = include_str!("compression.txt");

#[test]
fn test_encode_decode() {
    let expected = RAW_STR;

    let compressed = HuffmanCodec::compress(expected.into()).unwrap();

    let decompressed = HuffmanCodec::decompress(&compressed).unwrap();

    assert_eq!(expected, decompressed);
}

proptest! {
    #[test]
    fn prop_check_encode_decode(s in "[a-zA-Z0-9._!~$&'()*+;,=/?:@-]+[a-zA-Z0-9._!~$&'()*+;,=/?:@-]+") {
        let expected = String::from(&s);

        let compressed = HuffmanCodec::compress(s).unwrap();
        let decompressed = HuffmanCodec::decompress(&compressed).unwrap();

        assert_eq!(expected, decompressed);

    }

    #[test]
    fn prop_test_encode_decode_len(s in "[a-zA-Z0-9._!~$&'()*+;,=/?:@-]+[a-zA-Z0-9._!~$&'()*+;,=/?:@-]+") {
        let expected = s.len();

        let compressed = HuffmanCodec::compress(s).unwrap();
        let decompressed = HuffmanCodec::decompress(&compressed).unwrap();

        assert_eq!(expected, decompressed.len());
    }
}
