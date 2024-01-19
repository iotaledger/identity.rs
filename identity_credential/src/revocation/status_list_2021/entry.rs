// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Url;
use serde::Deserialize;
use serde::Serialize;

use crate::credential::Status;

use super::credential::StatusPurpose;

const CREDENTIAL_STATUS_TYPE: &str = "StatusList2021Entry";

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize)]
#[serde(transparent)]
struct EntryType(&'static str);

impl Default for EntryType {
  fn default() -> Self {
    Self(CREDENTIAL_STATUS_TYPE)
  }
}

impl<'de> Deserialize<'de> for EntryType {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    use serde::de::Error;
    use serde::de::Visitor;
    struct ExactStrVisitor(&'static str);
    impl<'a> Visitor<'a> for ExactStrVisitor {
      type Value = &'static str;
      fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "the exact string \"{}\"", self.0)
      }
      fn visit_str<E: Error>(self, str: &str) -> Result<Self::Value, E> {
        if str == self.0 {
          Ok(self.0)
        } else {
          Err(E::custom(format!("not \"{}\"", self.0)))
        }
      }
    }

    deserializer
      .deserialize_str(ExactStrVisitor(CREDENTIAL_STATUS_TYPE))
      .map(EntryType)
  }
}

/// [StatusList2021Entry](https://www.w3.org/TR/2023/WD-vc-status-list-20230427/#statuslist2021entry) implementation.
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StatusList2021Entry {
  id: Option<Url>,
  #[serde(rename = "type")]
  type_: EntryType,
  status_purpose: StatusPurpose,
  #[serde(deserialize_with = "serde_aux::prelude::deserialize_number_from_string")]
  status_list_index: usize,
  status_list_credential: Url,
}

impl TryFrom<&Status> for StatusList2021Entry {
  type Error = serde_json::Error;
  fn try_from(status: &Status) -> Result<Self, Self::Error> {
    let json_status = serde_json::to_value(status).unwrap();
    serde_json::from_value(json_status)
  }
}

impl From<StatusList2021Entry> for Status {
  fn from(entry: StatusList2021Entry) -> Self {
    let json_status = serde_json::to_value(entry).unwrap(); // Safety: shouldn't go out of memory
    serde_json::from_value(json_status).unwrap() // Safety: `StatusList2021Entry` is a credential status
  }
}

impl StatusList2021Entry {
  /// Creates a new [`StatusList2021Entry`].
  pub fn new(status_list: Url, purpose: StatusPurpose, index: usize, id: Option<Url>) -> Self {
    Self {
      id,
      type_: EntryType::default(),
      status_purpose: purpose,
      status_list_credential: status_list,
      status_list_index: index,
    }
  }

  /// Returns this `credentialStatus`'s `id`.
  pub const fn id(&self) -> Option<&Url> {
    self.id.as_ref()
  }

  /// Returns the purpose of this entry.
  pub const fn purpose(&self) -> StatusPurpose {
    self.status_purpose
  }

  /// Returns the index of this entry.
  pub const fn index(&self) -> usize {
    self.status_list_index
  }

  /// Returns the referenced [`StatusList2021Credential`]'s [`Url`].
  pub const fn credential(&self) -> &Url {
    &self.status_list_credential
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const STATUS_LIST_ENTRY_SAMPLE: &str = r#"
{
    "id": "https://example.com/credentials/status/3#94567",
    "type": "StatusList2021Entry",
    "statusPurpose": "revocation",
    "statusListIndex": "94567",
    "statusListCredential": "https://example.com/credentials/status/3"
}"#;

  #[test]
  fn entry_deserialization_works() {
    let deserialized =
      serde_json::from_str::<StatusList2021Entry>(STATUS_LIST_ENTRY_SAMPLE).expect("Failed to deserialize");

    assert_eq!(deserialized.index(), 94567);
  }
}
