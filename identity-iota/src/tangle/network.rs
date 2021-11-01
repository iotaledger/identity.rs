// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryFrom;
use core::fmt::Display;
use core::fmt::Formatter;
use core::ops::Deref;
use std::borrow::Cow;

use identity_core::common::Url;

use crate::did::IotaDID;
use crate::error::Error;
use crate::error::Result;

const NETWORK_NAME_MAIN: &str = "main";
const NETWORK_NAME_DEV: &str = "dev";

lazy_static! {
  static ref EXPLORER_MAIN: Url = Url::parse("https://explorer.iota.org/mainnet/identity-resolver").unwrap();
  static ref EXPLORER_DEV: Url = Url::parse("https://explorer.iota.org/devnet/identity-resolver/").unwrap();
  static ref NODE_MAIN: Url = Url::parse("https://chrysalis-nodes.iota.org").unwrap();
  static ref NODE_DEV: Url = Url::parse("https://api.lb-0.h.chrysalis-devnet.iota.cafe").unwrap();
}

/// The Tangle network to use ([`Mainnet`][Network::Mainnet] or [`Devnet`][Network::Devnet]).
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Network {
  #[serde(rename = "main")]
  Mainnet,
  #[serde(rename = "dev")]
  Devnet,
  Other {
    name: NetworkName,
    explorer_url: Option<Url>,
  },
}

impl Network {
  /// Parses the provided string to a [`Network`].
  ///
  /// The names `"main"` and `"dev"` are mapped to the well-known [`Mainnet`][Network::Mainnet]
  /// and [`Devnet`][Network::Devnet] networks respectively.
  ///
  /// Other inputs will return an instance of [`Other`][Network::Other] if the name is valid.
  /// It must match a part or all of the `networkId` returned in the nodeinfo from a node.
  /// For example, if the networkId is `"private-tangle"`, `"tangle"` can be used.
  ///
  /// Network names must comply with the IOTA DID Method spec, that is: be non-empty, at most
  /// 6 characters long, and only include alphanumeric characters `0-9` and `a-z`.
  ///
  /// See [`NetworkName`].
  pub fn try_from_name<S>(name: S) -> Result<Self>
  where
    // Allow String, &'static str, Cow<'static, str>, NetworkName
    S: AsRef<str> + Into<Cow<'static, str>>,
  {
    match name.as_ref() {
      NETWORK_NAME_MAIN => Ok(Self::Mainnet),
      NETWORK_NAME_DEV => Ok(Self::Devnet),
      _ => {
        // Accept any other valid string - validation is performed by NetworkName
        let network_name: NetworkName = NetworkName::try_from(name)?;
        Ok(Self::Other {
          name: network_name,
          explorer_url: None,
        })
      }
    }
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

  /// Returns true if this network is the same network as specified in the DID.
  pub fn matches_did(self, did: &IotaDID) -> bool {
    did.network_str() == self.name_str()
  }

  /// Returns the default node [`Url`] of the Tangle network.
  pub fn default_node_url(&self) -> Option<&'static Url> {
    match self {
      Self::Mainnet => Some(&*NODE_MAIN),
      Self::Devnet => Some(&*NODE_DEV),
      _ => None,
    }
  }

  /// Returns the web explorer [`Url`] of the Tangle network.
  pub fn explorer_url(&self) -> Option<&Url> {
    match self {
      Self::Mainnet => Some(&EXPLORER_MAIN),
      Self::Devnet => Some(&EXPLORER_DEV),
      Self::Other { explorer_url, .. } => explorer_url.as_ref(),
    }
  }

  /// Returns the web explorer URL of the given `message_id`.
  pub fn message_url(&self, message_id: &str) -> Result<Url> {
    let mut url = self.explorer_url().ok_or(Error::NoExplorerURLSet)?.clone();
    url
      .path_segments_mut()
      .map_err(|_| Error::InvalidExplorerURL)?
      .push("message")
      .push(message_id);
    Ok(url)
  }

  /// Returns the [`NetworkName`] of the network.
  pub fn name(&self) -> NetworkName {
    match self {
      Self::Mainnet => NetworkName(Cow::from(NETWORK_NAME_MAIN)),
      Self::Devnet => NetworkName(Cow::from(NETWORK_NAME_DEV)),
      Self::Other { name, .. } => name.clone(),
    }
  }

  /// Returns the name of the network.
  pub fn name_str(&self) -> &str {
    match self {
      Self::Mainnet => NETWORK_NAME_MAIN,
      Self::Devnet => NETWORK_NAME_DEV,
      Self::Other { name, .. } => name.as_ref(),
    }
  }
}

impl Default for Network {
  /// The default `Network` is the `Mainnet`.
  fn default() -> Self {
    Network::Mainnet
  }
}

