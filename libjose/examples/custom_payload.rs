use libjose::crypto::PKey;
use libjose::crypto::Secret;
use libjose::jwa::HmacSigner;
use libjose::jwa::HmacVerifier;
use libjose::jwa::HmacAlgorithm::*;
use libjose::jws::JwsHeader;
use libjose::jws::Encoder;
use libjose::jws::Decoder;
use libjose::jws::JwsRawToken;
use std::str::from_utf8;

fn main() {
  // Create a JSON Web Signature JOSE header
  let header: JwsHeader = JwsHeader::new();
  let payload: &str = "hello world";

  //
  // See "examples/basic_jws.rs" for more information on the following:
  //

  let pkey: PKey<Secret> = HS512.generate_key().unwrap();
  let signer: HmacSigner = HS512.signer_from_bytes(&pkey).unwrap();
  let verifier: HmacVerifier = HS512.verifier_from_bytes(&pkey).unwrap();

  // Use the `Encoder` helper to encode a vector of bytes as the token claims.
  let encoded: String = Encoder::encode_compact(payload, &header, &signer).unwrap();

  // Use the `Decoder` helper to decode token WITHOUT deserializing the claims.
  let decoded: JwsRawToken = Decoder::decode_compact(encoded, &verifier).unwrap();
  let claims: &str = from_utf8(&decoded.claims).unwrap();

  println!("Encoded Payload: {}", payload);
  println!("Decoded Payload: {}", claims);

  assert_eq!(claims, payload);
}
