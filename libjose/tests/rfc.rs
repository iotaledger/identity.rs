// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use libjose::jwe;
use libjose::jwe::JweHeader;
use libjose::jwk::EcxCurve;
use libjose::jwk::Jwk;
use libjose::jwk::JwkParamsOkp;
use libjose::jwk::JwkSet;
use libjose::jws;
use libjose::jws::JwsHeader;
use libjose::utils::diffie_hellman;
use libjose::utils::encode_b64;
use libjose::utils::Secret;
use serde_json::Value;

#[test]
fn test_rfc7515() {
  struct TestVector {
    deterministic: bool,
    header: &'static str,
    claims: &'static [u8],
    encoded: &'static [u8],
    private_key: &'static str,
  }

  static TVS: &[TestVector] = &include!("fixtures/rfc7515.rs");

  for tv in TVS {
    let header: JwsHeader = serde_json::from_str(tv.header).unwrap();
    let jwk: Jwk = serde_json::from_str(tv.private_key).unwrap();

    if tv.deterministic {
      let encoded: String = jws::Encoder::new()
        .recipient((&jwk, &header))
        .encode(tv.claims)
        .unwrap();

      assert_eq!(encoded.as_bytes(), tv.encoded);
    }

    let decoded: _ = jws::Decoder::new(&jwk).decode(tv.encoded).unwrap();

    assert_eq!(decoded.protected.unwrap(), header);
    assert_eq!(decoded.claims, tv.claims);
  }
}

#[test]
fn test_rfc7516() {
  struct TestVector {
    claims: &'static [u8],
    header: &'static str,
    recipient: &'static str,
    encoded: &'static str,
  }

  static TVS: &[TestVector] = &include!("fixtures/rfc7516.rs");

  for tv in TVS {
    let secret: Jwk = serde_json::from_str(tv.recipient).unwrap();
    let secret: Secret<'_> = Secret::Jwk(&secret);

    let decoded: jwe::Token = jwe::Decoder::new(secret).decode(tv.encoded.as_bytes()).unwrap();
    let header: JweHeader = serde_json::from_str(tv.header).unwrap();

    assert_eq!(decoded.0, header);
    assert_eq!(decoded.1, tv.claims);
  }
}

