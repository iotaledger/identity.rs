// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Url;

use crate::did::DID;

lazy_static! {
  static ref EXPLORER_MAIN: Url = Url::parse("https://explorer.iota.org/chrysalis").unwrap();
  static ref EXPLORER_TEST: Url = Url::parse("https://explorer.iota.org/chrysalis").unwrap();
  static ref NODE_MAIN: Url = Url::parse("https://api.lb-0.testnet.chrysalis2.com:443").unwrap();
  static ref NODE_TEST: Url = Url::parse("https://api.lb-0.testnet.chrysalis2.com:443").unwrap();
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Network {
  Mainnet,
  Testnet,
}

impl Network {
  pub fn from_name(string: &str) -> Self {
    match string {
      "test" => Self::Testnet,
      _ => Self::Mainnet,
    }
  }

  pub fn matches_did(self, did: &DID) -> bool {
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
      Self::Mainnet => "main",
      Self::Testnet => "test",
    }
  }
}

impl Default for Network {
  fn default() -> Self {
    Network::Mainnet
  }
}

impl<'a> From<&'a DID> for Network {
  fn from(other: &'a DID) -> Self {
    Self::from_name(other.network())
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
    let did: DID = DID::new(b"").unwrap();
    assert!(Network::matches_did(Network::Mainnet, &did));
    assert!(!Network::matches_did(Network::Testnet, &did));

    let did: DID = DID::with_network(b"", "test").unwrap();
    assert!(Network::matches_did(Network::Testnet, &did));
    assert!(!Network::matches_did(Network::Mainnet, &did));
  }
}
