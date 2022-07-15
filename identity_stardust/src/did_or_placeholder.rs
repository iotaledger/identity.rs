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
  CoreDID::parse("did:stardust:0x0").unwrap();
}

#[derive(Debug, Hash, Clone, PartialEq, PartialOrd, Ord, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DidOrPlaceholder {
  #[serde(with = "placeholder_serde")]
  Placeholder,
  Core(CoreDID),
}

impl DID for DidOrPlaceholder {
  fn scheme(&self) -> &'static str {
    BaseDIDUrl::SCHEME
  }

  fn authority(&self) -> &str {
    match self {
      DidOrPlaceholder::Core(core_did) => core_did.authority(),
      DidOrPlaceholder::Placeholder => PLACEHOLDER_DID.authority(),
    }
  }

  fn method(&self) -> &str {
    match self {
      DidOrPlaceholder::Core(core_did) => core_did.method(),
      DidOrPlaceholder::Placeholder => PLACEHOLDER_DID.method(),
    }
  }

  fn method_id(&self) -> &str {
    match self {
      DidOrPlaceholder::Core(core_did) => core_did.method_id(),
      DidOrPlaceholder::Placeholder => PLACEHOLDER_DID.method_id(),
    }
  }

  fn as_str(&self) -> &str {
    match self {
      DidOrPlaceholder::Core(core_did) => core_did.as_str(),
      DidOrPlaceholder::Placeholder => PLACEHOLDER_DID.as_str(),
    }
  }

  fn into_string(self) -> String {
    match self {
      DidOrPlaceholder::Core(core_did) => core_did.into_string(),
      DidOrPlaceholder::Placeholder => PLACEHOLDER_DID.as_str().to_owned(),
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

impl FromStr for DidOrPlaceholder {
  type Err = DIDError;

  fn from_str(string: &str) -> Result<Self, Self::Err> {
    if string == PLACEHOLDER_DID.as_str() {
      Ok(Self::Placeholder)
    } else {
      CoreDID::from_str(string).map(Self::Core)
    }
  }
}

impl TryFrom<BaseDIDUrl> for DidOrPlaceholder {
  type Error = DIDError;

  fn try_from(base_did_url: BaseDIDUrl) -> Result<Self, Self::Error> {
    CoreDID::try_from(base_did_url).map(Self::Core)
  }
}

impl KeyComparable for DidOrPlaceholder {
  type Key = CoreDID;

  #[inline]
  fn key(&self) -> &Self::Key {
    match self {
      DidOrPlaceholder::Core(core_did) => core_did,
      DidOrPlaceholder::Placeholder => &PLACEHOLDER_DID,
    }
  }
}

impl From<CoreDID> for DidOrPlaceholder {
  fn from(did: CoreDID) -> Self {
    // TODO: Why does putting `did` instead of `&did` here produce a stack overflow (infinite recursion probably)?
    if &did == (&PLACEHOLDER_DID) as &CoreDID {
      DidOrPlaceholder::Placeholder
    } else {
      DidOrPlaceholder::Core(did)
    }
  }
}

pub(crate) mod placeholder_serde {
  //! Provides serialization for the DidOrPlaceholder::Placeholder as a string.

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
    struct DidOrPlaceholderVisitor;

    impl<'de> Visitor<'de> for DidOrPlaceholderVisitor {
      type Value = ();

      fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a placeholder did")
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

    deserializer.deserialize_str(DidOrPlaceholderVisitor)
  }
}
