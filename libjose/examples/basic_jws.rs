use libjose::jwk::Jwk;
use libjose::jws::JwsHeader;
use libjose::jws::JwsEncoder;
use libjose::jwt::JwtClaims;
use libjose::jwa::HmacAlgorithm::*;
use libjose::jwa::HmacSigner;

fn main() {
  let header: JwsHeader = JwsHeader::new();
  let claims: JwtClaims = JwtClaims::new();

  let jwk: serde_json::Value = serde_json::json!({
    "kty": "oct",
    "k": "AyM1SysPpbyDfgZld3umj1qzKObwVMkoqQ-EstJQLr_T-1qS0gZH75aKtMN3Yj0iPS4hcgUuTwjAzZr1Z9CAow",
  });

  let jwk: Jwk = serde_json::from_value(jwk).unwrap();
  let signer: HmacSigner = HS256.signer_from_jwk(&jwk).unwrap();

  let encoded: String = JwsEncoder::new()
    .encode(&claims, &header, &signer)
    .unwrap()
    .to_string()
    .unwrap();

  println!("Encoded: {}", encoded);
}
