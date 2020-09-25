use libjose::jwa::HmacAlgorithm::*;
use libjose::jws::JwsAlgorithm;
use libjose::jws::JwsDecoder;
use libjose::jws::JwsEncoder;
use libjose::jws::JwsHeader;
use libjose::jws::JwsRawToken;
use libjose::jwt::JwtClaims;
use libjose::utils::encode_b64;

fn segment_count(string: &str) -> usize {
  string
    .split('.')
    .filter(|segment| !segment.is_empty())
    .count()
}

#[test]
fn test_compact() {
  let mut header: JwsHeader = JwsHeader::new();
  let mut claims: JwtClaims = JwtClaims::new();

  header.set_kid("#my-key");
  claims.set_aud(vec!["jwk aud"]);
  claims.set_sub("jwk sub");

  let payload: Vec<u8> = serde_json::to_vec(&claims).unwrap();
  let key = HS512.generate_key().unwrap();
  let signer = HS512.signer_from_bytes(key.as_ref()).unwrap();

  let encoded: String = JwsEncoder::new()
    .encode_slice(&payload, &header, &signer)
    .unwrap()
    .to_string()
    .unwrap();

  assert_eq!(segment_count(&encoded), 3);

  let verifier = HS512.verifier_from_bytes(key.as_ref()).unwrap();

  let decoded: JwsRawToken = JwsDecoder::new()
    .alg(HS512)
    .decode_token(&encoded, &verifier)
    .unwrap();

  assert_eq!(decoded.header.alg(), JwsAlgorithm::HS512);
  assert_eq!(decoded.header.kid().unwrap(), "#my-key");
  assert_eq!(decoded.claims, payload);
}

#[test]
fn test_compact_unencoded() {
  let mut header: JwsHeader = JwsHeader::new();

  header.set_b64(false);
  header.set_crit(vec!["b64"]);

  let payload: &[u8] = b"hello world";
  let key = HS512.generate_key().unwrap();
  let signer = HS512.signer_from_bytes(key.as_ref()).unwrap();

  let encoded: String = JwsEncoder::new()
    .encode_slice(payload, &header, &signer)
    .unwrap()
    .to_string()
    .unwrap();

  assert_eq!(segment_count(&encoded), 3);

  let verifier = HS512.verifier_from_bytes(key.as_ref()).unwrap();

  let decoded: JwsRawToken = JwsDecoder::new()
    .alg(HS512)
    .decode_token(&encoded, &verifier)
    .unwrap();

  assert_eq!(decoded.header.alg(), JwsAlgorithm::HS512);
  assert_eq!(decoded.header.b64().unwrap(), false);
  assert_eq!(decoded.header.crit().unwrap(), &["b64".to_string()]);
  assert_eq!(decoded.claims, payload);
}

#[test]
#[should_panic = "Invalid Character: `.`"]
fn test_compact_unencoded_invalid() {
  let mut header: JwsHeader = JwsHeader::new();
  let claims: JwtClaims = JwtClaims::new();

  header.set_b64(false);
  header.set_crit(vec!["b64"]);

  let mut payload: Vec<u8> = serde_json::to_vec(&claims).unwrap();
  payload.push(b'.');

  let key = HS512.generate_key().unwrap();
  let signer = HS512.signer_from_bytes(key.as_ref()).unwrap();

  JwsEncoder::new()
    .encode_slice(&payload, &header, &signer)
    .unwrap()
    .to_string()
    .unwrap();
}

#[test]
#[should_panic = "CryptoError"]
fn test_compact_signature_invalid() {
  let header: JwsHeader = JwsHeader::new();
  let claims: JwtClaims = JwtClaims::new();
  let payload: Vec<u8> = serde_json::to_vec(&claims).unwrap();
  let key = HS512.generate_key().unwrap();
  let signer = HS512.signer_from_bytes(key.as_ref()).unwrap();

  let encoded: String = JwsEncoder::new()
    .encode_slice(&payload, &header, &signer)
    .unwrap()
    .to_string()
    .unwrap();

  let segments: Vec<&str> = encoded.split(".").collect();
  let modified: String = [segments[0], segments[1], "my-signature"].join(".");
  let verifier = HS512.verifier_from_bytes(key.as_ref()).unwrap();

  let _: JwsRawToken = JwsDecoder::new()
    .alg(HS512)
    .decode_token(&modified, &verifier)
    .unwrap();
}

