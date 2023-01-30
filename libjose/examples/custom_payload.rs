// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example custom_payload

use core::str;

use libjose::jwk::Jwk;
use libjose::jws::Decoder;
use libjose::jws::Encoder;
use libjose::jws::JwsAlgorithm;
use libjose::jws::JwsHeader;
use libjose::jws::Token;

fn main() {
  let header: JwsHeader = JwsHeader::new(JwsAlgorithm::HS256);
  let claims: &str = "hello world";
  let secret: Jwk = Jwk::random(header.alg()).unwrap();

  println!("Header: {}", serde_json::to_string_pretty(&header).unwrap());
  println!("Claims: {claims:?}");
  println!("Secret: {}", serde_json::to_string_pretty(&secret).unwrap());

  let token: String = Encoder::new()
    .recipient((&secret, &header))
    .encode(claims.as_bytes())
    .unwrap();

  println!("Token: {token}");

  let decoded: Token<'_> = Decoder::new(&secret).decode(token.as_bytes()).unwrap();
  let decoded: &str = str::from_utf8(&decoded.claims).unwrap();

  println!("Decoded: {decoded:?}");

  assert_eq!(claims, decoded);
}