/// Network name compliant with the IOTA DID method specification:
/// https://github.com/iotaledger/identity.rs/blob/dev/documentation/docs/specs/iota_did_method_spec.md
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[repr(transparent)]
pub struct NetworkName(Cow<'static, str>);

impl NetworkName {
  /// Creates a new [`NetworkName`] if the name passes validation.
  pub fn try_from<T>(name: T) -> Result<Self>
  where
    T: Into<Cow<'static, str>>,
  {
    let name_cow: Cow<'static, str> = name.into();
    Self::validate_network_name(&name_cow)?;
    Ok(Self(name_cow))
  }

  /// Validates whether a string is a spec-compliant IOTA DID [`NetworkName`].
  pub fn validate_network_name(name: &str) -> Result<()> {
    if name.is_empty() {
      return Err(Error::InvalidNetworkName);
    }

    if name.len() > 6 {
      return Err(Error::InvalidNetworkName);
    };

    if !name.chars().all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit()) {
      return Err(Error::InvalidNetworkName);
    }

    Ok(())
  }
}

impl AsRef<str> for NetworkName {
  fn as_ref(&self) -> &str {
    self.0.as_ref()
  }
}

impl From<NetworkName> for Cow<'static, str> {
  fn from(network_name: NetworkName) -> Self {
    network_name.0
  }
}

impl Deref for NetworkName {
  type Target = Cow<'static, str>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl TryFrom<&'static str> for NetworkName {
  type Error = Error;

  fn try_from(name: &'static str) -> Result<Self, Self::Error> {
    Self::try_from(Cow::Borrowed(name))
  }
}

impl TryFrom<String> for NetworkName {
  type Error = Error;

  fn try_from(name: String) -> Result<Self, Self::Error> {
    Self::try_from(Cow::Owned(name))
  }
}

impl Display for NetworkName {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    f.write_str(self.as_ref())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_from_name_standard_networks() {
    assert_eq!(Network::try_from_name(NETWORK_NAME_MAIN).unwrap(), Network::Mainnet);
    assert_eq!(Network::try_from_name(NETWORK_NAME_DEV).unwrap(), Network::Devnet);
  }

  #[test]
  fn test_from_name_types() {
    let static_str = "custom";
    assert!(Network::try_from_name(static_str).is_ok());

    let string = static_str.to_owned();
    assert!(Network::try_from_name(string.clone()).is_ok());

    let cow_owned = Cow::Owned(string);
    assert!(Network::try_from_name(cow_owned).is_ok());

    let cow_borrowed = Cow::Borrowed(static_str);
    assert!(Network::try_from_name(cow_borrowed).is_ok());
  }

  #[test]
  fn test_from_name() {
    // Valid
    assert_eq!(
      Network::try_from_name("6chars").unwrap(),
      Network::Other {
        name: NetworkName::try_from("6chars").unwrap(),
        explorer_url: None,
      }
    );

    // Must be non-empty
    assert!(matches!(
      Network::try_from_name("").unwrap_err(),
      Error::InvalidNetworkName
    ));

    // Must be <= 6 chars
    assert!(matches!(
      Network::try_from_name("7seven7").unwrap_err(),
      Error::InvalidNetworkName
    ));

    // Must only include 0-9, a-z
    assert!(matches!(
      Network::try_from_name("täst").unwrap_err(),
      Error::InvalidNetworkName
    ));
    assert!(matches!(
      Network::try_from_name(" ").unwrap_err(),
      Error::InvalidNetworkName
    ));
    assert!(matches!(
      Network::try_from_name("Test").unwrap_err(),
      Error::InvalidNetworkName
    ));
  }

  #[test]
  fn test_matches_did() {
    let did: IotaDID = IotaDID::new(b"").unwrap();
    assert!(Network::matches_did(Network::Mainnet, &did));
    assert!(!Network::matches_did(Network::Devnet, &did));

    let did: IotaDID = IotaDID::new_with_network(b"", "main").unwrap();
    assert!(Network::matches_did(Network::Mainnet, &did));
    assert!(!Network::matches_did(Network::Devnet, &did));

    let did: IotaDID = IotaDID::new_with_network(b"", "dev").unwrap();
    assert!(Network::matches_did(Network::Devnet, &did));
    assert!(!Network::matches_did(Network::Mainnet, &did));
  }

  #[test]
  fn test_explorer_url() {
    let mainnet = Network::Mainnet;
    assert!(mainnet.explorer_url().is_some());
    assert_eq!(mainnet.explorer_url().unwrap().as_str(), EXPLORER_MAIN.as_str());

    let devnet = Network::Devnet;
    assert!(devnet.explorer_url().is_some());
    assert_eq!(devnet.explorer_url().unwrap().as_str(), EXPLORER_DEV.as_str());

    let mut other = Network::try_from_name("atoi").unwrap();
    assert!(other.explorer_url().is_none());

    // Try setting a `cannot_be_a_base` url.
    assert!(matches!(
      other
        .set_explorer_url(Url::parse("data:text/plain,stuff").unwrap())
        .unwrap_err(),
      Error::InvalidExplorerURL
    ));

    let url = Url::parse("https://explorer.iota.org/devnet").unwrap();
    assert!(other.set_explorer_url(url.clone()).is_ok());
    assert_eq!(other.explorer_url().unwrap().clone(), url);
  }
}
