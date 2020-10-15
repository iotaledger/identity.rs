use identity_core::common::Object;
use identity_proof::LinkedDataSignature;
use serde_json::{from_str, to_string};

macro_rules! test_parse_lds {
    ($type:expr, $path:expr) => {
        let signature_data = include_str!($path);
        let signature = from_str::<LinkedDataSignature>(signature_data).unwrap();
        let signature_json = to_string(&signature).unwrap();

        // Ensure we have the expected type of proof
        assert_eq!(signature.proof_type, $type);

        let object_a = from_str::<Object>(signature_data).unwrap();
        let object_b = from_str::<Object>(&signature_json).unwrap();

        // Ensure nothing was modified during de/serialization
        assert_eq!(object_a, object_b);
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
