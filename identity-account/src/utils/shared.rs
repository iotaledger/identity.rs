// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::RwLock;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;

use crate::error::Error;
use crate::error::Result;

#[derive(Debug)]
pub struct Shared<T>(RwLock<T>);

impl<T> Shared<T> {
  pub fn new(data: T) -> Self {
    Self(RwLock::new(data))
  }

  pub fn read(&self) -> Result<RwLockReadGuard<'_, T>> {
    self.0.read().map_err(|_| Error::RwLockReadPoisoned)
  }

  pub fn write(&self) -> Result<RwLockWriteGuard<'_, T>> {
    self.0.write().map_err(|_| Error::RwLockWritePoisoned)
  }
}
