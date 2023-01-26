// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example custom_claims

#[macro_use]
extern crate serde;

use libjose::jwk::Jwk;
use libjose::jws::Encoder;
use libjose::jws::JwsAlgorithm;
use libjose::jws::JwsHeader;
use libjose::jwt::JwtClaims;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct MyClaims {
  claim1: String,
  claim2: Vec<char>,
}

fn main() {
  let header: JwsHeader = JwsHeader::new(JwsAlgorithm::HS256);
  let mut claims: JwtClaims<MyClaims> = JwtClaims::new();
  let secret: Jwk = Jwk::random(header.alg()).unwrap();

  claims.set_custom(MyClaims {
    claim1: "hello".into(),
    claim2: "world".chars().collect(),
  });

  println!("Header: {}", serde_json::to_string_pretty(&header).unwrap());
  println!("Claims: {}", serde_json::to_string_pretty(&claims).unwrap());
  println!("Secret: {}", serde_json::to_string_pretty(&secret).unwrap());

  let token: String = Encoder::new()
    .recipient((&secret, &header))
    .encode_serde(&claims)
    .unwrap();

  println!("Token: {token}");
}
