// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;

use identity_core::common::Url;

use crate::did::IotaDID;
use crate::error::{Error, Result};

const MAIN_NETWORK_NAME: &str = "main";
const TEST_NETWORK_NAME: &str = "test";

lazy_static! {
  static ref EXPLORER_MAIN: Url = Url::parse("https://explorer.iota.org/mainnet").unwrap();
  static ref EXPLORER_TEST: Url = Url::parse("https://explorer.iota.org/testnet").unwrap();
  static ref NODE_MAIN: Url = Url::parse("https://chrysalis-nodes.iota.org").unwrap();
  static ref NODE_TEST: Url = Url::parse("https://api.lb-0.testnet.chrysalis2.com").unwrap();
}

/// The Tangle network to use ([`Mainnet`][Network::Mainnet] or [`Testnet`][Network::Testnet]).
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum Network {
  #[serde(rename = "main")]
  Mainnet,
  #[serde(rename = "test")]
  Testnet,
  Other {
    name: String,
    explorer_url: Option<Url>,
  },
}

impl Network {
  /// Parses the provided string to a [Network].
  ///
  /// The inputs `"test"` and `"main"` will be mapped to the well-known [Testnet][Network::Testnet]
  /// and [Mainnet][Network::Mainnet] variants, respectively.
  /// Other inputs will return an instance of [Other][Network::Other].
  ///
  /// Note that the empty string is not a valid network name, and that names have to be compliant
  /// with the IOTA DID Method spec, that is, be at most 6 characters long and only include
  /// characters `0-9` or `a-z`.
  pub fn from_name(string: &str) -> Result<Self> {
    match string {
      "" => Err(Error::InvalidNetworkName("name cannot be the empty string")),
      TEST_NETWORK_NAME => Ok(Self::Testnet),
      MAIN_NETWORK_NAME => Ok(Self::Mainnet),
      other => {
        Self::check_name_compliance(other)?;
        Ok(Self::Other {
          name: other.to_owned(),
          explorer_url: None,
        })
      }
    }
  }

  /// Checks if a string is a spec-compliant network name.
  fn check_name_compliance(string: &str) -> Result<()> {
    if string.len() > 6 {
      return Err(Error::InvalidNetworkName("name cannot exceed 6 characters"));
    };

    if !string.chars().all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit()) {
      return Err(Error::InvalidNetworkName(
        "name may only contain characters `0-9` and `a-z`",
      ));
    }

    Ok(())
  }

  /// Sets the explorer url if `self` is an `Other` variant.
  ///
  /// The `Url` needs to be a valid base url, i.e. `url.cannot_be_a_base()`
  /// must be false. An [InvalidExplorerUrl][Error::InvalidExplorerURL] is returned otherwise.
  pub fn set_explorer_url(&mut self, url: Url) -> Result<()> {
    if url.cannot_be_a_base() {
      return Err(Error::InvalidExplorerURL);
    }

    if let Self::Other { explorer_url, .. } = self {
      explorer_url.replace(url);
    }

    Ok(())
  }

  /// Returns the [Network] the [IotaDID] is associated with, if it is a valid one.
  pub fn try_from_did(did: &IotaDID) -> Result<Self> {
    did.network()
  }

  /// Returns true if this network is the same network as the DID.
  pub fn matches_did(self, did: &IotaDID) -> bool {
    did.network_str() == self.name()
  }

  /// Returns the default node URL of the Tangle network.
  pub fn default_node_url(&self) -> Option<&'static Url> {
    match self {
      Self::Mainnet => Some(&*NODE_MAIN),
      Self::Testnet => Some(&*NODE_TEST),
      _ => None,
    }
  }

  /// Returns the web explorer URL of the Tangle network.
  pub fn explorer_url(&self) -> Result<Url> {
    match self {
      Self::Mainnet => Ok(EXPLORER_MAIN.clone()),
      Self::Testnet => Ok(EXPLORER_TEST.clone()),
      Self::Other {
        explorer_url: Some(url),
        ..
      } => Ok(url.clone()),
      _ => Err(Error::NoExplorerURLSet),
    }
  }

  /// Returns the web explorer URL of the given `message`.
  pub fn message_url(&self, message_id: &str) -> Result<Url> {
    let mut url = self.explorer_url()?;
    // unwrap is safe, the explorer URL is always a valid base URL
    url.path_segments_mut().unwrap().push("message").push(message_id);
    Ok(url)
  }

  /// Returns the name of the network.
  pub fn name(&self) -> Cow<'static, str> {
    match self {
      Self::Mainnet => Cow::Borrowed(MAIN_NETWORK_NAME),
      Self::Testnet => Cow::Borrowed(TEST_NETWORK_NAME),
      Self::Other { name, .. } => Cow::Owned(name.clone()),
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
    assert_eq!(Network::from_name("test").unwrap(), Network::Testnet);
    assert_eq!(Network::from_name("main").unwrap(), Network::Mainnet);
    assert_eq!(
      Network::from_name("6chars").unwrap(),
      Network::Other {
        name: "6chars".to_owned(),
        explorer_url: None
      }
    );

    assert!(matches!(
      Network::from_name("7seven7").unwrap_err(),
      Error::InvalidNetworkName("name cannot exceed 6 characters")
    ));

    assert!(matches!(
      Network::from_name("täst").unwrap_err(),
      Error::InvalidNetworkName("name may only contain characters `0-9` and `a-z`")
    ));

    assert!(matches!(
      Network::from_name(" ").unwrap_err(),
      Error::InvalidNetworkName("name may only contain characters `0-9` and `a-z`")
    ));

    assert!(matches!(
      Network::from_name("").unwrap_err(),
      Error::InvalidNetworkName("name cannot be the empty string")
    ));
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

  #[test]
  fn test_explorer_url() {
    let testnet = Network::Testnet;

    assert!(testnet.explorer_url().is_ok());

    let mut other = Network::from_name("atoi").unwrap();

    assert!(matches!(other.explorer_url().unwrap_err(), Error::NoExplorerURLSet));

    // Try setting a `cannot_be_a_base` url.
    assert!(matches!(
      other
        .set_explorer_url(Url::parse("data:text/plain,stuff").unwrap())
        .unwrap_err(),
      Error::InvalidExplorerURL
    ));

    let url = Url::parse("https://explorer.iota.org/testnet").unwrap();

    assert!(other.set_explorer_url(url.clone()).is_ok());

    assert_eq!(other.explorer_url().unwrap(), url);
  }
}
