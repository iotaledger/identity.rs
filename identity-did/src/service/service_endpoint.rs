// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Display;
use core::fmt::Error as FmtError;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;

use serde::Serialize;

use identity_core::common::Url;
use identity_core::convert::ToJson;

use crate::utils::OrderedSet;

/// An endpoint or set of endpoints specified in a [`Service`].
///
/// [Specification](https://www.w3.org/TR/did-core/#dfn-serviceendpoint)
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ServiceEndpoint {
  One(Url),
  Set(OrderedSet<Url>),
  // TODO: Enforce set is non-empty?
  // TODO: Should this support an ordered map and nested maps which the specification allows?
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

impl Display for ServiceEndpoint {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    if f.alternate() {
      f.write_str(&self.to_json_pretty().map_err(|_| FmtError)?)
    } else {
      f.write_str(&self.to_json().map_err(|_| FmtError)?)
    }
  }
}

#[cfg(test)]
mod tests {
  use identity_core::common::Url;
  use identity_core::convert::{FromJson, ToJson};

  use crate::service::ServiceEndpoint;
  use crate::utils::OrderedSet;

  #[test]
  fn test_service_endpoint_serde() {
    let url1 = Url::parse("https://iota.org/").unwrap();
    let url2 = Url::parse("wss://www.example.com/socketserver/").unwrap();
    let url3 = Url::parse("did:abc:123#service").unwrap();

    // VALID: One.
    let endpoint1: ServiceEndpoint = ServiceEndpoint::One(url1.clone());
    let ser_endpoint1: String = endpoint1.to_json().unwrap();
    assert_eq!(ser_endpoint1, "\"https://iota.org/\"");
    assert_eq!(endpoint1, ServiceEndpoint::from_json(&ser_endpoint1).unwrap());

    let endpoint2: ServiceEndpoint = ServiceEndpoint::One(url2.clone());
    let ser_endpoint2: String = endpoint2.to_json().unwrap();
    assert_eq!(ser_endpoint2, "\"wss://www.example.com/socketserver/\"");
    assert_eq!(endpoint2, ServiceEndpoint::from_json(&ser_endpoint2).unwrap());

    let endpoint3: ServiceEndpoint = ServiceEndpoint::One(url3.clone());
    let ser_endpoint3: String = endpoint3.to_json().unwrap();
    assert_eq!(ser_endpoint3, "\"did:abc:123#service\"");
    assert_eq!(endpoint3, ServiceEndpoint::from_json(&ser_endpoint3).unwrap());

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
    assert_eq!(ser_endpoint_set, "[\"https://iota.org/\",\"wss://www.example.com/socketserver/\"]");
    assert_eq!(endpoint_set, ServiceEndpoint::from_json(&ser_endpoint_set).unwrap());
    // Three elements.
    assert!(set.append(url3.clone()));
    let endpoint_set: ServiceEndpoint = ServiceEndpoint::Set(set.clone());
    let ser_endpoint_set: String = endpoint_set.to_json().unwrap();
    assert_eq!(ser_endpoint_set, "[\"https://iota.org/\",\"wss://www.example.com/socketserver/\",\"did:abc:123#service\"]");
    assert_eq!(endpoint_set, ServiceEndpoint::from_json(&ser_endpoint_set).unwrap());

    // VALID: Set ignore duplicates.
    let mut duplicates_set: OrderedSet<Url> = OrderedSet::new();
    duplicates_set.append(url1.clone());
    duplicates_set.append(url1.clone());
    assert_eq!(ServiceEndpoint::Set(duplicates_set.clone()).to_json().unwrap(), "[\"https://iota.org/\"]");
    duplicates_set.append(url2.clone());
    duplicates_set.append(url2.clone());
    duplicates_set.append(url1.clone());
    assert_eq!(ServiceEndpoint::Set(duplicates_set.clone()).to_json().unwrap(), "[\"https://iota.org/\",\"wss://www.example.com/socketserver/\"]");
    assert!(duplicates_set.append(url3.clone()));
    duplicates_set.append(url3.clone());
    duplicates_set.append(url1.clone());
    duplicates_set.append(url2.clone());
    assert_eq!(ser_endpoint_set, "[\"https://iota.org/\",\"wss://www.example.com/socketserver/\",\"did:abc:123#service\"]");
  }

  #[test]
  fn test_service_endpoint_serde_fails() {
    // INVALID: empty
    assert!(ServiceEndpoint::from_json("\"\"").is_err());
    assert!(ServiceEndpoint::from_json("").is_err());

    // INVALID: spaces
    assert!(ServiceEndpoint::from_json("\" \"").is_err());
    assert!(ServiceEndpoint::from_json("\"\t\"").is_err());
    assert!(ServiceEndpoint::from_json("\"https:// iota.org/\"").is_err());
    assert!(ServiceEndpoint::from_json("\"https://\tiota.org/\"").is_err());
    assert!(ServiceEndpoint::from_json("[\"https:// iota.org/\",\"wss://www.example.com/socketserver/\"]").is_err());
    assert!(ServiceEndpoint::from_json("[\"https:// iota.org/\",\"wss://www.example.com/socketserver/\"]").is_err());

    // INVALID: duplicate keys
    assert!(ServiceEndpoint::from_json("[\"https://iota.org/\",\"https://iota.org/\"]").is_err());
    // INVALID: duplicate keys when normalised
    assert!(ServiceEndpoint::from_json("[\"https://iota.org/a/b/b/b\",\"https://iota.org/a/b/\"]").is_err());

    // INVALID: wrong map syntax (no keys)
    assert!(ServiceEndpoint::from_json("{\"https:// iota.org/\",\"wss://www.example.com/socketserver/\"}").is_err());
  }
}
