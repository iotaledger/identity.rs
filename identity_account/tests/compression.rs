use identity_account::storage::HuffmanCodec;
use proptest::proptest;

#[test]
fn test_encode_decode() {
    let expected = "uuuuuuuuuuuuuuuuuuu987123iouyasdoi7a8s7d698a7sydn9c87wyqei87yjh8q7dh6e8ji7qw6jd8q7w6eji8q76ein8qc76eni867tni76ycnjukaytsduytuiuytiuytaisudnc19826n87tna9876tn987yarnfffffffffffffffffffffti87aynt8";

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
