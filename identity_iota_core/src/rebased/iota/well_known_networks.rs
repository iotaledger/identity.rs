// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::base_types::ObjectID;
use phf::phf_map;
use phf::Map;

/// A Mapping `network_id` -> metadata needed by the library.
pub(crate) static IOTA_NETWORKS: Map<&str, IdentityNetworkMetadata> = phf_map! {
  // devnet
  "e678123a" => IdentityNetworkMetadata::new(
    &["0x156dfa0c4d4e576f5675de7d4bbe161c767947ffceefd7498cb39c406bc1cb67"],
    "0x0247da7f3b8708fc1d326f70153c01b7caf52a19a6f42dd3b868ac8777486b11",
  ),
  // testnet
  "2304aa97" => IdentityNetworkMetadata::new(
    &["0x7a67dd504eb1291958495c71a07d20985951648dd5ebf01ac921a50257346818"],
    "0xf1e20e6e3fa4de99ca269a0168f431dc459bc3a1ee5b76b426d5cf3094680483",
  ),
};

/// `iota_identity` package information for a given network.
#[derive(Debug)]
pub(crate) struct IdentityNetworkMetadata {
  /// `package[0]` is the current version, `package[1]`
  /// is the version before, and so forth.
  pub package: &'static [&'static str],
  pub migration_registry: &'static str,
}

/// Returns the [`IdentityNetworkMetadata`] for a given network, if any.
pub(crate) fn network_metadata(network_id: &str) -> Option<&'static IdentityNetworkMetadata> {
  IOTA_NETWORKS.get(network_id)
}

impl IdentityNetworkMetadata {
  const fn new(pkgs: &'static [&'static str], migration_registry: &'static str) -> Self {
    assert!(!pkgs.is_empty());
    Self {
      package: pkgs,
      migration_registry,
    }
  }

  /// Returns the latest `IotaIdentity` package ID on this network.
  pub(crate) fn latest_pkg_id(&self) -> ObjectID {
    self
      .package
      .first()
      .expect("a package was published")
      .parse()
      .expect("valid package ID")
  }

  /// Returns the ID for the `MigrationRegistry` on this network.
  pub(crate) fn migration_registry(&self) -> ObjectID {
    self.migration_registry.parse().expect("valid ObjectID")
  }
}

#[cfg(test)]
mod test {
  use iota_sdk::IotaClientBuilder;

  use crate::rebased::client::IdentityClientReadOnly;

  #[tokio::test]
  async fn identity_client_connection_to_devnet_works() -> anyhow::Result<()> {
    IdentityClientReadOnly::new(IotaClientBuilder::default().build_devnet().await?).await?;
    Ok(())
  }

  #[tokio::test]
  async fn identity_client_connection_to_testnet_works() -> anyhow::Result<()> {
    IdentityClientReadOnly::new(IotaClientBuilder::default().build_testnet().await?).await?;
    Ok(())
  }
}
