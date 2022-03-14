// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use bee_message::MessageId;
use identity_core::common::Object;
use identity_core::common::Timestamp;
use identity_core::diff::Diff;
use identity_core::diff::DiffString;
use identity_core::diff::Error;
use identity_core::diff::Result;
use serde::Deserialize;
use serde::Serialize;

use crate::document::IotaDocumentMetadata;

/// NOTE: excludes the `proof` [`Signature`] from the diff to save space on the Tangle and because
/// a merged signature will be invalid in general.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DiffIotaDocumentMetadata {
  #[serde(skip_serializing_if = "Option::is_none")]
  created: Option<<Timestamp as Diff>::Type>,
  #[serde(skip_serializing_if = "Option::is_none")]
  updated: Option<<Timestamp as Diff>::Type>,
  #[serde(skip_serializing_if = "Option::is_none")]
  previous_message_id: Option<DiffString>,
  #[serde(skip_serializing_if = "Option::is_none")]
  properties: Option<<Object as Diff>::Type>,
}

impl Diff for IotaDocumentMetadata {
  type Type = DiffIotaDocumentMetadata;

  fn diff(&self, other: &Self) -> Result<Self::Type> {
    Ok(DiffIotaDocumentMetadata {
      created: if self.created == other.created {
        None
      } else {
        Some(self.created.diff(&other.created)?)
      },
      updated: if self.updated == other.updated {
        None
      } else {
        Some(self.updated.diff(&other.updated)?)
      },
      // TODO: see if we can impl Diff for MessageId
      previous_message_id: if self.previous_message_id == other.previous_message_id {
        None
      } else {
        Some(
          self
            .previous_message_id
            .to_string()
            .diff(&other.previous_message_id.to_string())?,
        )
      },
      properties: if self.properties == other.properties {
        None
      } else {
        Some(self.properties.diff(&other.properties)?)
      },
    })
  }

  fn merge(&self, diff: Self::Type) -> Result<Self> {
    let created: Timestamp = diff
      .created
      .map(|value| self.created.merge(value))
      .transpose()?
      .unwrap_or(self.created);

    let updated: Timestamp = diff
      .updated
      .map(|value| self.updated.merge(value))
      .transpose()?
      .unwrap_or(self.updated);

    // TODO: see if we can impl Diff for MessageId
    let previous_message_id: MessageId = diff
      .previous_message_id
      .map(|value| self.previous_message_id.to_string().merge(value))
      .transpose()?
      .map(|message_id_str| MessageId::from_str(&message_id_str))
      .transpose()
      .map_err(identity_core::diff::Error::merge)?
      .unwrap_or(self.previous_message_id);

    let properties: Object = diff
      .properties
      .map(|value| self.properties.merge(value))
      .transpose()?
      .unwrap_or_else(|| self.properties.clone());

    Ok(IotaDocumentMetadata {
      created,
      updated,
      previous_message_id,
      proof: None, // NOTE: proof intentionally excluded.
      properties,
    })
  }

  fn from_diff(diff: Self::Type) -> Result<Self> {
    let created: Timestamp = diff
      .created
      .map(Timestamp::from_diff)
      .transpose()?
      .ok_or_else(|| Error::convert("Missing field `metadata.created`"))?;

    let updated: Timestamp = diff
      .updated
      .map(Timestamp::from_diff)
      .transpose()?
      .ok_or_else(|| Error::convert("Missing field `metadata.updated`"))?;

    // TODO: see if we can impl Diff for MessageId
    let previous_message_id: MessageId = diff
      .previous_message_id
      .map(String::from_diff)
      .transpose()?
      .map(|message_id_str| MessageId::from_str(&message_id_str))
      .transpose()
      .map_err(identity_core::diff::Error::merge)?
      .ok_or_else(|| Error::convert("Missing field `metadata.previous_message_id`"))?;

    let properties: Object = diff.properties.map(Object::from_diff).transpose()?.unwrap_or_default();

    Ok(IotaDocumentMetadata {
      created,
      updated,
      previous_message_id,
      proof: None, // NOTE: proof intentionally excluded.
      properties,
    })
  }

  fn into_diff(self) -> Result<Self::Type> {
    Ok(DiffIotaDocumentMetadata {
      created: Some(self.created.into_diff()?),
      updated: Some(self.updated.into_diff()?),
      previous_message_id: Some(self.previous_message_id.to_string().into_diff()?),
      properties: if self.properties == Default::default() {
        None
      } else {
        Some(self.properties.into_diff()?)
      },
    })
  }
}

#[cfg(test)]
mod test {
  use bee_message::MESSAGE_ID_LENGTH;

  use identity_core::common::Object;
  use identity_core::convert::FromJson;
  use identity_core::convert::ToJson;

  use super::*;

