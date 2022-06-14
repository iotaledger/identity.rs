// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryFrom;
use core::fmt::Display;
use core::fmt::Formatter;
use std::str::FromStr;

use identity_core::common::Url;
use identity_did::did::DID;
use identity_iota_core::tangle::MessageId;
use serde;
use serde::Deserialize;
use serde::Serialize;

use crate::error::Error;
use crate::error::Result;

lazy_static::lazy_static! {
  static ref EXPLORER_MAIN: ExplorerUrl =
    ExplorerUrl::new(Url::parse("https://explorer.iota.org/mainnet").unwrap()).unwrap();
  static ref EXPLORER_DEV: ExplorerUrl =
    ExplorerUrl::new(Url::parse("https://explorer.iota.org/devnet").unwrap()).unwrap();
}

/// A Tangle explorer URL with convenience functions for constructing URLs for viewing
/// published messages or IOTA DIDs.
///
/// # Example
///
/// ```
/// # use identity_iota_core::did::IotaDID;
/// # use identity_iota_client::tangle::ExplorerUrl;
/// let explorer = ExplorerUrl::mainnet();
/// let did = IotaDID::parse("did:iota:H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV")?;
/// assert_eq!(
///   explorer.resolver_url(&did)?,
///   "https://explorer.iota.org/mainnet/identity-resolver/did:iota:H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
/// );
/// # Ok::<(), identity_iota_client::Error>(())
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct ExplorerUrl(Url);

impl ExplorerUrl {
  /// Constructs a new Tangle explorer URL.
  ///
  /// Use [`ExplorerUrl::mainnet`] or [`ExplorerUrl::devnet`] unless using a private Tangle
  /// or local explorer.
  ///
  /// NOTE: does not validate whether this corresponds to an actual Tangle explorer.
  pub fn new(url: Url) -> Result<Self> {
    if url.cannot_be_a_base() {
      return Err(Error::InvalidExplorerURL);
    }
    Ok(Self(url))
  }

  /// Constructs a new Tangle explorer URL from a string.
  ///
  /// See [`ExplorerUrl::new`].
  ///
  /// # Example
  ///
  /// Point to a Tangle explorer deployed locally.
  /// ```
  /// # use identity_iota_client::tangle::ExplorerUrl;
  /// let explorer = ExplorerUrl::parse("http://127.0.0.1:8082/")?;
  /// # Ok::<(), identity_iota_client::Error>(())
  /// ```
  pub fn parse(url: &str) -> Result<Self> {
    let url: Url = Url::parse(url).map_err(|_| Error::InvalidExplorerURL)?;
    Self::new(url)
  }

  /// Returns the Tangle explorer URL for the mainnet.
  #[inline]
  pub fn mainnet() -> &'static ExplorerUrl {
    &EXPLORER_MAIN
  }

  /// Returns the Tangle explorer URL for the devnet.
  #[inline]
  pub fn devnet() -> &'static ExplorerUrl {
    &EXPLORER_DEV
  }

  /// Returns the web explorer URL of the given `message_id`.
  ///
  /// E.g. `https://explorer.iota.org/mainnet/message/<message_id>`
  pub fn message_url(&self, message_id: &MessageId) -> Result<Url> {
    let mut url: Url = self.0.clone();
    url
      .path_segments_mut()
      .map_err(|_| Error::InvalidExplorerURL)?
      .push("message")
      .push(&message_id.to_string());
    Ok(url)
  }

  /// Returns the web identity resolver URL for the given DID.
  ///
  /// E.g. `https://explorer.iota.org/mainnet/identity-resolver/<did>`
  pub fn resolver_url(&self, did: &impl DID) -> Result<Url> {
    let mut url: Url = self.0.clone();
    url
      .path_segments_mut()
      .map_err(|_| Error::InvalidExplorerURL)?
      .push("identity-resolver")
      .push(did.as_str());
    Ok(url)
  }
}

