// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::LazyLock;

use anyhow::Context;
use iota_interaction::types::base_types::ObjectID;
use product_core::core_client::CoreClientReadOnly;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use tokio::sync::RwLock;
use tokio::sync::RwLockReadGuard;
use tokio::sync::RwLockWriteGuard;
use tokio::sync::TryLockError;

use crate::rebased::Error;

pub(crate) const MAINNET_CHAIN_ID: &str = "6364aad5";

type PackageRegistryLock = RwLockReadGuard<'static, PackageRegistry>;
type PackageRegistryLockMut = RwLockWriteGuard<'static, PackageRegistry>;

static IDENTITY_PACKAGE_REGISTRY: LazyLock<RwLock<PackageRegistry>> = LazyLock::new(|| {
  let move_lock_content = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/packages/iota_identity/Move.lock"));
  RwLock::new(PackageRegistry::from_move_lock_content(move_lock_content).expect("Move.lock exists and it's valid"))
});

pub(crate) async fn identity_package_registry() -> PackageRegistryLock {
  IDENTITY_PACKAGE_REGISTRY.read().await
}

pub(crate) fn try_identity_package_registry() -> Result<PackageRegistryLock, TryLockError> {
  IDENTITY_PACKAGE_REGISTRY.try_read()
}

pub(crate) fn blocking_identity_registry() -> PackageRegistryLock {
  IDENTITY_PACKAGE_REGISTRY.blocking_read()
}

pub(crate) async fn identity_package_registry_mut() -> PackageRegistryLockMut {
  IDENTITY_PACKAGE_REGISTRY.write().await
}

pub(crate) fn try_identity_package_registry_mut() -> Result<PackageRegistryLockMut, TryLockError> {
  IDENTITY_PACKAGE_REGISTRY.try_write()
}

pub(crate) fn blocking_identity_registry_mut() -> PackageRegistryLockMut {
  IDENTITY_PACKAGE_REGISTRY.blocking_write()
}

/// Network / Chain information.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct Env {
  pub chain_id: String,
  pub alias: Option<String>,
}

impl Env {
  /// Creates a new package's environment.
  pub(crate) fn new(chain_id: impl Into<String>) -> Self {
    Self {
      chain_id: chain_id.into(),
      alias: None,
    }
  }

  /// Creates a new package's environment with the given alias.
  pub(crate) fn new_with_alias(chain_id: impl Into<String>, alias: impl Into<String>) -> Self {
    Self {
      chain_id: chain_id.into(),
      alias: Some(alias.into()),
    }
  }
}

/// A published package's metadata for a certain environment.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Metadata {
  pub original_published_id: ObjectID,
  pub latest_published_id: ObjectID,
  #[serde(deserialize_with = "deserialize_u64_from_str")]
  pub published_version: u64,
}

impl Metadata {
  /// Create a new [Metadata] assuming a newly published package.
  pub(crate) fn from_package_id(package: ObjectID) -> Self {
    Self {
      original_published_id: package,
      latest_published_id: package,
      published_version: 1,
    }
  }
}

fn deserialize_u64_from_str<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
  D: Deserializer<'de>,
{
  use serde::de::Error;

  String::deserialize(deserializer)?.parse().map_err(D::Error::custom)
}

#[derive(Debug, Clone, Default)]
pub(crate) struct PackageRegistry {
  aliases: HashMap<String, String>,
  envs: HashMap<String, Metadata>,
}

impl PackageRegistry {
  /// Returns the package [Metadata] for a given `chain`.
  /// `chain` can either be a chain identifier or its alias.
  pub(crate) fn metadata(&self, chain: &str) -> Option<&Metadata> {
    let from_alias = || self.aliases.get(chain).and_then(|chain_id| self.envs.get(chain_id));
    self.envs.get(chain).or_else(from_alias)
  }

  /// Returns this package's latest version ID for a given chain.
  pub(crate) fn package_id(&self, chain: &str) -> Option<ObjectID> {
    self.metadata(chain).map(|meta| meta.latest_published_id)
  }

  /// Returns the alias of a given chain-id.
  pub(crate) fn chain_alias(&self, chain_id: &str) -> Option<&str> {
    self
      .aliases
      .iter()
      .find_map(|(alias, chain)| (chain == chain_id).then_some(alias.as_str()))
  }

  /// Adds or replaces this package's metadata for a given environment.
  pub(crate) fn insert_env(&mut self, env: Env, metadata: Metadata) {
    let Env { chain_id, alias } = env;

    if let Some(alias) = alias {
      self.aliases.insert(alias, chain_id.clone());
    }
    self.envs.insert(chain_id, metadata);
  }

  /// Merges another [PackageRegistry] into this one.
  pub(crate) fn join(&mut self, other: PackageRegistry) {
    self.aliases.extend(other.aliases);
    self.envs.extend(other.envs);
  }

  /// Creates a [PackageRegistry] from a Move.lock file.
  pub(crate) fn from_move_lock_content(move_lock: &str) -> anyhow::Result<Self> {
    let mut move_lock: toml::Table = move_lock.parse()?;

    move_lock
      .remove("env")
      .context("invalid Move.lock file: missing `env` table")?
      .as_table_mut()
      .map(std::mem::take)
      .context("invalid Move.lock file: `env` is not a table")?
      .into_iter()
      .try_fold(Self::default(), |mut registry, (alias, table)| {
        let toml::Value::Table(mut table) = table else {
          anyhow::bail!("invalid Move.lock file: invalid `env` table");
        };
        let chain_id: String = table
          .remove("chain-id")
          .context(format!("invalid Move.lock file: missing `chain-id` for env {alias}"))?
          .try_into()
          .context("invalid Move.lock file: invalid `chain-id`")?;

        let env = Env::new_with_alias(chain_id, alias.clone());
        let metadata = table
          .try_into()
          .context(format!("invalid Move.lock file: invalid env metadata for {alias}"))?;
        registry.insert_env(env, metadata);

        Ok(registry)
      })
  }
}

pub(crate) async fn identity_package_id<C>(client: &C) -> Result<ObjectID, Error>
where
  C: CoreClientReadOnly,
{
  let network = client.network_name().as_ref();
  identity_package_registry()
    .await
    .package_id(network)
    .ok_or_else(|| Error::InvalidConfig(format!("cannot find IdentityIota package ID for network {network}")))
}

#[cfg(test)]
mod tests {
  use iota_sdk::IotaClientBuilder;

  use crate::rebased::client::IdentityClientReadOnly;

  #[tokio::test]
  async fn can_connect_to_testnet() -> anyhow::Result<()> {
    let iota_client = IotaClientBuilder::default().build_testnet().await?;
    let _identity_client = IdentityClientReadOnly::new(iota_client).await?;

    Ok(())
  }

  #[tokio::test]
  async fn can_connect_to_devnet() -> anyhow::Result<()> {
    let iota_client = IotaClientBuilder::default().build_devnet().await?;
    let _identity_client = IdentityClientReadOnly::new(iota_client).await?;

    Ok(())
  }

  #[tokio::test]
  async fn can_connect_to_mainnet() -> anyhow::Result<()> {
    let iota_client = IotaClientBuilder::default().build_mainnet().await?;
    let _identity_client = IdentityClientReadOnly::new(iota_client).await?;

    Ok(())
  }
}
