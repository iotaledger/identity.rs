// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;

use serde::Deserialize;
use serde::Serialize;

use crate::error::Error;
use crate::error::Result;

#[derive(Default, Deserialize, Serialize)]
pub struct Shared<T>(RwLock<T>);

impl<T> Shared<T> {
  pub fn new(data: T) -> Self {
    Self(RwLock::new(data))
  }

  pub fn read(&self) -> Result<RwLockReadGuard<'_, T>> {
    self.0.read().map_err(|_| Error::SharedReadPoisoned)
  }

  pub fn write(&self) -> Result<RwLockWriteGuard<'_, T>> {
    self.0.write().map_err(|_| Error::SharedWritePoisoned)
  }
}

impl<T: Debug> Debug for Shared<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    Debug::fmt(&self.0, f)
  }
}
