use libjose::jwa::HmacAlgorithm::HS512;
use libjose::jws::Decoder;
use libjose::jws::Encoder;
use libjose::jws::JwsAlgorithm;
use libjose::jws::JwsHeader;
use libjose::jws::JwsRawToken;
use libjose::jwt::JwtClaims;
use libjose::utils::encode_b64;
use libjose::utils::Empty;

fn segment_count(string: &str) -> usize {
  string
    .split('.')
    .filter(|segment| !segment.is_empty())
    .count()
}

#[test]
fn test_compact() {
  let mut header: JwsHeader<Empty> = JwsHeader::new();
  let mut claims: JwtClaims<Empty> = JwtClaims::new();

  header.set_kid("#my-key");
  claims.set_aud(vec!["jwk aud"]);
  claims.set_sub("jwk sub");

  let payload: Vec<u8> = serde_json::to_vec(&claims).unwrap();
  let key = HS512.generate_key().unwrap();
  let signer = HS512.signer_from_bytes(key.as_ref()).unwrap();
  let serialized: String = Encoder::encode_compact(&payload, &header, &signer).unwrap();
  assert_eq!(segment_count(&serialized), 3);

  let verifier = HS512.verifier_from_bytes(key.as_ref()).unwrap();
  let decoder: Decoder = Decoder::with_algorithms(vec![HS512]);
  let deserialized: JwsRawToken<Empty> = decoder.decode_compact(&serialized, &verifier).unwrap();
  assert_eq!(deserialized.header.alg(), JwsAlgorithm::HS512);
  assert_eq!(deserialized.header.kid().unwrap(), "#my-key");
  assert_eq!(deserialized.claims, payload);
}

#[test]
fn test_compact_unencoded() {
  let mut header: JwsHeader<Empty> = JwsHeader::new();
  let mut claims: JwtClaims<Empty> = JwtClaims::new();

  header.set_b64(false);
  header.set_crit(vec!["b64"]);
  claims.set_aud(vec!["jwk aud"]);
  claims.set_iss("jwk iss");

  let payload: Vec<u8> = serde_json::to_vec(&claims).unwrap();
  let key = HS512.generate_key().unwrap();
  let signer = HS512.signer_from_bytes(key.as_ref()).unwrap();
  let serialized: String = Encoder::encode_compact(&payload, &header, &signer).unwrap();
  assert_eq!(segment_count(&serialized), 3);

  let verifier = HS512.verifier_from_bytes(key.as_ref()).unwrap();
  let decoder: Decoder = Decoder::with_algorithms(vec![HS512]);
  let deserialized: JwsRawToken<Empty> = decoder.decode_compact(&serialized, &verifier).unwrap();
  assert_eq!(deserialized.header.alg(), JwsAlgorithm::HS512);
  assert_eq!(deserialized.header.b64().unwrap(), false);
  assert_eq!(deserialized.header.crit().unwrap(), &["b64".to_string()]);
  assert_eq!(deserialized.claims, payload);
}

#[test]
#[should_panic = "InvalidContentChar"]
fn test_compact_unencoded_invalid() {
  let mut header: JwsHeader<Empty> = JwsHeader::new();
  let claims: JwtClaims<Empty> = JwtClaims::new();

  header.set_b64(false);
  header.set_crit(vec!["b64"]);

  let mut payload: Vec<u8> = serde_json::to_vec(&claims).unwrap();
  payload.push(b'.');

  let key = HS512.generate_key().unwrap();
  let signer = HS512.signer_from_bytes(key.as_ref()).unwrap();

  Encoder::encode_compact(&payload, &header, &signer).unwrap();
}

#[test]
#[should_panic = "CryptoError"]
fn test_compact_signature_invalid() {
  let header: JwsHeader<Empty> = JwsHeader::new();
  let claims: JwtClaims<Empty> = JwtClaims::new();
  let payload: Vec<u8> = serde_json::to_vec(&claims).unwrap();
  let key = HS512.generate_key().unwrap();
  let signer = HS512.signer_from_bytes(key.as_ref()).unwrap();
  let serialized: String = Encoder::encode_compact(&payload, &header, &signer).unwrap();
  let segments: Vec<&str> = serialized.split(".").collect();
  let modified: String = [segments[0], segments[1], "my-signature"].join(".");
  let verifier = HS512.verifier_from_bytes(key.as_ref()).unwrap();
  let decoder: Decoder = Decoder::with_algorithms(vec![HS512]);
  let _: JwsRawToken<Empty> = decoder.decode_compact(&modified, &verifier).unwrap();
}

#[test]
#[should_panic = "CryptoError"]
fn test_compact_payload_invalid() {
  let header: JwsHeader<Empty> = JwsHeader::new();
  let claims: JwtClaims<Empty> = JwtClaims::new();
  let payload: Vec<u8> = serde_json::to_vec(&claims).unwrap();
  let key = HS512.generate_key().unwrap();
  let signer = HS512.signer_from_bytes(key.as_ref()).unwrap();
  let serialized: String = Encoder::encode_compact(&payload, &header, &signer).unwrap();
  let segments: Vec<&str> = serialized.split(".").collect();
  let modified: String = [segments[0], "my-payload", segments[2]].join(".");
  let verifier = HS512.verifier_from_bytes(key.as_ref()).unwrap();
  let decoder: Decoder = Decoder::with_algorithms(vec![HS512]);
  let _: JwsRawToken<Empty> = decoder.decode_compact(&modified, &verifier).unwrap();
}

