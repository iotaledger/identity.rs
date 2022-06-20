// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
//! Media type and file extension constants for DIDComm messages.
pub trait EnvelopeExt {
  const FEXT: &'static str;
  const MIME: &'static str;

  fn as_bytes(&self) -> &[u8];
}
