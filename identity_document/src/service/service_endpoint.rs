// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Display;
use core::fmt::Formatter;

use indexmap::map::IndexMap;
use serde::Serialize;

use identity_core::common::OrderedSet;
use identity_core::common::Url;
use identity_core::convert::FmtJson;

/// A single URL, set, or map of endpoints specified in a [`Service`](crate::service::Service).
///
/// [Specification](https://www.w3.org/TR/did-core/#dfn-serviceendpoint)
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ServiceEndpoint {
  One(Url),
  Set(OrderedSet<Url>),
  Map(IndexMap<String, OrderedSet<Url>>),
}

impl From<Url> for ServiceEndpoint {
  fn from(url: Url) -> Self {
    ServiceEndpoint::One(url)
  }
}

impl From<OrderedSet<Url>> for ServiceEndpoint {
  fn from(set: OrderedSet<Url>) -> Self {
    ServiceEndpoint::Set(set)
  }
}

impl From<IndexMap<String, OrderedSet<Url>>> for ServiceEndpoint {
  fn from(map: IndexMap<String, OrderedSet<Url>>) -> Self {
    ServiceEndpoint::Map(map)
  }
}

impl Display for ServiceEndpoint {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    self.fmt_json(f)
  }
}

#[cfg(test)]
mod tests {
  use identity_core::convert::FromJson;
  use identity_core::convert::ToJson;

  use super::*;

  #[test]
  fn test_service_endpoint_one() {
    let url1 = Url::parse("https://iota.org/").unwrap();
    let url2 = Url::parse("wss://www.example.com/socketserver/").unwrap();
    let url3 = Url::parse("did:abc:123#service").unwrap();

    // VALID: One.
    let endpoint1: ServiceEndpoint = ServiceEndpoint::One(url1);
    let ser_endpoint1: String = endpoint1.to_json().unwrap();
    assert_eq!(ser_endpoint1, "\"https://iota.org/\"");
    assert_eq!(endpoint1, ServiceEndpoint::from_json(&ser_endpoint1).unwrap());

    let endpoint2: ServiceEndpoint = ServiceEndpoint::One(url2);
    let ser_endpoint2: String = endpoint2.to_json().unwrap();
    assert_eq!(ser_endpoint2, "\"wss://www.example.com/socketserver/\"");
    assert_eq!(endpoint2, ServiceEndpoint::from_json(&ser_endpoint2).unwrap());

    let endpoint3: ServiceEndpoint = ServiceEndpoint::One(url3);
    let ser_endpoint3: String = endpoint3.to_json().unwrap();
    assert_eq!(ser_endpoint3, "\"did:abc:123#service\"");
    assert_eq!(endpoint3, ServiceEndpoint::from_json(&ser_endpoint3).unwrap());
  }

  #[test]
  fn test_service_endpoint_set() {
    let url1 = Url::parse("https://iota.org/").unwrap();
    let url2 = Url::parse("wss://www.example.com/socketserver/").unwrap();
    let url3 = Url::parse("did:abc:123#service").unwrap();

    // VALID: Set.
    let mut set: OrderedSet<Url> = OrderedSet::new();
    // One element.
    assert!(set.append(url1.clone()));
    let endpoint_set: ServiceEndpoint = ServiceEndpoint::Set(set.clone());
    let ser_endpoint_set: String = endpoint_set.to_json().unwrap();
    assert_eq!(ser_endpoint_set, "[\"https://iota.org/\"]");
    assert_eq!(endpoint_set, ServiceEndpoint::from_json(&ser_endpoint_set).unwrap());
    // Two elements.
    assert!(set.append(url2.clone()));
    let endpoint_set: ServiceEndpoint = ServiceEndpoint::Set(set.clone());
    let ser_endpoint_set: String = endpoint_set.to_json().unwrap();
    assert_eq!(
      ser_endpoint_set,
      "[\"https://iota.org/\",\"wss://www.example.com/socketserver/\"]"
    );
    assert_eq!(endpoint_set, ServiceEndpoint::from_json(&ser_endpoint_set).unwrap());
    // Three elements.
    assert!(set.append(url3.clone()));
    let endpoint_set: ServiceEndpoint = ServiceEndpoint::Set(set.clone());
    let ser_endpoint_set: String = endpoint_set.to_json().unwrap();
    assert_eq!(
      ser_endpoint_set,
      "[\"https://iota.org/\",\"wss://www.example.com/socketserver/\",\"did:abc:123#service\"]"
    );
    assert_eq!(endpoint_set, ServiceEndpoint::from_json(&ser_endpoint_set).unwrap());

    // VALID: Set ignores duplicates.
    let mut duplicates_set: OrderedSet<Url> = OrderedSet::new();
    duplicates_set.append(url1.clone());
    duplicates_set.append(url1.clone());
    assert_eq!(
      ServiceEndpoint::Set(duplicates_set.clone()).to_json().unwrap(),
      "[\"https://iota.org/\"]"
    );
    duplicates_set.append(url2.clone());
    duplicates_set.append(url2.clone());
    duplicates_set.append(url1.clone());
    assert_eq!(
      ServiceEndpoint::Set(duplicates_set.clone()).to_json().unwrap(),
      "[\"https://iota.org/\",\"wss://www.example.com/socketserver/\"]"
    );
    assert!(duplicates_set.append(url3.clone()));
    duplicates_set.append(url3);
    duplicates_set.append(url1);
    duplicates_set.append(url2);
    assert_eq!(
      ServiceEndpoint::Set(duplicates_set.clone()).to_json().unwrap(),
      "[\"https://iota.org/\",\"wss://www.example.com/socketserver/\",\"did:abc:123#service\"]"
    );
  }