impl AsRef<str> for ExplorerUrl {
  fn as_ref(&self) -> &str {
    self.0.as_ref()
  }
}

impl TryFrom<Url> for ExplorerUrl {
  type Error = Error;

  fn try_from(url: Url) -> Result<Self, Self::Error> {
    Self::new(url)
  }
}

impl FromStr for ExplorerUrl {
  type Err = Error;

  fn from_str(url: &str) -> Result<Self, Self::Err> {
    Self::try_from(url)
  }
}

impl TryFrom<&str> for ExplorerUrl {
  type Error = Error;

  fn try_from(url: &str) -> Result<Self, Self::Error> {
    Self::parse(url)
  }
}

impl Display for ExplorerUrl {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    f.write_str(self.as_ref())
  }
}

#[cfg(test)]
mod tests {
  use identity_iota_core::did::IotaDID;

  use super::*;

  #[test]
  fn test_explorer_url() {
    let main_url_str: &str = "https://explorer.iota.org/mainnet";
    let dev_url_str: &str = "https://explorer.iota.org/devnet";
    let localhost: &str = "http://127.0.0.1:8082";

    // Valid new()
    assert_eq!(
      &ExplorerUrl::new(Url::parse(main_url_str).unwrap()).unwrap(),
      ExplorerUrl::mainnet()
    );
    assert_eq!(
      &ExplorerUrl::new(Url::parse(dev_url_str).unwrap()).unwrap(),
      ExplorerUrl::devnet()
    );
    assert!(ExplorerUrl::new(Url::parse(localhost).unwrap()).is_ok());

    // Valid parse()
    assert_eq!(&ExplorerUrl::parse(main_url_str).unwrap(), ExplorerUrl::mainnet());
    assert_eq!(&ExplorerUrl::parse(dev_url_str).unwrap(), ExplorerUrl::devnet());
    assert!(ExplorerUrl::parse(localhost).is_ok());

    // Try setting a `cannot_be_a_base` url.
    assert!(matches!(
      ExplorerUrl::new(Url::parse("data:text/plain,stuff").unwrap()).unwrap_err(),
      Error::InvalidExplorerURL
    ));
  }

  #[test]
  fn test_message_url() {
    let message_id: MessageId = MessageId::new([0u8; 32]);

    let explorer_main: &ExplorerUrl = ExplorerUrl::mainnet();
    assert_eq!(
      explorer_main.message_url(&message_id).unwrap(),
      format!("{}/{}/{}", explorer_main, "message", message_id)
    );

    let explorer_dev: &ExplorerUrl = ExplorerUrl::devnet();
    assert_eq!(
      explorer_dev.message_url(&message_id).unwrap(),
      format!("{}/{}/{}", explorer_dev, "message", message_id)
    );

    let localhost: &str = "http://127.0.0.1:8082";
    let other = ExplorerUrl::parse(localhost).unwrap();
    assert_eq!(
      other.message_url(&message_id).unwrap(),
      format!("{}/{}/{}", localhost, "message", message_id)
    );
  }

  #[test]
  fn test_resolver_url() {
    let did: IotaDID = IotaDID::parse("did:iota:H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV").unwrap();

    let explorer_main: &ExplorerUrl = ExplorerUrl::mainnet();
    assert_eq!(
      explorer_main.resolver_url(&did).unwrap(),
      format!("{}/{}/{}", explorer_main, "identity-resolver", did)
    );

    let explorer_dev: &ExplorerUrl = ExplorerUrl::devnet();
    assert_eq!(
      explorer_dev.resolver_url(&did).unwrap(),
      format!("{}/{}/{}", explorer_dev, "identity-resolver", did)
    );

    let localhost: &str = "http://127.0.0.1:8082";
    let other: ExplorerUrl = ExplorerUrl::parse(localhost).unwrap();
    assert_eq!(
      other.resolver_url(&did).unwrap(),
      format!("{}/{}/{}", localhost, "identity-resolver", did)
    );
  }
}
