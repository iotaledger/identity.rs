// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use libjose::jwk::Jwk;
use libjose::jws::Decoder;
use libjose::jws::Encoder;
use libjose::jws::JwsAlgorithm;
use libjose::jws::JwsAlgorithm::*;
use libjose::jws::JwsFormat;
use libjose::jws::JwsHeader;
use libjose::jws::Token;

const __RSA: bool = cfg!(not(feature = "test-rsa-sig"));

const CLAIMS: &[u8] = b"libjose";

fn roundtrip(algorithm: JwsAlgorithm) -> Result<(), Box<dyn std::error::Error>> {
  let header: JwsHeader = JwsHeader::new(algorithm);
  let secret: Jwk = Jwk::random(algorithm)?;
  if secret.kty().name() == "oct" {
    // TODO: Make a proper fix for this.
    return Err(String::from("cannot call to_public when kty = oct").into());
  };

  let public: Jwk = secret.to_public();

  let mut encoder: Encoder<'_> = Encoder::new().recipient((&secret, &header));
  let mut decoder: Decoder<'_, '_> = Decoder::new(&public);

  let encoded: String = encoder.encode(CLAIMS)?;
  let decoded: Token<'_> = decoder.decode(encoded.as_bytes())?;

  assert_eq!(decoded.protected.unwrap(), header);
  assert_eq!(decoded.claims, CLAIMS);

  encoder = encoder.format(JwsFormat::General);
  decoder = decoder.format(JwsFormat::General);

  let encoded: String = encoder.encode(CLAIMS)?;
  let decoded: Token<'_> = decoder.decode(encoded.as_bytes())?;

  assert_eq!(decoded.protected.unwrap(), header);
  assert_eq!(decoded.claims, CLAIMS);

  encoder = encoder.format(JwsFormat::Flatten);
  decoder = decoder.format(JwsFormat::Flatten);

  let encoded: String = encoder.encode(CLAIMS)?;
  let decoded: Token<'_> = decoder.decode(encoded.as_bytes())?;

  assert_eq!(decoded.protected.unwrap(), header);
  assert_eq!(decoded.claims, CLAIMS);

  Ok(())
}

#[test]
fn test_jws_roundtrip() {
  for alg in JwsAlgorithm::ALL {
    // skip - not supported
    if matches!(alg, ES384 | ES512 | NONE) {
      continue;
    }

    // skip unless opted-in - rsa is SLOWWWW
    if __RSA && matches!(alg, RS256 | RS384 | RS512 | PS256 | PS384 | PS512) {
      continue;
    }

    let result = roundtrip(*alg);
    assert!(
      result.is_ok()
        || result
          .err()
          .unwrap()
          .to_string()
          .contains("cannot call to_public when kty = oct")
    );
  }
}
