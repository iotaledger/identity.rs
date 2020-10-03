use libjose::jwa::HmacAlgorithm::*;
use libjose::jwa::HmacKey;
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
  let key: HmacKey = HS512.generate_key().unwrap();

  // Create a signer and verifier from the generated key
  let signer: HmacSigner = key.to_signer();
  let verifier: HmacVerifier = key.to_verifier();

  // Use the `JwsEncoder` helper to sign and encode the token
  let encoded: String = JwsEncoder::new()
    .encode_slice(payload, &header, &signer)
    .unwrap()
    .to_string()
    .unwrap();

  println!("Payload: {}", payload);
  println!();

  println!("Encoded: {}", encoded);
  println!();

  // Use the `JwsDecoder` helper to decode token WITHOUT deserializing the
  // claims.
  let decoded: JwsRawToken = JwsDecoder::new()
    .alg(HS512)
    .decode_token(&encoded, &verifier)
    .unwrap();

  let claims: &str = from_utf8(&decoded.claims).unwrap();

  assert_eq!(claims, payload);
}
