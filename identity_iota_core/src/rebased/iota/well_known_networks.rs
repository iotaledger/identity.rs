// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota_interaction::types::base_types::ObjectID;
use phf::phf_map;
use phf::Map;

use crate::NetworkName;

/// A Mapping `network_id` -> metadata needed by the library.
pub(crate) static IOTA_NETWORKS: Map<&str, IdentityNetworkMetadata> = phf_map! {
  "e678123a" => IdentityNetworkMetadata::new(
    Some("devnet"),
    &["0x6a976d3da90db5d27f8a0c13b3268a37e582b455cfc7bf72d6461f6e8f668823",
      "0x03242ae6b87406bd0eb5d669fbe874ed4003694c0be9c6a9ee7c315e6461a553"],
    "0x0x940ae1c2c48dade9ec01cc1eebab33ab6fecadda422ea18b105c47839fc64425",
  ),
  "2304aa97" => IdentityNetworkMetadata::new(
    Some("testnet"),
    &["0x3403da7ec4cd2ff9bdf6f34c0b8df5a2bd62c798089feb0d2ebf1c2e953296dc",
      "0x222741bbdff74b42df48a7b4733185e9b24becb8ccfbafe8eac864ab4e4cc555"],
    "0xaacb529c289aec9de2a474faaa4ef68b04632bb6a5d08372ca5b60e3df659f59",
  ),
};

/// `iota_identity` package information for a given network.
#[derive(Debug)]
pub(crate) struct IdentityNetworkMetadata {
  pub alias: Option<&'static str>,
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
  const fn new(alias: Option<&'static str>, pkgs: &'static [&'static str], migration_registry: &'static str) -> Self {
    assert!(!pkgs.is_empty());
    Self {
      alias,
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

  /// Returns a [`NetworkName`] if `alias` is set.
  pub(crate) fn network_alias(&self) -> Option<NetworkName> {
    self.alias.map(|alias| {
      NetworkName::try_from(alias).expect("an hardcoded network alias is valid (unless a dev messed it up)")
    })
  }
}

#[cfg(test)]
mod test {
  use identity_iota_interaction::IotaClientBuilder;

  use crate::rebased::client::IdentityClientReadOnly;

  #[tokio::test]
  async fn identity_client_connection_to_devnet_works() -> anyhow::Result<()> {
    let client = IdentityClientReadOnly::new(IotaClientBuilder::default().build_devnet().await?).await?;
    assert_eq!(client.network().as_ref(), "devnet");
    Ok(())
  }

  #[tokio::test]
  async fn identity_client_connection_to_testnet_works() -> anyhow::Result<()> {
    let client = IdentityClientReadOnly::new(IotaClientBuilder::default().build_testnet().await?).await?;
    assert_eq!(client.network().as_ref(), "testnet");
    Ok(())
  }
}
