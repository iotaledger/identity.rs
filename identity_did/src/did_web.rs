//TODO: Web - WebDID

use std::{fmt::{Display, Formatter}, str::FromStr};

use identity_core::common::Url;
use crate::{CoreDID, Error, DID};
// use ref_cast::{ref_cast_custom, RefCastCustom};
use ::serde::{Deserialize, Serialize};
use crate::Error as DIDError;

/// Alias for a `Result` with the error type [`DIDError`].
type Result<T> = std::result::Result<T, DIDError>;

/// A DID conforming to the Web DID method specification.
///
/// This is a thin wrapper around the [`DID`][`CoreDID`] type from the
/// [`identity_did`][`identity_did`] crate.
#[derive(Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize/* , RefCastCustom*/)]
#[repr(transparent)]
#[serde(into = "CoreDID", try_from = "CoreDID")]
pub struct WebDID(CoreDID);

impl WebDID {
  /// The URL scheme for Decentralized Identifiers.
  pub const SCHEME: &'static str = CoreDID::SCHEME;

  /// The IOTA DID method name (`"iota"`).
  pub const METHOD: &'static str = "web";

  /// Create a new valid Web DID.
  pub fn new(url: &str) -> Result<Self> {
    let parsed_url: Url = Url::parse(url).map_err(|_| Error::Other("Not a valid Url"))?;
    
    // Extract the domain and path
    if let Some(domain) = parsed_url.domain() {

      let port = parsed_url.port().map_or(String::new(), |p| format!("%3a{}", p));

      let path = parsed_url.path_segments().map_or(String::new(), |p| {
        format!("{}{}", ":", p.into_iter().collect::<Vec<&str>>().join(":"))
      });

      let did_web = format!("did:{}:{}{}{}", Self::METHOD, domain, port, path);
      println!("DID Web: {}", did_web);
      let core_did = CoreDID::parse(did_web).map_err(|_| Error::Other("Cannot convert to CoreDID"))?;
      println!("{}",core_did);
      Ok(Self(core_did))

    } else {
        return Err(Error::InvalidMethodId);
    }
  }

  /// Parses an [`WebDID`] from the given `input`.
  ///
  /// # Errors
  ///
  /// Returns `Err` if the input does not conform to the [`WebDID`] specification.
  pub fn parse(input: impl AsRef<str>) -> Result<Self> {
    CoreDID::parse(input.as_ref().to_lowercase()).and_then(Self::try_from_core)
  }

  /// Converts a [`WebDID`] to a [`Url`]
  pub fn to_url(&self) -> Url {
    //This is safe because if i have constructed a WebDID I already know it is valid
    WebDID::check_validity(self).unwrap()
  }


  /// Converts a [`CoreDID`] to a [`WebDID`].
  ///
  /// # Errors
  ///
  /// Returns `Err` if the input does not conform to the [`IotaDID`] specification.
  pub fn try_from_core(did: CoreDID) -> Result<Self> {
    Self::check_validity(&did)?;
    Ok(Self(did))
  }

  // ===========================================================================
  // Validation
  // ===========================================================================

  /// Checks if the given `DID` is syntactically valid according to the [`WebDID`] method specification.
  ///
  /// # Errors
  ///
  /// Returns the corresponding [`Url`] or `Err` if the input is not a syntactically valid [`WebDID`].
  pub fn check_validity<D: DID>(did: &D) -> Result<Url> {
    Self::check_method(did)
      .and_then(|_| Self::check_method_id(did))
  }

  /// Returns a `bool` indicating if the given `DID` is valid according to the
  /// [`WebDID`] method specification.
  ///
  /// Equivalent to `WebDID::check_validity(did).is_ok()`.
  pub fn is_valid(did: &CoreDID) -> bool {
    Self::check_validity(did).is_ok()
  }


  // ===========================================================================
  // Helpers
  // ===========================================================================

  /// Checks if the given `DID` has a valid [`WebDID`] `method` (i.e. `"web"`).
  ///
  /// # Errors
  ///
  /// Returns `Err` if the input represents another method.
  fn check_method<D: DID>(did: &D) -> Result<()> {
    (did.method() == Self::METHOD)
      .then_some(())
      .ok_or(DIDError::InvalidMethodName)
  }

  /// Checks if the given `DID` has a valid [`IotaDID`] `method_id`.
  ///
  /// # Errors
  ///
  /// Returns `Err` if the input does not have a [`IotaDID`] compliant method id.
  fn check_method_id<D: DID>(did: &D) -> Result<Url> {
    let (domain, port, path) = Self::denormalized_components(did.method_id());

    let port = port.map(|p| u16::from_str(&p))
    .map_or(Ok(None), |r| r.map(Some)
    .map_err(|_| Error::InvalidMethodId))?;

    let mut url = Url::parse(&format!("https://{}", domain))
    .map_err(|_| Error::InvalidMethodId)?;

    url.set_port(port).map_err(|_| Error::InvalidMethodId)?;

    path.and_then(|p| Some(url.set_path(&p)));

    url.domain().ok_or(Error::InvalidMethodId)?;

    Ok(url)
  }

  // did:web:cybersecurity-links.github.io%3A3000:did-web-server:.well-known:did.json

  /// cybersecurity-links.github.io%3A3000:did-web-server:.well-known:did.json -> https:://cybersecurity-links.github.io:3000/did-web-server/.well-known/did.json
  #[inline(always)]
  fn denormalized_components(input: &str) -> (String, Option<String>, Option<String>) {

    match input.find("%3a") {
      Some(i) => {
        let (domain, tail) = input.split_at(i);
        match tail.find(":") {
          Some(i) => {
            let (port, path) = tail.split_at(i);
            (domain.to_owned(), Some(port[3..].to_owned()), Some(path.replace(":", "/")))
          },
          None => (domain.to_owned(), Some(tail[3..].to_owned()), None),
        }
      
      },
      None => {
        match input.find(":") {
          Some(i) => {
            let (domain, path) = input.split_at(i);
            (domain.to_owned(), None, Some(path.replace(":", "/")))
          },
          None => (input.to_owned(), None, None),
        }
      }
    }
      
  }

}

impl From<WebDID> for CoreDID {
  fn from(id: WebDID) -> Self {
    id.0
  }
}

impl TryFrom<CoreDID> for WebDID {
  type Error = DIDError;

  fn try_from(value: CoreDID) -> std::result::Result<Self, Self::Error> {
    Self::try_from_core(value)
  }
}

impl FromStr for WebDID {
  type Err = DIDError;

  fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
    Self::parse(s)
  }
}

impl From<WebDID> for String {
  fn from(did: WebDID) -> Self {
    did.into_string()
  }
}


impl TryFrom<&str> for WebDID {
  type Error = DIDError;

  fn try_from(other: &str) -> std::result::Result<Self, Self::Error> {
    Self::parse(other)
  }
}

impl TryFrom<String> for WebDID {
  type Error = DIDError;

  fn try_from(other: String) -> std::result::Result<Self, Self::Error> {
    Self::parse(other)
  }
}

impl Display for WebDID {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl AsRef<CoreDID> for WebDID {
  fn as_ref(&self) -> &CoreDID {
    &self.0
  }
}