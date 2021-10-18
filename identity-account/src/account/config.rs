// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use identity_iota::tangle::ClientMap;

use crate::storage::{MemStore, Storage};

/// Configuration for [`Account`][crate::account::Account]s
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

  /// Creates a new default [`Config`].
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

/// A wrapper that holds configuration for an account instantiation.
/// The setup implements `Clone`, so that multiple [`Account`][crate::account::Account]s can be created
/// from the same setup. [`Storage`] and [`ClientMap`] are shared among those accounts,
/// while the [`Config`] is unique to every account.
#[derive(Clone, Debug)]
pub struct AccountSetup {
  pub(crate) config: Config,
  pub(crate) storage: Arc<dyn Storage>,
  pub(crate) client_map: Arc<ClientMap>,
}

impl Default for AccountSetup {
  fn default() -> Self {
    Self::new(Arc::new(MemStore::new()))
  }
}

impl AccountSetup {
  /// Create a new setup from the given [`Storage`] implementation
  /// and with defaults for [`Config`] and [`ClientMap`].
  pub fn new(storage: Arc<dyn Storage>) -> Self {
    Self {
      config: Config::new(),
      storage,
      client_map: Arc::new(ClientMap::new()),
    }
  }

  /// Create a new setup from the given [`Storage`] implementation,
  /// as well as optional [`Config`] and [`ClientMap`].
  /// If `None` is passed, the defaults will be used.
  pub fn new_with_options(
    storage: Arc<dyn Storage>,
    config: Option<Config>,
    client_map: Option<Arc<ClientMap>>,
  ) -> Self {
    Self {
      config: config.unwrap_or_default(),
      storage,
      client_map: client_map.unwrap_or_else(|| Arc::new(ClientMap::new())),
    }
  }

  /// Set the [`Config`] for this setup.
  pub fn config(mut self, value: Config) -> Self {
    self.config = value;
    self
  }
}

impl Config {
  /// Sets the account auto-save behaviour.
  /// - [`Every`][AutoSave::Every] => Save to storage on every update
  /// - [`Never`][AutoSave::Never] => Never save to storage when updating
  /// - [`Batch(n)`][AutoSave::Batch] => Save to storage after every `n` updates.
  ///
  /// Note that when [`Never`][AutoSave::Never] is selected, you will most
  /// likely want to set [`dropsave`][Self::dropsave] to `true`.
  ///
  /// Default: [`Every`][AutoSave::Every]
  pub fn autosave(mut self, value: AutoSave) -> Self {
    self.autosave = value;
    self
  }

  /// Sets the account auto-publish behaviour.
  /// - `true` => publish to the Tangle on every DID document change
  /// - `false` => never publish automatically
  ///
  /// Default: `true`
  pub fn autopublish(mut self, value: bool) -> Self {
    self.autopublish = value;
    self
  }

  /// Save the account state on drop.
  /// If set to `false`, set [`autosave`][Self::autosave] to
  /// either [`Every`][AutoSave::Every] or [`Batch(n)`][AutoSave::Batch].
  ///
  /// Default: `true`
  pub fn dropsave(mut self, value: bool) -> Self {
    self.dropsave = value;
    self
  }

  /// Save a state snapshot every N actions.
  pub fn milestone(mut self, value: u32) -> Self {
    self.milestone = value;
    self
  }

  #[doc(hidden)]
  pub fn testmode(mut self, value: bool) -> Self {
    self.testmode = value;
    self
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