#[test]
fn test_rfc7517() {
  enum TestVector {
    KeySet { json: &'static str },
    Key { json: &'static str },
  }

  static TVS: &[TestVector] = &include!("fixtures/rfc7517.rs");

  for tv in TVS {
    match tv {
      TestVector::KeySet { json } => {
        let value: Value = serde_json::from_str(json).unwrap();
        let jwks: JwkSet = serde_json::from_str(json).unwrap();

        for (index, jwk) in jwks.iter().enumerate() {
          let ser: Value = serde_json::to_value(jwk).unwrap();
          assert_eq!(ser, value["keys"][index]);
        }
      }
      TestVector::Key { json } => {
        let value: Value = serde_json::from_str(json).unwrap();
        let jwk: Jwk = serde_json::from_str(json).unwrap();
        let ser: Value = serde_json::to_value(&jwk).unwrap();

        assert_eq!(ser, value);
      }
    }
  }
}

#[test]
fn test_rfc7518() {
  struct TestVector {
    alice_jwk: &'static str,
    bob_jwk: &'static str,
    header: &'static str,
    apu_bytes: &'static [u8],
    apv_bytes: &'static [u8],
    derived_key_b64: &'static str,
  }

  static TVS: &[TestVector] = &include!("fixtures/rfc7518.rs");

  for tv in TVS {
    let alice_jwk: Jwk = serde_json::from_str(tv.alice_jwk).unwrap();
    let bob_jwk: Jwk = serde_json::from_str(tv.bob_jwk).unwrap();
    let epk_jwk: Jwk = alice_jwk.to_public();

    let header: JweHeader = serde_json::from_str(tv.header).unwrap();
    assert_eq!(header.apu().unwrap(), encode_b64(tv.apu_bytes));
    assert_eq!(header.apv().unwrap(), encode_b64(tv.apv_bytes));
    assert_eq!(header.epk().unwrap(), &epk_jwk);

    let encryption_key: _ = jwe::Decoder::new(&bob_jwk)
      .ecdh_curve(bob_jwk.try_ec_curve().unwrap())
      .__test_decrypt_key(&header)
      .unwrap();

    assert_eq!(encode_b64(encryption_key), tv.derived_key_b64);
  }
}

#[test]
fn test_rfc7638() {
  struct TestVector {
    jwk_json: &'static str,
    thumbprint_b64: &'static str,
  }

  static TVS: &[TestVector] = &include!("fixtures/rfc7638.rs");

  for tv in TVS {
    let key: Jwk = serde_json::from_str(tv.jwk_json).unwrap();
    let kid: String = key.thumbprint_b64().unwrap();

    assert_eq!(kid, tv.thumbprint_b64);
  }
}

#[test]
fn test_rfc7797() {
  struct TestVector {
    detach: bool,
    header: &'static [u8],
    encoded: &'static [u8],
    payload: &'static [u8],
    public_key: &'static str,
  }

  static TVS: &[TestVector] = &include!("fixtures/rfc7797.rs");

  for tv in TVS {
    let header: JwsHeader = serde_json::from_slice(tv.header).unwrap();
    let jwk: Jwk = serde_json::from_str(tv.public_key).unwrap();

    let mut decoder: jws::Decoder = jws::Decoder::new(&jwk);

    if tv.detach {
      decoder = decoder.payload(tv.payload);
    }

    let decoded: _ = decoder.critical("b64").decode(tv.encoded).unwrap();

    assert_eq!(decoded.protected.unwrap(), header);
    assert_eq!(decoded.claims, tv.payload);
  }
}

#[test]
fn test_rfc8037_ed25519() {
  struct TestVector {
    private_jwk: &'static str,
    public_jwk: &'static str,
    thumbprint_b64: &'static str,
    header: &'static str,
    payload: &'static str,
    encoded: &'static str,
  }

  static TVS: &[TestVector] = &include!("fixtures/rfc8037_ed25519.rs");

  for tv in TVS {
    let secret: Jwk = serde_json::from_str(tv.private_jwk).unwrap();
    let public: Jwk = serde_json::from_str(tv.public_jwk).unwrap();

    assert_eq!(secret.thumbprint_b64().unwrap(), tv.thumbprint_b64);
    assert_eq!(public.thumbprint_b64().unwrap(), tv.thumbprint_b64);

    let header: JwsHeader = serde_json::from_str(tv.header).unwrap();
    let encoded: String = jws::Encoder::new()
      .recipient((&secret, &header))
      .encode(tv.payload.as_bytes())
      .unwrap();

    assert_eq!(encoded, tv.encoded);

    let decoded: jws::Token = jws::Decoder::new(&public).decode(encoded.as_bytes()).unwrap();

    assert_eq!(decoded.protected.unwrap(), header);
    assert_eq!(decoded.claims, tv.payload.as_bytes());
  }
}

#[test]
fn test_rfc8037_x25519() {
  struct TestVector {
    public_jwk: &'static str,
    public_key: &'static [u8],
    eph_public_jwk: &'static str,
    eph_public_key: &'static [u8],
    eph_secret_key: &'static [u8],
    z: &'static [u8],
  }

  static TVS: &[TestVector] = &include!("fixtures/rfc8037_x25519.rs");

  for tv in TVS {
    let public: Jwk = serde_json::from_str(tv.public_jwk).unwrap();

    assert_eq!(
      EcxCurve::X25519,
      public.try_okp_params().unwrap().try_ecx_curve().unwrap()
    );

    assert_eq!(
      tv.public_key,
      &Secret::Jwk(&public).to_x25519_public().unwrap().to_bytes()[..]
    );

    let eph_public: Jwk = serde_json::from_str(tv.eph_public_jwk).unwrap();

    assert_eq!(
      EcxCurve::X25519,
      eph_public.try_okp_params().unwrap().try_ecx_curve().unwrap()
    );

    assert_eq!(
      tv.eph_public_key,
      &Secret::Jwk(&eph_public).to_x25519_public().unwrap().to_bytes()[..]
    );

    let eph_secret: Jwk = Jwk::from_params(JwkParamsOkp {
      crv: EcxCurve::X25519.name().to_string(),
      x: encode_b64(tv.eph_public_key),
      d: Some(encode_b64(tv.eph_secret_key)),
    });

    assert_eq!(eph_public, eph_secret.to_public());
    assert_eq!(tv.z, diffie_hellman(EcxCurve::X25519, &public, &eph_secret).unwrap());
  }
}

#[test]
fn test_rfc8037_x448() {
  struct TestVector {
    public_jwk: &'static str,
    public_key: &'static [u8],
    eph_public_jwk: &'static str,
    eph_public_key: &'static [u8],
    eph_secret_key: &'static [u8],
    z: &'static [u8],
  }

  static TVS: &[TestVector] = &include!("fixtures/rfc8037_x448.rs");

  for tv in TVS {
    let public: Jwk = serde_json::from_str(tv.public_jwk).unwrap();

    assert_eq!(
      EcxCurve::X448,
      public.try_okp_params().unwrap().try_ecx_curve().unwrap()
    );

    assert_eq!(
      tv.public_key,
      &Secret::Jwk(&public).to_x448_public().unwrap().to_bytes()[..]
    );

    let eph_public: Jwk = serde_json::from_str(tv.eph_public_jwk).unwrap();

    assert_eq!(
      EcxCurve::X448,
      eph_public.try_okp_params().unwrap().try_ecx_curve().unwrap()
    );

    assert_eq!(
      tv.eph_public_key,
      &Secret::Jwk(&eph_public).to_x448_public().unwrap().to_bytes()[..]
    );

    let eph_secret: Jwk = Jwk::from_params(JwkParamsOkp {
      crv: EcxCurve::X448.name().to_string(),
      x: encode_b64(tv.eph_public_key),
      d: Some(encode_b64(tv.eph_secret_key)),
    });

    assert_eq!(eph_public, eph_secret.to_public());
    assert_eq!(tv.z, diffie_hellman(EcxCurve::X448, &public, &eph_secret).unwrap());
  }
}
