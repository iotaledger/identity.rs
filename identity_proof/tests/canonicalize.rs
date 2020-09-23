use identity_common::SerdeInto;
use identity_proof::{CanonicalJson, Canonicalize};
use serde_json::json;

#[test]
fn test_canonicalize_json() {
    let object = json!({"3": true, "1": true, "2": true}).serde_into().unwrap();

    let json = CanonicalJson.canonicalize(&object).unwrap();
    let output = String::from_utf8_lossy(&json).to_string();

    assert_eq!(output, r#"{"1":true,"2":true,"3":true}"#);
}

#[test]
fn test_canonicalize_json_equality() {
    let a = json!({"3": true, "1": true, "2": true}).serde_into().unwrap();
    let b = json!({"2": true, "3": true, "1": true}).serde_into().unwrap();

    let json_a = CanonicalJson.canonicalize(&a).unwrap();
    let json_b = CanonicalJson.canonicalize(&b).unwrap();

    assert_eq!(json_a, json_b);
}

#[test]
fn test_canonicalize_json_nested() {
    let object = json!({
      "vc": {
        "proof": {
          "jws": "jws-token",
          "data": [3,2,1],
        },
        "issuer": "did:example:123",
      }
    })
    .serde_into()
    .unwrap();

    let json = CanonicalJson.canonicalize(&object).unwrap();
    let output = String::from_utf8_lossy(&json).to_string();

    assert_eq!(
        output,
        r#"{"vc":{"issuer":"did:example:123","proof":{"data":[3,2,1],"jws":"jws-token"}}}"#
    );
}
