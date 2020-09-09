use identity_ldp::LinkedDataSignature;
use serde_json::from_str;

macro_rules! test_parse_lds {
    ($type:expr, $path:expr) => {
        let signature: LinkedDataSignature = from_str(include_str!($path)).unwrap();
        assert_eq!(signature.proof_type, $type);
    };
}

#[test]
fn test_parse_lds_bbsblssignature2020() {
    test_parse_lds!("BbsBlsSignature2020", "input/bbsblssignature2020.json");
}

#[test]
fn test_parse_lds_ecdsasecp256k1signature2019() {
    test_parse_lds!("EcdsaSecp256k1Signature2019", "input/ecdsasecp256k1signature2019.json");
}

#[test]
fn test_parse_lds_ed25519signature2018() {
    test_parse_lds!("Ed25519Signature2018", "input/ed25519signature2018.json");
}

#[test]
fn test_parse_lds_jcsed25519signature2020() {
    test_parse_lds!("JcsEd25519Signature2020", "input/jcsed25519signature2020.json");
}

#[test]
fn test_parse_lds_jsonwebsignature2020() {
    test_parse_lds!("JsonWebSignature2020", "input/jsonwebsignature2020.json");
}

#[test]
fn test_parse_lds_rsasignature2018() {
    test_parse_lds!("RsaSignature2018", "input/rsasignature2018.json");
}
