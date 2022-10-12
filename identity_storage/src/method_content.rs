// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::PrivateKey;
use identity_core::crypto::PublicKey;

// TODO: Impl Zeroize.
#[derive(Debug)]
pub enum MethodContent {
  Generate,
  Private(PrivateKey),
  Public(PublicKey),
}
