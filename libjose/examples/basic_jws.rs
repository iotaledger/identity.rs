// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example basic_jws

use libjose::jwk::Jwk;
use libjose::jws::Encoder;
use libjose::jws::JwsAlgorithm;
use libjose::jws::JwsHeader;
use libjose::jwt::JwtClaims;

fn main() {
  let header: JwsHeader = JwsHeader::new(JwsAlgorithm::HS256);
  let claims: JwtClaims = JwtClaims::new();
  let secret: Jwk = Jwk::random(header.alg()).unwrap();

  println!("Header: {}", serde_json::to_string_pretty(&header).unwrap());
  println!("Claims: {}", serde_json::to_string_pretty(&claims).unwrap());
  println!("Secret: {}", serde_json::to_string_pretty(&secret).unwrap());

  let token: String = Encoder::new()
    .recipient((&secret, &header))
    .encode_serde(&claims)
    .unwrap();

  println!("Token: {token}");
}
