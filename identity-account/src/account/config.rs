// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use crate::storage::Storage;

#[derive(Clone, Debug)]
pub struct AccountConfig {
  pub(crate) inner: Config,
  pub(crate) storage: Arc<dyn Storage>,
}

/// Top-level configuration for Identity [Account]s
#[derive(Clone, Debug)]
pub struct Config {
  pub(crate) autosave: AutoSave,
  pub(crate) autopublish: bool,
  pub(crate) dropsave: bool,
  pub(crate) testmode: bool,
  pub(crate) milestone: u32,
}

impl Config {
  const MILESTONE: u32 = 1;

  /// Creates a new default `Config`.
  pub fn new() -> Self {
    Self {
      autosave: AutoSave::Every,
      autopublish: true,
      dropsave: true,
      testmode: false,
      milestone: Self::MILESTONE,
    }
  }
}

impl AccountConfig {
  pub fn new(storage: impl Storage) -> Self {
    Self {
      inner: Config::new(),
      storage: Arc::new(storage),
    }
  }

  pub fn new_with_config(storage: impl Storage, config: Config) -> Self {
    Self {
      inner: config,
      storage: Arc::new(storage),
    }
  }

  /// Sets the account auto-save behaviour.
  /// - [`Every`][AutoSave::Every] => Save to storage on every update
  /// - [`Never`][AutoSave::Never] => Never save to storage when updating
  /// - [`Batch(n)`][AutoSave::Batch] => Save to storage after every `n` updates.
  ///
  /// Note that when [`Never`][AutoSave::Never] is selected, you will most
  /// likely want to set [`dropsave`][Self::dropsave] to `true`.
  ///
  /// Default: [`Every`][AutoSave::Every]
  pub fn set_autosave(&mut self, value: AutoSave) {
    self.inner.autosave = value;
  }

  pub fn autosave(&self) -> AutoSave {
      self.inner.autosave
  }

  /// Sets the account auto-publish behaviour.
  /// - `true` => publish to the Tangle on every DID document change
  /// - `false` => never publish automatically
  ///
  /// Default: `true`
  pub fn set_autopublish(&mut self, value: bool) {
    self.inner.autopublish = value;
  }

  pub fn autopublish(&self) -> bool {
      self.inner.autopublish
  }

  /// Save the account state on drop.
  /// If set to `false`, set [`autosave`][Self::autosave] to
  /// either [`Every`][AutoSave::Every] or [`Batch(n)`][AutoSave::Batch].
  ///
  /// Default: `true`
  pub fn set_dropsave(&mut self, value: bool) {
    self.inner.dropsave = value;
  }

  pub fn dropsave(&self) -> bool {
      self.inner.dropsave
  }

  /// Save a state snapshot every N actions.
  pub fn set_milestone(mut self, value: u32) {
    self.inner.milestone = value;
  }

  pub fn milestone(&self) -> u32 {
      self.inner.milestone
  }

  #[doc(hidden)]
  pub fn set_testmode(&mut self, value: bool) {
    self.inner.testmode = value;
  }

  #[doc(hidden)]
  pub fn testmode(&self) -> bool { 
      self.inner.testmode
  }

  pub fn storage(&self) -> &dyn Storage {
      self.storage.as_ref()
  }
}

impl Default for Config {
  fn default() -> Self {
    Self::new()
  }
}

/// Available auto-save behaviours.
#[derive(Clone, Copy, Debug)]
pub enum AutoSave {
  /// Never save
  Never,
  /// Save after every action
  Every,
  /// Save after every N actions
  Batch(usize),
}
