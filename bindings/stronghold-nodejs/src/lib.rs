// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![deny(clippy::all)]

use types::NapiKeyType;


#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi]
pub fn flip_keytype(algo: NapiKeyType) -> NapiKeyType {
  match algo {
    NapiKeyType::Ed25519 =>NapiKeyType::X25519,
    NapiKeyType::X25519 => NapiKeyType::Ed25519,
  }
}

mod types;