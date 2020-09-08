use identity_ldp::LinkedDataSignature;
use serde_json::from_str;

#[test]
fn test_parse_signature() {
    let signature: LinkedDataSignature = from_str(include_str!("input/ecdsasecp256k1signature2019.json")).unwrap();
    assert_eq!(signature.proof_type, "EcdsaSecp256k1Signature2019");

    let signature: LinkedDataSignature = from_str(include_str!("input/rsasignature2018.json")).unwrap();
    assert_eq!(signature.proof_type, "RsaSignature2018");

    let signature: LinkedDataSignature = from_str(include_str!("input/ed25519signature2018.json")).unwrap();
    assert_eq!(signature.proof_type, "Ed25519Signature2018");

    let signature: LinkedDataSignature = from_str(include_str!("input/jsonwebsignature2020.json")).unwrap();
    assert_eq!(signature.proof_type, "JsonWebSignature2020");

    let signature: LinkedDataSignature = from_str(include_str!("input/bbsblssignature2020.json")).unwrap();
    assert_eq!(signature.proof_type, "BbsBlsSignature2020");
}
