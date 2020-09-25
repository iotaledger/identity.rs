use libjose::crypto::PKey;
use libjose::crypto::Secret;
use libjose::jwa::HmacAlgorithm::*;
use libjose::jwa::HmacSigner;
use libjose::jws::JwsHeader;
use libjose::jws::JwsEncoder;
use libjose::jwt::JwtClaims;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct MyClaims {
  claim1: String,
  claim2: Vec<u8>,
}

fn main() {
  // Create a JSON Web Signature JOSE header
  let mut header: JwsHeader = JwsHeader::new();

  // Manually set the algorithm property
  header.set_alg(HS512);

  // Create a set of registered JSON Web Token claims
  let mut claims: JwtClaims<MyClaims> = JwtClaims::new();

  // Any type can be set and retrieved as the custom claims
  //
  // Note: The type MUST implement `serde::Deserialize` and/or `serde::Serialize`
  // to have support for encoding/decoding.
  claims.set_custom(MyClaims {
    claim1: "hello".into(),
    claim2: b"world".to_vec(),
  });

  println!("Header: {:#?}", header);
  println!("Claims: {:#?}", claims);

  // Generate a key for the `HS512` JSON Web Algorithm
  let pkey: PKey<Secret> = HS512.generate_key().unwrap();

  // Create a signer from the generated key
  let signer: HmacSigner = HS512.signer_from_bytes(&pkey).unwrap();

  // Use the `JwsEncoder` helper to sign and encode the token
  let encoded: String = JwsEncoder::new()
    .encode(&claims, &header, &signer)
    .unwrap()
    .to_string()
    .unwrap();

  println!("Encoded: {}", encoded);
}