#[test]
#[should_panic = "CryptoError"]
fn test_compact_payload_invalid() {
  let header: JwsHeader = JwsHeader::new();
  let claims: JwtClaims = JwtClaims::new();
  let payload: Vec<u8> = serde_json::to_vec(&claims).unwrap();
  let key = HS512.generate_key().unwrap();
  let signer = HS512.signer_from_bytes(key.as_ref()).unwrap();

  let encoded: String = JwsEncoder::new()
    .encode_slice(&payload, &header, &signer)
    .unwrap()
    .to_string()
    .unwrap();

  let segments: Vec<&str> = encoded.split(".").collect();
  let modified: String = [segments[0], &encode_b64(b"my-payload"), segments[2]].join(".");
  let verifier = HS512.verifier_from_bytes(key.as_ref()).unwrap();

  let _: JwsRawToken = JwsDecoder::new()
    .alg(HS512)
    .decode_token(&modified, &verifier)
    .unwrap();
}

#[test]
#[should_panic = r#"InvalidParam("alg")"#]
fn test_compact_algorithm_invalid() {
  let header: JwsHeader = JwsHeader::new();
  let claims: JwtClaims = JwtClaims::new();
  let payload: Vec<u8> = serde_json::to_vec(&claims).unwrap();
  let key = HS512.generate_key().unwrap();
  let signer = HS512.signer_from_bytes(key.as_ref()).unwrap();

  let encoded: String = JwsEncoder::new()
    .encode_slice(&payload, &header, &signer)
    .unwrap()
    .to_string()
    .unwrap();

  let segments: Vec<&str> = encoded.split(".").collect();
  let modified: String = [segments[0], segments[1], "my-signature"].join(".");
  let verifier = HS256.verifier_from_bytes(key.as_ref()).unwrap();

  let _: JwsRawToken = JwsDecoder::new()
    .alg(HS256)
    .alg(HS512)
    .decode_token(&modified, &verifier)
    .unwrap();
}

#[test]
#[should_panic = r#"InvalidParam("alg")"#]
fn test_compact_algorithm_not_allowed() {
  let header: JwsHeader = JwsHeader::new();
  let claims: JwtClaims = JwtClaims::new();
  let payload: Vec<u8> = serde_json::to_vec(&claims).unwrap();
  let key = HS512.generate_key().unwrap();
  let signer = HS512.signer_from_bytes(key.as_ref()).unwrap();

  let encoded: String = JwsEncoder::new()
    .encode_slice(&payload, &header, &signer)
    .unwrap()
    .to_string()
    .unwrap();

  let segments: Vec<&str> = encoded.split(".").collect();
  let modified: String = [segments[0], segments[1], "my-signature"].join(".");
  let verifier = HS512.verifier_from_bytes(key.as_ref()).unwrap();

  let _: JwsRawToken = JwsDecoder::new()
    .alg(HS256)
    .decode_token(&modified, &verifier)
    .unwrap();
}

#[test]
fn test_detached() {
  let mut header: JwsHeader = JwsHeader::new();

  header.set_kid("#my-key");

  let payload: Vec<u8> = vec![1, 2, 3, 4];
  let key = HS512.generate_key().unwrap();
  let signer = HS512.signer_from_bytes(key.as_ref()).unwrap();

  let encoded: String = JwsEncoder::new()
    .detach()
    .encode_slice(&payload, &header, &signer)
    .unwrap()
    .to_string()
    .unwrap();

  assert_eq!(segment_count(&encoded), 2);

  let payload: Vec<u8> = encode_b64(payload).into_bytes();
  let verifier = HS512.verifier_from_bytes(key.as_ref()).unwrap();

  let decoded: JwsRawToken = JwsDecoder::new()
    .alg(HS512)
    .payload(&payload)
    .decode_token(&encoded, &verifier)
    .unwrap();

  assert!(decoded.claims.is_empty());

  assert_eq!(decoded.header.alg(), JwsAlgorithm::HS512);
  assert_eq!(decoded.header.kid().unwrap(), "#my-key");
}