#[test]
fn test_detached() {
  let mut header: JwsHeader<Empty> = JwsHeader::new();

  header.set_kid("#my-key");

  let payload: Vec<u8> = vec![1, 2, 3, 4];
  let key = HS512.generate_key().unwrap();
  let signer = HS512.signer_from_bytes(key.as_ref()).unwrap();
  let serialized: String = Encoder::encode_compact_detached(&payload, &header, &signer).unwrap();
  assert_eq!(segment_count(&serialized), 2);

  let payload: Vec<u8> = encode_b64(payload).into_bytes();
  let verifier = HS512.verifier_from_bytes(key.as_ref()).unwrap();
  let decoder: Decoder = Decoder::with_algorithms(vec![HS512]);
  let deserialized: JwsHeader<Empty> = decoder
    .decode_compact_detached(&serialized, &payload, &verifier)
    .unwrap();
  assert_eq!(deserialized.alg(), JwsAlgorithm::HS512);
  assert_eq!(deserialized.kid().unwrap(), "#my-key");
}

#[test]
fn test_detached_unencoded() {
  let mut header: JwsHeader<Empty> = JwsHeader::new();

  header.set_kid("#my-key");
  header.set_b64(false);
  header.set_crit(vec!["b64"]);

  let payload: Vec<u8> = vec![1, 2, 3, 4];
  let key = HS512.generate_key().unwrap();
  let signer = HS512.signer_from_bytes(key.as_ref()).unwrap();
  let serialized: String = Encoder::encode_compact_detached(&payload, &header, &signer).unwrap();
  assert_eq!(segment_count(&serialized), 2);

  let verifier = HS512.verifier_from_bytes(key.as_ref()).unwrap();
  let decoder: Decoder = Decoder::with_algorithms(vec![HS512]);
  let deserialized: JwsHeader<Empty> = decoder
    .decode_compact_detached(&serialized, &payload, &verifier)
    .unwrap();
  assert_eq!(deserialized.alg(), JwsAlgorithm::HS512);
  assert_eq!(deserialized.kid().unwrap(), "#my-key");
}

#[test]
fn test_detached_is_truly_detached() {
  let mut header: JwsHeader<Empty> = JwsHeader::new();

  header.set_kid("#my-key");
  header.set_b64(false);
  header.set_crit(vec!["b64"]);

  let payload: Vec<u8> = vec![1, 2, 3, 4];
  let key = HS512.generate_key().unwrap();
  let signer = HS512.signer_from_bytes(key.as_ref()).unwrap();
  let serialized: String = Encoder::encode_compact_detached(&payload, &header, &signer).unwrap();
  assert_eq!(segment_count(&serialized), 2);

  let segments: Vec<&str> = serialized.split(".").collect();
  let modified: String = [segments[0], "", segments[2]].join(".");
  let verifier = HS512.verifier_from_bytes(key.as_ref()).unwrap();
  let decoder: Decoder = Decoder::with_algorithms(vec![HS512]);
  let deserialized: JwsHeader<Empty> = decoder
    .decode_compact_detached(&modified, &payload, &verifier)
    .unwrap();
  assert_eq!(deserialized.alg(), JwsAlgorithm::HS512);
  assert_eq!(deserialized.kid().unwrap(), "#my-key");
}

#[test]
#[should_panic = "CryptoError"]
fn test_detached_signature_invalid() {
  let header: JwsHeader<Empty> = JwsHeader::new();
  let payload: Vec<u8> = vec![1, 2, 3, 4];
  let key = HS512.generate_key().unwrap();
  let signer = HS512.signer_from_bytes(key.as_ref()).unwrap();
  let serialized: String = Encoder::encode_compact_detached(&payload, &header, &signer).unwrap();
  let segments: Vec<&str> = serialized.split(".").collect();
  let modified: String = [segments[0], segments[1], "my-signature"].join(".");
  let verifier = HS512.verifier_from_bytes(key.as_ref()).unwrap();
  let decoder: Decoder = Decoder::with_algorithms(vec![HS512]);
  let _: JwsHeader<Empty> = decoder
    .decode_compact_detached(&modified, &payload, &verifier)
    .unwrap();
}

#[test]
#[should_panic = "CryptoError"]
fn test_detached_payload_invalid() {
  let header: JwsHeader<Empty> = JwsHeader::new();
  let payload: Vec<u8> = vec![1, 2, 3, 4];
  let key = HS512.generate_key().unwrap();
  let signer = HS512.signer_from_bytes(key.as_ref()).unwrap();
  let serialized: String = Encoder::encode_compact_detached(&payload, &header, &signer).unwrap();
  let payload: Vec<u8> = vec![5, 6, 7, 8];
  let verifier = HS512.verifier_from_bytes(key.as_ref()).unwrap();
  let decoder: Decoder = Decoder::with_algorithms(vec![HS512]);
  let _: JwsHeader<Empty> = decoder
    .decode_compact_detached(&serialized, &payload, &verifier)
    .unwrap();
}
