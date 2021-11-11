// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::did::CoreDIDUrl;
use core::convert::AsRef;
use identity_core::common::Url;

/// A trait for comparing types only by a certain key.
pub trait KeyComparable {
  type Key: PartialEq + ?Sized;

  fn as_key(&self) -> &Self::Key;
}

impl<T: AsRef<CoreDIDUrl>> KeyComparable for T {
  type Key = CoreDIDUrl;

  fn as_key(&self) -> &Self::Key {
    self.as_ref()
  }
}

impl KeyComparable for Url {
  type Key = Url;

  fn as_key(&self) -> &Self::Key {
    self
  }
}
