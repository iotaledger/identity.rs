// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::PrivateKey;
use identity_core::crypto::PublicKey;

use crate::MethodType1;

pub enum MethodContent {
  Generate(MethodType1),
  Private(MethodType1, PrivateKey),
  Public(MethodType1, PublicKey),
}
