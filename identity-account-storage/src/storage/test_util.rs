// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub(crate) fn random_string() -> String {
  random_bytes().into_iter().map(char::from).collect::<String>()
}

pub(crate) fn random_bytes() -> [u8; 32] {
  let mut dest: [u8; 32] = Default::default();
  getrandom::getrandom(&mut dest).unwrap();
  dest
}