  #[test]
  fn test_created() {
    let original: IotaDocumentMetadata = IotaDocumentMetadata::new();
    let mut updated: IotaDocumentMetadata = original.clone();
    updated.created = Timestamp::parse("1999-01-01T00:00:00Z").unwrap();

    let diff: DiffIotaDocumentMetadata = original.diff(&updated).unwrap();
    assert_eq!(diff.created, Some(DiffString(Some("1999-01-01T00:00:00Z".to_owned()))));
    assert!(diff.updated.is_none());
    assert!(diff.previous_message_id.is_none());
    assert!(diff.properties.is_none());
    let merged: IotaDocumentMetadata = original.merge(diff).unwrap();
    assert_eq!(merged, updated);
  }

  #[test]
  fn test_updated() {
    let original: IotaDocumentMetadata = IotaDocumentMetadata::new();
    let mut updated: IotaDocumentMetadata = original.clone();
    updated.updated = Timestamp::parse("1999-01-01T00:00:00Z").unwrap();

    let diff: DiffIotaDocumentMetadata = original.diff(&updated).unwrap();
    assert!(diff.created.is_none());
    assert_eq!(diff.updated, Some(DiffString(Some("1999-01-01T00:00:00Z".to_owned()))));
    assert!(diff.previous_message_id.is_none());
    assert!(diff.properties.is_none());
    let merged: IotaDocumentMetadata = original.merge(diff).unwrap();
    assert_eq!(merged, updated);
  }

  #[test]
  fn test_previous_message_id() {
    let original: IotaDocumentMetadata = IotaDocumentMetadata::new();
    let mut updated: IotaDocumentMetadata = original.clone();
    updated.previous_message_id = MessageId::from([8; MESSAGE_ID_LENGTH]);

    let diff: DiffIotaDocumentMetadata = original.diff(&updated).unwrap();
    assert!(diff.created.is_none());
    assert!(diff.updated.is_none());
    assert_eq!(
      diff.previous_message_id,
      Some(DiffString(Some(
        "0808080808080808080808080808080808080808080808080808080808080808".to_owned()
      )))
    );
    assert!(diff.properties.is_none());
    let merged: IotaDocumentMetadata = original.merge(diff).unwrap();
    assert_eq!(merged, updated);
  }

  #[test]
  fn test_add_properties() {
    let mut original: IotaDocumentMetadata = IotaDocumentMetadata::new();
    let mut properties: Object = Object::default();
    properties.insert("key1".into(), "value2".into());
    original.properties = properties;

    let mut updated: IotaDocumentMetadata = original.clone();
    updated.properties.insert("key2".into(), "value2".into());

    assert_ne!(original, updated);
    let diff: DiffIotaDocumentMetadata = original.diff(&updated).unwrap();
    let merged: IotaDocumentMetadata = original.merge(diff).unwrap();
    assert_eq!(merged, updated);
  }

  #[test]
  fn test_replace_properties() {
    let mut original: IotaDocumentMetadata = IotaDocumentMetadata::new();
    let mut properties: Object = Object::default();
    properties.insert("key".to_string(), "value".into());
    original.properties = properties;

    let mut updated: IotaDocumentMetadata = original.clone();
    updated.properties = Object::default();

    assert_ne!(original, updated);
    let diff: DiffIotaDocumentMetadata = original.diff(&updated).unwrap();
    let merged: IotaDocumentMetadata = original.merge(diff).unwrap();
    assert_eq!(merged, updated);
  }

  #[test]
  fn test_from_into_diff() {
    let original: IotaDocumentMetadata = IotaDocumentMetadata::new();
    let diff: DiffIotaDocumentMetadata = original.clone().into_diff().unwrap();
    let from: IotaDocumentMetadata = IotaDocumentMetadata::from_diff(diff.clone()).unwrap();
    assert_eq!(from, original);

    let ser: String = diff.to_json().unwrap();
    let de: DiffIotaDocumentMetadata = DiffIotaDocumentMetadata::from_json(&ser).unwrap();
    assert_eq!(diff, de);
    let from: IotaDocumentMetadata = IotaDocumentMetadata::from_diff(de).unwrap();
    assert_eq!(from, original);
  }

  #[test]
  fn test_serde() {
    let original: IotaDocumentMetadata = IotaDocumentMetadata::new();
    let mut updated: IotaDocumentMetadata = IotaDocumentMetadata::new();
    updated.previous_message_id = MessageId::new([1; 32]);
    updated.created = Timestamp::from_unix(1).unwrap();
    updated.updated = Timestamp::from_unix(100000).unwrap();
    let diff: DiffIotaDocumentMetadata = Diff::diff(&original, &updated).unwrap();

    let ser: String = diff.to_json().unwrap();
    let de: DiffIotaDocumentMetadata = DiffIotaDocumentMetadata::from_json(&ser).unwrap();
    assert_eq!(diff, de);
    let merge: IotaDocumentMetadata = Diff::merge(&original, de).unwrap();
    assert_eq!(merge, updated);
  }
}
