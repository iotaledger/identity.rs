// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use fastcrypto::secp256r1::Secp256r1KeyPair;
use fastcrypto::secp256r1::Secp256r1PublicKeyAsBytes;
use fastcrypto::traits::ToFromBytes;
use identity_verification::jwk::Jwk;
use identity_verification::jws::JwsAlgorithm;
use p256::PublicKey;
use p256::SecretKey;

pub(crate) fn pk_to_jwk(pk: &Secp256r1PublicKeyAsBytes) -> Jwk {
  let jwk_str = PublicKey::from_sec1_bytes(&pk.0)
    .expect("valid secp256r1 pk")
    .to_jwk_string();
  let mut jwk: Jwk = serde_json::from_str(&jwk_str).expect("valid JWK encoded secp256r1");
  jwk.set_alg(JwsAlgorithm::ES256.name());
  jwk
}

pub(crate) fn jwk_to_keypair(jwk: &Jwk) -> anyhow::Result<Secp256r1KeyPair> {
  let sk = SecretKey::from_jwk_str(&serde_json::to_string(jwk)?)?;
  Ok(Secp256r1KeyPair::from_bytes(sk.to_bytes().as_ref()).expect("valid secp256r1 key"))
}
