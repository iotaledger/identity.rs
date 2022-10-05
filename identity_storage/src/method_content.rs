// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::NewMethodType;

pub enum MethodContent {
  Generate(NewMethodType),
  Private(NewMethodType, Vec<u8>),
  Public(NewMethodType, Vec<u8>),
}
