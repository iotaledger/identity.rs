// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub trait EnvelopeExt {
  const FEXT: &'static str;
  const MIME: &'static str;

  fn as_bytes(&self) -> &[u8];
}
