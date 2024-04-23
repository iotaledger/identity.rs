// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod es256 {
    use identity_verification::jwk::EcCurve;
    use identity_verification::jwk::Jwk;
    use identity_verification::jwk::JwkParamsEc;
    use identity_verification::jwu;
    use p256::ecdsa::Signature;
    use p256::ecdsa::SigningKey;
    use p256::SecretKey;

    pub(crate) fn expand_p256_jwk(jwk: &Jwk) -> SecretKey {
        let params: &JwkParamsEc = jwk.try_ec_params().unwrap();

        if params.try_ec_curve().unwrap() != EcCurve::P256 {
            panic!("expected a P256 curve");
        }

        let sk_bytes = params.d.as_ref().map(jwu::decode_b64).unwrap().unwrap();
        SecretKey::from_slice(&sk_bytes).unwrap()
    }

    pub(crate) fn sign(message: &[u8], private_key: &Jwk) -> impl AsRef<[u8]> {
        let sk: SecretKey = expand_p256_jwk(private_key);
        let signing_key: SigningKey = SigningKey::from(sk);
        let signature: Signature = signature::Signer::sign(&signing_key, message);
        signature.to_bytes()
    }
}

mod es256k1 {
    use identity_verification::jwk::EcCurve;
    use identity_verification::jwk::Jwk;
    use identity_verification::jwk::JwkParamsEc;
    use identity_verification::jwu;
    use k256::ecdsa::Signature;
    use k256::ecdsa::SigningKey;
    use k256::SecretKey;

    pub(crate) fn expand_k256_jwk(jwk: &Jwk) -> SecretKey {
        let params: &JwkParamsEc = jwk.try_ec_params().unwrap();

        if params.try_ec_curve().unwrap() != EcCurve::Secp256K1 {
            panic!("expected a Secp256K1 curve");
        }

        let sk_bytes = params.d.as_ref().map(jwu::decode_b64).unwrap().unwrap();
        SecretKey::from_slice(&sk_bytes).unwrap()
    }

    pub(crate) fn sign(message: &[u8], private_key: &Jwk) -> impl AsRef<[u8]> {
        let sk: SecretKey = expand_k256_jwk(private_key);
        let signing_key: SigningKey = SigningKey::from(sk);
        let signature: Signature = signature::Signer::sign(&signing_key, message);
        signature.to_bytes()
    }
}

use identity_verification::jwk::Jwk;
use identity_verification::jws;
use identity_verification::jws::JwsHeader;

use crate::EcDSAJwsVerifier;

#[test]
fn test_es256_rfc7515() {
    // Test Vector taken from https://datatracker.ietf.org/doc/html/rfc7515#appendix-A.3.
    let tv_header: &str = r#"{"alg":"ES256"}"#;
    let tv_claims: &[u8] = &[
        123, 34, 105, 115, 115, 34, 58, 34, 106, 111, 101, 34, 44, 13, 10, 32, 34, 101, 120, 112,
        34, 58, 49, 51, 48, 48, 56, 49, 57, 51, 56, 48, 44, 13, 10, 32, 34, 104, 116, 116, 112, 58,
        47, 47, 101, 120, 97, 109, 112, 108, 101, 46, 99, 111, 109, 47, 105, 115, 95, 114, 111,
        111, 116, 34, 58, 116, 114, 117, 101, 125,
    ];
    let tv_encoded: &[u8] = b"eyJhbGciOiJFUzI1NiJ9.eyJpc3MiOiJqb2UiLA0KICJleHAiOjEzMDA4MTkzODAsDQogImh0dHA6Ly9leGFtcGxlLmNvbS9pc19yb290Ijp0cnVlfQ.e4ZrhZdbFQ7630Tq51E6RQiJaae9bFNGJszIhtusEwzvO21rzH76Wer6yRn2Zb34VjIm3cVRl0iQctbf4uBY3w";
    let tv_private_key: &str = r#"
        {
          "kty": "EC",
          "crv": "P-256",
          "x": "f83OJ3D2xF1Bg8vub9tLe1gHMzV76e8Tus9uPHvRVEU",
          "y": "x_FEzRu9m36HLN_tue659LNpXW6pCyStikYjKIWI5a0",
          "d": "jpsQnnGQmL-YBIffH1136cspYG6-0iY7X1fCE9-E9LI"
        }
      "#;

    let header: JwsHeader = serde_json::from_str(tv_header).unwrap();
    let jwk: Jwk = serde_json::from_str(tv_private_key).unwrap();
    let encoder: jws::CompactJwsEncoder<'_> =
        jws::CompactJwsEncoder::new(tv_claims, &header).unwrap();
    let signing_input: &[u8] = encoder.signing_input();
    let encoded: String = {
        let signature = es256::sign(signing_input, &jwk);
        encoder.into_jws(signature.as_ref())
    };
    assert_eq!(encoded.as_bytes(), tv_encoded);

    let jws_verifier = EcDSAJwsVerifier::default();
    let decoder = jws::Decoder::new();
    let token = decoder
        .decode_compact_serialization(tv_encoded, None)
        .and_then(|decoded| decoded.verify(&jws_verifier, &jwk))
        .unwrap();

    assert_eq!(token.protected, header);
    assert_eq!(token.claims, tv_claims);
}

