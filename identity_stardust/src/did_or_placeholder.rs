// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use identity_core::common::KeyComparable;
use identity_did::did::BaseDIDUrl;
use identity_did::did::CoreDID;
use identity_did::did::DIDError;
use identity_did::did::DIDUrl;
use identity_did::did::DID;
use serde::Deserialize;
use serde::Serialize;

lazy_static::lazy_static! {
  pub static ref PLACEHOLDER_DID: CoreDID =
  CoreDID::parse("did:0:0").unwrap();
}

/// A [`CoreDID`] or a placeholder.
#[derive(Debug, Hash, Clone, PartialEq, PartialOrd, Ord, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DIDOrPlaceholder {
  #[serde(with = "placeholder_serde")]
  Placeholder,
  Core(CoreDID),
}

impl DIDOrPlaceholder {
  /// Returns the contained [`CoreDID`] or computes it from a closure if the variant is `Placeholder`.
  pub fn unwrap_or_else<F>(self, f: F) -> CoreDID
  where
    F: FnOnce() -> CoreDID,
  {
    match self {
      DIDOrPlaceholder::Placeholder => f(),
      DIDOrPlaceholder::Core(did) => did,
    }
  }

  /// Creates a new [`DIDOrPlaceholder`] from a `did`.
  ///
  /// If `did` matches `original_did`, the `Placeholder` variant is used.
  pub fn new(did: CoreDID, original_did: &CoreDID) -> Self {
    if &did == original_did {
      DIDOrPlaceholder::Placeholder
    } else {
      DIDOrPlaceholder::Core(did)
    }
  }
}

impl DID for DIDOrPlaceholder {
  fn scheme(&self) -> &'static str {
    BaseDIDUrl::SCHEME
  }

  fn authority(&self) -> &str {
    match self {
      DIDOrPlaceholder::Core(core_did) => core_did.authority(),
      DIDOrPlaceholder::Placeholder => PLACEHOLDER_DID.authority(),
    }
  }

  fn method(&self) -> &str {
    match self {
      DIDOrPlaceholder::Core(core_did) => core_did.method(),
      DIDOrPlaceholder::Placeholder => PLACEHOLDER_DID.method(),
    }
  }

  fn method_id(&self) -> &str {
    match self {
      DIDOrPlaceholder::Core(core_did) => core_did.method_id(),
      DIDOrPlaceholder::Placeholder => PLACEHOLDER_DID.method_id(),
    }
  }

  fn as_str(&self) -> &str {
    match self {
      DIDOrPlaceholder::Core(core_did) => core_did.as_str(),
      DIDOrPlaceholder::Placeholder => PLACEHOLDER_DID.as_str(),
    }
  }

  fn into_string(self) -> String {
    match self {
      DIDOrPlaceholder::Core(core_did) => core_did.into_string(),
      DIDOrPlaceholder::Placeholder => PLACEHOLDER_DID.as_str().to_owned(),
    }
  }

  fn join(self, value: impl AsRef<str>) -> Result<DIDUrl<Self>, DIDError> {
    DIDUrl::new(self, None).join(value)
  }

  fn to_url(&self) -> DIDUrl<Self> {
    DIDUrl::new(self.clone(), None)
  }

  fn into_url(self) -> DIDUrl<Self> {
    DIDUrl::new(self, None)
  }
}

impl FromStr for DIDOrPlaceholder {
  type Err = DIDError;

  fn from_str(string: &str) -> Result<Self, Self::Err> {
    if string == PLACEHOLDER_DID.as_str() {
      Ok(Self::Placeholder)
    } else {
      CoreDID::from_str(string).map(Self::Core)
    }
  }
}

impl TryFrom<BaseDIDUrl> for DIDOrPlaceholder {
  type Error = DIDError;

  fn try_from(base_did_url: BaseDIDUrl) -> Result<Self, Self::Error> {
    if base_did_url.as_str() == PLACEHOLDER_DID.as_str() {
      Ok(Self::Placeholder)
    } else {
      CoreDID::try_from(base_did_url).map(Self::Core)
    }
  }
}

impl KeyComparable for DIDOrPlaceholder {
  type Key = CoreDID;

  #[inline]
  fn key(&self) -> &Self::Key {
    match self {
      DIDOrPlaceholder::Core(core_did) => core_did,
      DIDOrPlaceholder::Placeholder => &PLACEHOLDER_DID,
    }
  }
}

impl From<CoreDID> for DIDOrPlaceholder {
  fn from(did: CoreDID) -> Self {
    if &did == (&PLACEHOLDER_DID) as &CoreDID {
      DIDOrPlaceholder::Placeholder
    } else {
      DIDOrPlaceholder::Core(did)
    }
  }
}

mod placeholder_serde {
  //! Provides serialization for DidOrPlaceholder::Placeholder as a string.

  use identity_did::did::DID;
  use serde::de::Visitor;
  use serde::de::{self};
  use serde::Deserializer;
  use serde::Serialize;
  use serde::Serializer;

  use super::PLACEHOLDER_DID;

  pub(crate) fn serialize<S>(serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    PLACEHOLDER_DID.serialize(serializer)
  }

  pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<(), D::Error>
  where
    D: Deserializer<'de>,
  {
    struct DIDOrPlaceholderVisitor;

    impl<'de> Visitor<'de> for DIDOrPlaceholderVisitor {
      type Value = ();

      fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("the placeholder did `did:0:0`")
      }

      fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        if value == PLACEHOLDER_DID.as_str() {
          Ok(())
        } else {
          Err(E::custom("expected placeholder did"))
        }
      }
    }

    deserializer.deserialize_str(DIDOrPlaceholderVisitor)
  }
}
