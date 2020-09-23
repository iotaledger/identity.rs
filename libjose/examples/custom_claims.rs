use libjose::crypto::PKey;
use libjose::crypto::Secret;
use libjose::jwa::HmacAlgorithm::*;
use libjose::jwa::HmacSigner;
use libjose::jws::JwsHeader;
use libjose::jws::JwsToken;
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

  // See "examples/basic_jws.rs" for more information on the following:
  //

  let token: JwsToken<_, MyClaims> = JwsToken::new(header, claims);
  let pkey: PKey<Secret> = HS512.generate_key().unwrap();
  let signer: HmacSigner = HS512.signer_from_bytes(&pkey).unwrap();
  let encoded: String = token.encode_compact(&signer).unwrap();

  println!("Encoded: {}", encoded);
}