#[test]
fn test_es256k_verifier() {
    let tv_header: &str = r#"{
    "typ": "JWT",
    "alg":"ES256K"
  }"#;
    let tv_private_key: &str = r#"
  {
    "kty":"EC",
    "crv":"secp256k1",
    "d":"y0zUV7bLeUG_kDOvACFHnSmtH7j8MSJek25R2wJbWWg",
    "x":"BBobbZkiC8E4C4EYekPNJkcXFCsMNHhh0AV2USy_xSs",
    "y":"VQcPHjIQClX0b5TLluFl6jpIf9U-norWC0oEvIQRNyU"
  }"#;
    let tv_claims: &[u8] = br#"{"key":"value"}"#;

    let header: JwsHeader = serde_json::from_str(tv_header).unwrap();
    let jwk: Jwk = serde_json::from_str(tv_private_key).unwrap();
    let encoder: jws::CompactJwsEncoder<'_> =
        jws::CompactJwsEncoder::new(tv_claims, &header).unwrap();
    let signing_input: &[u8] = encoder.signing_input();
    let encoded: String = {
        let signature = es256k1::sign(signing_input, &jwk);
        encoder.into_jws(signature.as_ref())
    };

    let jws_verifier = EcDSAJwsVerifier::default();
    let jwk: Jwk = serde_json::from_str(tv_private_key).unwrap();
    let decoder = jws::Decoder::new();
    assert!(decoder
        .decode_compact_serialization(encoded.as_bytes(), None)
        .and_then(|decoded| decoded.verify(&jws_verifier, &jwk))
        .is_ok());
}

/// In the absence of official test vectors for secp256k1,
/// this ensures we can verify JWTs created by other libraries.
mod test_es256k_josekit {
    use identity_verification::jws;
    use josekit::jwk::alg::ec::EcKeyPair;
    use josekit::jwk::Jwk;
    use josekit::jws::JwsHeader;
    use josekit::jwt::JwtPayload;

    use crate::EcDSAJwsVerifier;

    #[test]
    fn test_es256k_josekit() {
        let alg = josekit::jws::ES256K;

        let private_key: &str = r#"
    {
      "kty":"EC",
      "crv":"secp256k1",
      "d":"y0zUV7bLeUG_kDOvACFHnSmtH7j8MSJek25R2wJbWWg",
      "x":"BBobbZkiC8E4C4EYekPNJkcXFCsMNHhh0AV2USy_xSs",
      "y":"VQcPHjIQClX0b5TLluFl6jpIf9U-norWC0oEvIQRNyU"
    }"#;
        let josekit_jwk: Jwk = serde_json::from_str(private_key).unwrap();
        let mut src_header = JwsHeader::new();
        src_header.set_token_type("JWT");
        let mut src_payload = JwtPayload::new();
        src_payload.set_claim("key", Some("value".into())).unwrap();
        let eckp = EcKeyPair::from_jwk(&josekit_jwk).unwrap();
        let signer = alg.signer_from_jwk(&eckp.to_jwk_key_pair()).unwrap();
        let jwt_string =
            josekit::jwt::encode_with_signer(&src_payload, &src_header, &signer).unwrap();

        let jws_verifier = EcDSAJwsVerifier::default();
        let decoder = jws::Decoder::new();
        let jwk: identity_verification::jwk::Jwk = serde_json::from_str(private_key).unwrap();
        assert!(decoder
            .decode_compact_serialization(jwt_string.as_bytes(), None)
            .and_then(|decoded| decoded.verify(&jws_verifier, &jwk))
            .is_ok());
    }
}
