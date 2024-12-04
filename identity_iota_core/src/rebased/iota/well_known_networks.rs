use iota_sdk::types::base_types::ObjectID;
use phf::{phf_map, Map};

/// A Mapping `network_id` -> metadata needed by the library.
pub(crate) static IOTA_NETWORKS: Map<&str, IdentityNetworkMetadata> = phf_map! {
  // devnet
  "e678123a" => IdentityNetworkMetadata::new(
    &["0x156dfa0c4d4e576f5675de7d4bbe161c767947ffceefd7498cb39c406bc1cb67"],
    "0x0247da7f3b8708fc1d326f70153c01b7caf52a19a6f42dd3b868ac8777486b11",
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
mod tests {
  use std::ops::Deref;

  use crate::rebased::{client::IdentityClientReadOnly, migration::get_identity};

  #[tokio::test]
  async fn devnet_did_has_right_network_name() -> anyhow::Result<()> {
    let iota_client = iota_sdk::IotaClientBuilder::default().build_devnet().await?;
    let identity_client = IdentityClientReadOnly::new(iota_client).await?;
    let identity = get_identity(
      &identity_client,
      "0x867b7b3ff149e78216de81339b4d717696ce3089d22fc58b3eeb3c18f1778dfc".parse()?,
    )
    .await?
    .expect("identity exists on-chain");

    assert_eq!(identity.deref().id().network_str(), identity_client.network().as_ref());

    Ok(())
  }
}