  #[test]
  fn test_service_endpoint_map() {
    let url1 = Url::parse("https://iota.org/").unwrap();
    let url2 = Url::parse("wss://www.example.com/socketserver/").unwrap();
    let url3 = Url::parse("did:abc:123#service").unwrap();
    let url4 = Url::parse("did:xyz:789#link").unwrap();

    // VALID: Map.
    let mut map: IndexMap<String, OrderedSet<Url>> = IndexMap::new();
    // One entry.
    assert!(map
      .insert("key".to_owned(), OrderedSet::try_from(vec![url1]).unwrap())
      .is_none());
    let endpoint_map: ServiceEndpoint = ServiceEndpoint::Map(map.clone());
    let ser_endpoint_map: String = endpoint_map.to_json().unwrap();
    assert_eq!(ser_endpoint_map, r#"{"key":["https://iota.org/"]}"#);
    assert_eq!(endpoint_map, ServiceEndpoint::from_json(&ser_endpoint_map).unwrap());
    // Two entries.
    assert!(map
      .insert("apple".to_owned(), OrderedSet::try_from(vec![url2]).unwrap())
      .is_none());
    let endpoint_map: ServiceEndpoint = ServiceEndpoint::Map(map.clone());
    let ser_endpoint_map: String = endpoint_map.to_json().unwrap();
    assert_eq!(
      ser_endpoint_map,
      r#"{"key":["https://iota.org/"],"apple":["wss://www.example.com/socketserver/"]}"#
    );
    assert_eq!(endpoint_map, ServiceEndpoint::from_json(&ser_endpoint_map).unwrap());
    // Three entries.
    assert!(map
      .insert("example".to_owned(), OrderedSet::try_from(vec![url3]).unwrap())
      .is_none());
    let endpoint_map: ServiceEndpoint = ServiceEndpoint::Map(map.clone());
    let ser_endpoint_map: String = endpoint_map.to_json().unwrap();
    assert_eq!(
      ser_endpoint_map,
      r#"{"key":["https://iota.org/"],"apple":["wss://www.example.com/socketserver/"],"example":["did:abc:123#service"]}"#
    );
    assert_eq!(endpoint_map, ServiceEndpoint::from_json(&ser_endpoint_map).unwrap());

    // Ensure insertion order is maintained.
    // Remove first entry and add a new one.
    map.shift_remove("key"); // N.B: only shift_remove retains order for IndexMap
    assert!(map
      .insert("bee".to_owned(), OrderedSet::try_from(vec![url4]).unwrap())
      .is_none());
    let endpoint_map: ServiceEndpoint = ServiceEndpoint::Map(map.clone());
    let ser_endpoint_map: String = endpoint_map.to_json().unwrap();
    assert_eq!(
      ser_endpoint_map,
      r#"{"apple":["wss://www.example.com/socketserver/"],"example":["did:abc:123#service"],"bee":["did:xyz:789#link"]}"#
    );
    assert_eq!(endpoint_map, ServiceEndpoint::from_json(&ser_endpoint_map).unwrap());
  }

  #[test]
  fn test_service_endpoint_serde_fails() {
    // INVALID: empty
    assert!(ServiceEndpoint::from_json("").is_err());
    assert!(ServiceEndpoint::from_json("\"\"").is_err());

    // INVALID: spaces
    assert!(ServiceEndpoint::from_json("\" \"").is_err());
    assert!(ServiceEndpoint::from_json("\"\t\"").is_err());
    assert!(ServiceEndpoint::from_json(r#""https:// iota.org/""#).is_err());
    assert!(ServiceEndpoint::from_json(r#"["https://iota.org/","wss://www.example.com /socketserver/"]"#).is_err());
    assert!(ServiceEndpoint::from_json(r#"{"key":["https:// iota.org/"],"apple":["wss://www.example.com/socketserver/"],"example":["did:abc:123#service"]}"#).is_err());

    // INVALID: set with duplicate keys
    assert!(ServiceEndpoint::from_json(r#"["https://iota.org/","https://iota.org/"]"#).is_err());
    // INVALID: set with duplicate keys when normalised
    assert!(ServiceEndpoint::from_json(r#"["https://iota.org/a/./b/../b/.","https://iota.org/a/b/"]"#).is_err());

    // INVALID: map with no keys
    assert!(ServiceEndpoint::from_json(r#"{["https://iota.org/"],["wss://www.example.com/socketserver/"]}"#).is_err());
    assert!(
      ServiceEndpoint::from_json(r#"{"key1":["https://iota.org/"],["wss://www.example.com/socketserver/"]}"#).is_err()
    );
    assert!(
      ServiceEndpoint::from_json(r#"{["https://iota.org/"],"key2":["wss://www.example.com/socketserver/"]}"#).is_err()
    );
  }
}
