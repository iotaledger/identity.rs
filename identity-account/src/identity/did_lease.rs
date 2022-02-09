// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;

/// A type that represents the permission to modify an identity.
///
/// Holds an `AtomicBool` that is set to `false` on drop, signifying
/// the release of the lease.
#[derive(Clone, Debug, Serialize)]
pub struct DIDLease(Arc<AtomicBool>);

impl DIDLease {
  pub fn new() -> Self {
    Self(Arc::new(AtomicBool::new(true)))
  }

  pub fn store(&self, value: bool) {
    self.0.store(value, Ordering::SeqCst);
  }

  pub fn load(&self) -> bool {
    self.0.load(Ordering::SeqCst)
  }
}

impl Drop for DIDLease {
  fn drop(&mut self) {
    self.store(false);
  }
}

impl Default for DIDLease {
  fn default() -> Self {
    Self::new()
  }
}

impl From<AtomicBool> for DIDLease {
  fn from(atomic_bool: AtomicBool) -> Self {
    DIDLease(Arc::new(atomic_bool))
  }
}
