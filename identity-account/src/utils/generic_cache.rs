// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::ops::Deref;
use hashbrown::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;

use crate::error::Error;
use crate::error::Result;

type Map<T> = HashMap<String, T>;

pub type GenericReadGuard<'a, T> = RwLockReadGuard<'a, Map<T>>;

pub type GenericWriteGuard<'a, T> = RwLockWriteGuard<'a, Map<T>>;

#[derive(Clone, Debug)]
pub struct GenericCache<T> {
  data: Arc<RwLock<Map<T>>>,
}

impl<T> GenericCache<T> {
  pub fn new() -> Self {
    Self {
      data: Arc::new(RwLock::new(Map::new())),
    }
  }

  pub fn read(&self) -> Result<GenericReadGuard<'_, T>> {
    self.data.read().map_err(|error| Error::RwLockReadPoisoned)
  }

  pub fn write(&self) -> Result<GenericWriteGuard<'_, T>> {
    self.data.write().map_err(|error| Error::RwLockWritePoisoned)
  }
}

impl<T> Deref for GenericCache<T> {
  type Target = RwLock<Map<T>>;

  fn deref(&self) -> &Self::Target {
    &self.data
  }
}
