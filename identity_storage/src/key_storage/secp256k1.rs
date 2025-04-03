// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use fastcrypto::secp256k1::Secp256k1KeyPair;
use fastcrypto::secp256k1::Secp256k1PublicKeyAsBytes;
use fastcrypto::traits::ToFromBytes;
use identity_verification::jwk::Jwk;
use identity_verification::jws::JwsAlgorithm;
use k256::PublicKey;
use k256::SecretKey;

pub(crate) fn pk_to_jwk(pk: &Secp256k1PublicKeyAsBytes) -> Jwk {
  let jwk_str = PublicKey::from_sec1_bytes(&pk.0)
    .expect("valid secp256k1 pk")
    .to_jwk_string();
  let mut jwk: Jwk = serde_json::from_str(&jwk_str).expect("valid JWK encoded secp256k1");
  jwk.set_alg(JwsAlgorithm::ES256K.name());
  jwk
}

pub(crate) fn jwk_to_keypair(jwk: &Jwk) -> anyhow::Result<Secp256k1KeyPair> {
  let sk = SecretKey::from_jwk_str(&serde_json::to_string(jwk)?)?;
  Ok(Secp256k1KeyPair::from_bytes(sk.to_bytes().as_ref()).expect("valid secp256k1 key"))
}