#[test]
fn test_detached_unencoded() {
  let mut header: JwsHeader = JwsHeader::new();

  header.set_kid("#my-key");
  header.set_b64(false);
  header.set_crit(vec!["b64"]);

  let payload: &[u8] = b"hello";
  let key = HS512.generate_key().unwrap();
  let signer = HS512.signer_from_bytes(key.as_ref()).unwrap();

  let encoded: String = JwsEncoder::new()
    .detach()
    .encode_slice(payload, &header, &signer)
    .unwrap()
    .to_string()
    .unwrap();

  assert_eq!(segment_count(&encoded), 2);

  let verifier = HS512.verifier_from_bytes(key.as_ref()).unwrap();

  let decoded: JwsRawToken = JwsDecoder::new()
    .alg(HS512)
    .payload(payload)
    .decode_token(&encoded, &verifier)
    .unwrap();

  assert_eq!(decoded.header.alg(), JwsAlgorithm::HS512);
  assert_eq!(decoded.header.kid().unwrap(), "#my-key");
}

#[test]
fn test_detached_is_truly_detached() {
  let mut header: JwsHeader = JwsHeader::new();

  header.set_kid("#my-key");
  header.set_b64(false);
  header.set_crit(vec!["b64"]);

  let payload: &[u8] = b"hello";
  let key = HS512.generate_key().unwrap();
  let signer = HS512.signer_from_bytes(key.as_ref()).unwrap();

  let encoded: String = JwsEncoder::new()
    .detach()
    .encode_slice(payload, &header, &signer)
    .unwrap()
    .to_string()
    .unwrap();

  assert_eq!(segment_count(&encoded), 2);

  let segments: Vec<&str> = encoded.split(".").collect();
  let modified: String = [segments[0], "", segments[2]].join(".");
  let verifier = HS512.verifier_from_bytes(key.as_ref()).unwrap();

  let decoded: JwsRawToken = JwsDecoder::new()
    .alg(HS512)
    .payload(payload)
    .decode_token(&modified, &verifier)
    .unwrap();

  assert_eq!(decoded.header.alg(), JwsAlgorithm::HS512);
  assert_eq!(decoded.header.kid().unwrap(), "#my-key");
}

#[test]
#[should_panic = "CryptoError"]
fn test_detached_signature_invalid() {
  let header: JwsHeader = JwsHeader::new();
  let payload: &[u8] = b"hello";
  let key = HS512.generate_key().unwrap();
  let signer = HS512.signer_from_bytes(key.as_ref()).unwrap();

  let encoded: String = JwsEncoder::new()
    .detach()
    .encode_slice(payload, &header, &signer)
    .unwrap()
    .to_string()
    .unwrap();

  let segments: Vec<&str> = encoded.split(".").collect();
  let modified: String = [segments[0], segments[1], &encode_b64(b"my-signature")].join(".");
  let verifier = HS512.verifier_from_bytes(key.as_ref()).unwrap();

  let _: JwsRawToken = JwsDecoder::new()
    .alg(HS512)
    .payload(&encode_b64(payload))
    .decode_token(&modified, &verifier)
    .unwrap();
}

#[test]
#[should_panic = "CryptoError"]
fn test_detached_payload_invalid() {
  let header: JwsHeader = JwsHeader::new();
  let payload: Vec<u8> = vec![1, 2, 3, 4];
  let key = HS512.generate_key().unwrap();
  let signer = HS512.signer_from_bytes(key.as_ref()).unwrap();

  let encoded: String = JwsEncoder::new()
    .detach()
    .encode_slice(&payload, &header, &signer)
    .unwrap()
    .to_string()
    .unwrap();

  let payload: Vec<u8> = vec![5, 6, 7, 8];
  let verifier = HS512.verifier_from_bytes(key.as_ref()).unwrap();

  let _: JwsRawToken = JwsDecoder::new()
    .alg(HS512)
    .payload(&encode_b64(payload))
    .decode_token(&encoded, &verifier)
    .unwrap();
}
