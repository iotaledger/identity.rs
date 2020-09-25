use libjose::crypto::PKey;
use libjose::crypto::Secret;
use libjose::jwa::HmacAlgorithm::*;
use libjose::jwa::HmacSigner;
use libjose::jwa::HmacVerifier;
use libjose::jws::JwsDecoder;
use libjose::jws::JwsEncoder;
use libjose::jws::JwsHeader;
use libjose::jws::JwsRawToken;
use std::str::from_utf8;

fn main() {
  // Create a JSON Web Signature JOSE header
  let header: JwsHeader = JwsHeader::new();
  let payload: &str = "hello world";

  // Generate a key for the `HS512` JSON Web Algorithm
  let pkey: PKey<Secret> = HS512.generate_key().unwrap();

  // Create a signer and verifier from the generated key
  let signer: HmacSigner = HS512.signer_from_bytes(&pkey).unwrap();
  let verifier: HmacVerifier = HS512.verifier_from_bytes(&pkey).unwrap();

  // Use the `JwsEncoder` helper to sign and encode the token
  let encoded: String = JwsEncoder::new()
    .encode_slice(payload, &header, &signer)
    .unwrap()
    .to_string()
    .unwrap();

  println!("Payload: {}", payload);
  println!("Encoded: {}", encoded);

  // Use the `JwsDecoder` helper to decode token WITHOUT deserializing the
  // claims.
  let decoded: JwsRawToken = JwsDecoder::new()
    .alg(HS512)
    .decode_token(&encoded, &verifier)
    .unwrap();

  let claims: &str = from_utf8(&decoded.claims).unwrap();

  assert_eq!(claims, payload);
}
