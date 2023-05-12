// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use libjose::jwe::Decoder;
use libjose::jwe::Encoder;
use libjose::jwe::JweAlgorithm;
use libjose::jwe::JweAlgorithm::*;
use libjose::jwe::JweEncryption;
use libjose::jwe::JweFormat;
use libjose::jwe::JweHeader;
use libjose::jwe::Token;
use libjose::jwk::Jwk;

const __RSA: bool = cfg!(not(feature = "test-rsa-enc"));

const CLAIMS: &[u8] = b"libjose";

fn roundtrip(algorithm: JweAlgorithm, encryption: JweEncryption) -> Result<(), Box<dyn std::error::Error>> {
  let header: JweHeader = JweHeader::new(algorithm, encryption);

  let secret: Jwk = Jwk::random((algorithm, encryption))?;
  if secret.kty().name() == "oct" {
    // TODO: Make a proper fix for this.
    return Err(String::from("cannot call to_public when kty = oct").into());
  };
  let public: Jwk = secret.to_public();

  let secret2: Jwk = Jwk::random((algorithm, encryption))?;
  if secret.kty().name() == "oct" {
    // TODO: Make a proper fix for this.
    return Err(String::from("cannot call to_public when kty = oct").into());
  };
  let public2: Jwk = secret2.to_public();

  let mut encoder: Encoder = Encoder::new().protected(&header).secret(&secret2).recipient(&public);
  let mut decoder: Decoder = Decoder::new(&secret).public(&public2);

  let encoded: String = encoder.encode(CLAIMS)?;
  let decoded: Token = decoder.decode(encoded.as_bytes())?;

  assert_eq!(decoded.0.alg(), header.alg());
  assert_eq!(decoded.0.enc(), header.enc());
  assert_eq!(decoded.1, CLAIMS);

  encoder = encoder.format(JweFormat::General);
  decoder = decoder.format(JweFormat::General);

  let encoded: String = encoder.encode(CLAIMS)?;
  let decoded: Token = decoder.decode(encoded.as_bytes())?;

  assert_eq!(decoded.0.alg(), header.alg());
  assert_eq!(decoded.0.enc(), header.enc());
  assert_eq!(decoded.1, CLAIMS);

  encoder = encoder.format(JweFormat::Flatten);
  decoder = decoder.format(JweFormat::Flatten);

  let encoded: String = encoder.encode(CLAIMS)?;
  let decoded: Token = decoder.decode(encoded.as_bytes())?;

  assert_eq!(decoded.0.alg(), header.alg());
  assert_eq!(decoded.0.enc(), header.enc());
  assert_eq!(decoded.1, CLAIMS);

  Ok(())
}

#[test]
fn test_jwe_roundtrip() {
  for alg in JweAlgorithm::ALL {
    // skip unless opted-in - rsa is SLOWWWW
    if __RSA && matches!(alg, RSA1_5 | RSA_OAEP | RSA_OAEP_256 | RSA_OAEP_384 | RSA_OAEP_512) {
      continue;
    }

    for enc in JweEncryption::ALL {
      let result = roundtrip(*alg, *enc);
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
}
