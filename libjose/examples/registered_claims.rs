use libjose::crypto::PKey;
use libjose::crypto::Secret;
use libjose::jwa::HmacAlgorithm::*;
use libjose::jwa::HmacSigner;
use libjose::jwa::HmacVerifier;
use libjose::jws::JwsDecoder;
use libjose::jws::JwsEncoder;
use libjose::jws::JwsHeader;
use libjose::jws::JwsToken;
use libjose::jwt::JwtClaims;
use std::convert::TryInto;
use std::time::SystemTime;

const HOUR: i64 = 60 * 60;

fn timestamp() -> i64 {
  SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH)
    .expect("Epoch Fail")
    .as_secs()
    .try_into()
    .expect("Time Fail")
}

fn main() {
  // Create a JSON Web Signature JOSE header
  let mut header: JwsHeader = JwsHeader::new();

  // Set a few properties
  header.set_alg(HS512);
  header.set_kid("my-key");
  header.set_nonce("some-unique-value");

  println!("JWS Header: {:?}", header);
  println!();

  // Create a set of registered JSON Web Token claims
  let mut claims: JwtClaims = JwtClaims::new();

  // Set a value for the issuer claim
  claims.set_iss("token-issuer");

  // Set a value for the subject claim
  claims.set_sub("token-subject");

  // Set values for the audience claim
  claims.set_aud(vec!["audience-a", "audience-b"]);

  // Set a value for the token id claim
  claims.set_jti("token-id:1234");

  // Get the current time as a unix timestamp
  let now: i64 = timestamp();

  // The time the token was issued
  claims.set_iat(now);

  // The time the token becomes valid
  claims.set_nbf(now + (6 * HOUR));

  // The time the token expires
  claims.set_exp(now + (12 * HOUR));

  println!("JWT Claims: {:?}", claims);
  println!();

  // Generate a "private key" for the `HS512` JSON Web Algorithm
  let pkey: PKey<Secret> = HS512.generate_key().unwrap();

  // Create a "signer" from raw bytes
  //
  // Note: Other formats MAY be supported and are algorithm-specific.
  // Examples include: Base64 - DER - PEM
  let signer: HmacSigner = HS512.signer_from_bytes(&pkey).unwrap();

  // Encode the JSON Web Token using the compact serialization format.
  let encoded: String = JwsEncoder::new()
    .encode(&claims, &header, &signer)
    .unwrap()
    .to_string()
    .unwrap();

  println!("Encoded: {}", encoded);
  println!();

  // Create a "verifier" from the SAME key as the signer (because HMAC)
  //
  // Note: Similar to the "signer", available formats are algorithm-specific.
  let verifier: HmacVerifier = HS512.verifier_from_bytes(&pkey).unwrap();

  // Decode the JSON Web Token with signature validation
  let decoded: JwsToken = JwsDecoder::new()
    .alg(HS512)
    .decode(&encoded, &verifier)
    .unwrap();

  println!("Decoded: {:?}", decoded);
  println!();
}
