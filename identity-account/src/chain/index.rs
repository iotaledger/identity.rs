// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::str;
use identity_core::common::Value;
use identity_core::convert::FromJson;
use identity_core::convert::ToJson;
use identity_core::json;
use identity_core::utils::decode_b64;
use identity_core::utils::encode_b64;
use identity_iota::did::DID;
use serde::de;
use serde::ser;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

use crate::error::Error;
use crate::error::Result;
use crate::types::ChainId;

type Item<'a> = (&'a Slot, &'a ChainId);
type State = BTreeMap<Slot, ChainId>;

// =============================================================================
// Chain Index
// =============================================================================

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct ChainIndex(State);

impl ChainIndex {
  pub fn new() -> Self {
    Self(State::new())
  }

  pub fn first(&self) -> Option<ChainId> {
    self.0.values().next().copied()
  }

  pub fn last(&self) -> Option<ChainId> {
    self.0.values().last().copied()
  }

  pub fn try_first(&self) -> Result<ChainId> {
    self.first().ok_or(Error::NoChainsFound)
  }

  pub fn try_last(&self) -> Result<ChainId> {
    self.last().ok_or(Error::NoChainsFound)
  }

  pub fn get<K: Key>(&self, key: K) -> Option<ChainId> {
    key.find_iter(self.0.iter())
  }

  pub fn set(&mut self, chain: ChainId, did: &DID) -> Result<()> {
    self.insert(Slot::new(did), chain)
  }

  pub fn set_named(&mut self, chain: ChainId, did: &DID, name: String) -> Result<()> {
    self.insert(Slot::named(did, name), chain)
  }

  fn insert(&mut self, slot: Slot, chain: ChainId) -> Result<()> {
    match self.0.entry(slot) {
      Entry::Occupied(_) => Err(Error::ChainAlreadyExists),
      Entry::Vacant(entry) => {
        entry.insert(chain);
        Ok(())
      }
    }
  }

  pub fn count(&self) -> usize {
    self.0.len()
  }

  pub fn empty(&self) -> bool {
    self.0.is_empty()
  }

  pub fn next_id(&self) -> ChainId {
    self.0.values().max().copied().unwrap_or_default().next()
  }
}

impl Debug for ChainIndex {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    const LIMIT: usize = 10;

    f.debug_struct("ChainIndex")
      .field("count", &self.0.len())
      .field("state", &self.0.iter().take(LIMIT).collect::<BTreeMap<_, _>>())
      .finish()
  }
}

fn default_identifier(chain: ChainId) -> String {
  format!("Identity {}", chain.to_u32())
}

// =============================================================================
// Slot
// =============================================================================

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Slot {
  // DID method id
  meid: String,
  // User-assigned name
  name: Name,
}

impl Slot {
  pub fn new(did: &DID) -> Self {
    Self {
      meid: did.method_id().to_string(),
      name: Name::Default,
    }
  }

  pub fn named(did: &DID, name: String) -> Self {
    Self {
      meid: did.method_id().to_string(),
      name: Name::Literal(name),
    }
  }

  pub fn id(&self) -> &str {
    &self.meid
  }

  pub fn name(&self, chain: ChainId) -> Cow<'_, str> {
    self.name.as_str(chain)
  }

  fn encode(&self) -> Result<String> {
    let data: Value = match self.name {
      Name::Default => json!({"meid": self.meid}),
      Name::Literal(ref value) => json!({"meid": self.meid, "name": value}),
    };

    let data: Vec<u8> = data.to_json_vec()?;
    let data: String = encode_b64(&data);

    Ok(data)
  }

  fn decode(string: &str) -> Result<Self> {
    #[derive(Deserialize)]
    struct Data {
      meid: String,
      name: Option<String>,
    }

    let data: Vec<u8> = decode_b64(string)?;
    let this: Data = Data::from_json_slice(&data)?;

    Ok(Self {
      meid: this.meid,
      name: this.name.map(Name::Literal).unwrap_or(Name::Default),
    })
  }
}

impl Display for Slot {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    if let Some(name) = self.name.as_opt() {
      f.write_fmt(format_args!("{} - {}", self.id(), name))
    } else {
      f.write_fmt(format_args!("{}", self.id()))
    }
  }
}

impl Debug for Slot {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.write_fmt(format_args!("Slot({})", self))
  }
}

impl Serialize for Slot {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: ser::Serializer,
  {
    match self.encode() {
      Ok(data) => serializer.serialize_str(&data),
      Err(error) => Err(ser::Error::custom(error)),
    }
  }
}

impl<'de> Deserialize<'de> for Slot {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: de::Deserializer<'de>,
  {
    struct Visitor;

    impl<'de> de::Visitor<'de> for Visitor {
      type Value = Slot;

      fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("a base64-encoded string")
      }

      fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        Slot::decode(value).map_err(E::custom)
      }
    }

    deserializer.deserialize_str(Visitor)
  }
}

// =============================================================================
// Name (user-assigned)
// =============================================================================

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Name {
  Default,
  Literal(String),
}

impl Name {
  pub fn as_str(&self, chain: ChainId) -> Cow<'_, str> {
    match self {
      Self::Default => Cow::Owned(default_identifier(chain)),
      Self::Literal(ref inner) => Cow::Borrowed(inner),
    }
  }

  pub fn as_opt(&self) -> Option<&str> {
    match self {
      Name::Default => None,
      Name::Literal(ref value) => Some(value),
    }
  }
}

// =============================================================================
// Supported Index Keys
// =============================================================================

pub trait Key {
  fn find_iter<'i, I: Iterator<Item = Item<'i>>>(&self, iter: I) -> Option<ChainId>;
}

impl<'a, T> Key for &'a T
where
  T: Key,
{
  fn find_iter<'i, I: Iterator<Item = Item<'i>>>(&self, iter: I) -> Option<ChainId> {
    (**self).find_iter(iter)
  }
}

impl Key for DID {
  fn find_iter<'i, I: Iterator<Item = Item<'i>>>(&self, mut iter: I) -> Option<ChainId> {
    iter
      .find(|(slot, _)| slot.id() == self.method_id())
      .map(|(_, chain)| *chain)
  }
}

impl Key for str {
  fn find_iter<'i, I: Iterator<Item = Item<'i>>>(&self, mut iter: I) -> Option<ChainId> {
    iter
      .find(|(slot, chain)| slot.name(**chain).as_ref() == self)
      .map(|(_, chain)| *chain)
  }
}

impl Key for String {
  fn find_iter<'i, I: Iterator<Item = Item<'i>>>(&self, iter: I) -> Option<ChainId> {
    self[..].find_iter(iter)
  }
}

impl Key for ChainId {
  fn find_iter<'i, I: Iterator<Item = Item<'i>>>(&self, _: I) -> Option<ChainId> {
    Some(*self)
  }
}
