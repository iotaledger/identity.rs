// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Url;

use crate::did::IotaDID;

const MAIN_NETWORK_NAME: &str = "main";
const TEST_NETWORK_NAME: &str = "test";

lazy_static! {
  static ref EXPLORER_MAIN: Url = Url::parse("https://explorer.iota.org/mainnet").unwrap();
  static ref EXPLORER_TEST: Url = Url::parse("https://explorer.iota.org/testnet").unwrap();
  static ref NODE_MAIN: Url = Url::parse("https://chrysalis-nodes.iota.org").unwrap();
  static ref NODE_TEST: Url = Url::parse("https://api.lb-0.testnet.chrysalis2.com").unwrap();
}

/// The Tangle network to use (`Mainnet` or `Testnet`).
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Network {
  Mainnet,
  Testnet,
}

impl Network {
  /// Parses the provided string to a `Network`.
  ///
  /// If the input is `"test"` then `Testnet` is returned, otherwise `Mainnet` is returned.
  pub fn from_name(string: &str) -> Self {
    match string {
      TEST_NETWORK_NAME => Self::Testnet,
      _ => Self::Mainnet,
    }
  }

  /// Returns the `Network` the `IotaDID` is associated with.
  pub fn from_did(did: &IotaDID) -> Self {
    Self::from_name(did.network())
  }

  /// Returns true if this network is the same network as the DID.
  pub fn matches_did(self, did: &IotaDID) -> bool {
    did.network() == self.as_str()
  }

  /// Returns the default node URL of the Tangle network.
  pub fn node_url(self) -> &'static Url {
    match self {
      Self::Mainnet => &*NODE_MAIN,
      Self::Testnet => &*NODE_TEST,
    }
  }

  /// Returns the web explorer URL of the Tangle network.
  pub fn explorer_url(self) -> &'static Url {
    match self {
      Self::Mainnet => &*EXPLORER_MAIN,
      Self::Testnet => &*EXPLORER_TEST,
    }
  }

  /// Returns the name of the network as a static `str`.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Mainnet => MAIN_NETWORK_NAME,
      Self::Testnet => TEST_NETWORK_NAME,
    }
  }
}

impl Default for Network {
  /// The default `Network` is the `Mainnet`.
  fn default() -> Self {
    Network::Mainnet
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_from_name() {
    assert_eq!(Network::from_name("test"), Network::Testnet);
    assert_eq!(Network::from_name("main"), Network::Mainnet);
    assert_eq!(Network::from_name("anything"), Network::Mainnet);
  }

  #[test]
  fn test_matches_did() {
    let did: IotaDID = IotaDID::new(b"").unwrap();
    assert!(Network::matches_did(Network::Mainnet, &did));
    assert!(!Network::matches_did(Network::Testnet, &did));

    let did: IotaDID = IotaDID::with_network(b"", "test").unwrap();
    assert!(Network::matches_did(Network::Testnet, &did));
    assert!(!Network::matches_did(Network::Mainnet, &did));
  }
}
