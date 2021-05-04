// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Object;
use identity_core::diff::Diff;
use identity_core::diff::DiffString;
use identity_core::diff::Error;
use identity_core::diff::Result;
use serde::Deserialize;
use serde::Serialize;

use crate::did::DID;
use crate::diff::DiffMethodData;
use crate::verification::MethodData;
use crate::verification::MethodType;
use crate::verification::VerificationMethod;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DiffMethod<T = Object>
where
  T: Diff,
{
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<DiffString>,
  #[serde(skip_serializing_if = "Option::is_none")]
  controller: Option<DiffString>,
  #[serde(skip_serializing_if = "Option::is_none")]
  key_type: Option<MethodType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  key_data: Option<DiffMethodData>,
  #[serde(skip_serializing_if = "Option::is_none")]
  properties: Option<<T as Diff>::Type>,
}

impl<T> Diff for VerificationMethod<T>
where
  T: Diff + Serialize + for<'de> Deserialize<'de> + Default,
{
  type Type = DiffMethod<T>;

  fn diff(&self, other: &Self) -> Result<Self::Type> {
    Ok(DiffMethod {
      id: if self.id() == other.id() {
        None
      } else {
        Some(self.id().diff(other.id())?)
      },
      controller: if self.controller() == other.controller() {
        None
      } else {
        Some(self.controller().diff(other.controller())?)
      },
      key_type: if self.key_type() == other.key_type() {
        None
      } else {
        Some(self.key_type().diff(&other.key_type())?)
      },
      key_data: if self.key_data() == other.key_data() {
        None
      } else {
        Some(self.key_data().diff(other.key_data())?)
      },
      properties: if self.properties() == other.properties() {
        None
      } else {
        Some(self.properties().diff(other.properties())?)
      },
    })
  }

  fn merge(&self, diff: Self::Type) -> Result<Self> {
    let id: DID = diff
      .id
      .map(|value| self.id().merge(value))
      .transpose()?
      .unwrap_or_else(|| self.id().clone());

    let controller: DID = diff
      .controller
      .map(|value| self.controller().merge(value))
      .transpose()?
      .unwrap_or_else(|| self.controller().clone());

    let key_data: MethodData = diff
      .key_data
      .map(|value| self.key_data().merge(value))
      .transpose()?
      .unwrap_or_else(|| self.key_data().clone());

    let key_type: MethodType = diff
      .key_type
      .map(|value| self.key_type().merge(value))
      .transpose()?
      .unwrap_or_else(|| self.key_type());

    let properties: T = diff
      .properties
      .map(|value| self.properties().merge(value))
      .transpose()?
      .unwrap_or_else(|| self.properties().clone());

    Ok(VerificationMethod {
      id,
      controller,
      key_type,
      key_data,
      properties,
    })
  }

  fn from_diff(diff: Self::Type) -> Result<Self> {
    let id: DID = diff
      .id
      .map(DID::from_diff)
      .transpose()?
      .ok_or_else(|| Error::convert("Missing field `method.id`"))?;

    let controller: DID = diff
      .controller
      .map(DID::from_diff)
      .transpose()?
      .ok_or_else(|| Error::convert("Missing field `method.controller`"))?;

    let key_type: MethodType = diff
      .key_type
      .map(MethodType::from_diff)
      .transpose()?
      .ok_or_else(|| Error::convert("Missing field `method.key_type`"))?;

    let key_data: MethodData = diff
      .key_data
      .map(MethodData::from_diff)
      .transpose()?
      .ok_or_else(|| Error::convert("Missing field `method.key_data`"))?;

    let properties: T = diff.properties.map(T::from_diff).transpose()?.unwrap_or_default();

    Ok(VerificationMethod {
      id,
      controller,
      key_type,
      key_data,
      properties,
    })
  }

  fn into_diff(self) -> Result<Self::Type> {
    Ok(DiffMethod {
      id: Some(self.id().to_string().into_diff()?),
      controller: Some(self.controller().to_string().into_diff()?),
      key_type: Some(self.key_type().into_diff()?),
      key_data: Some(self.key_data().clone().into_diff()?),
      properties: Some(self.properties().clone().into_diff()?),
    })
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use identity_core::common::Object;
  use identity_core::common::Value;

  fn test_method() -> VerificationMethod {
    VerificationMethod::builder(Default::default())
      .id("did:example:123".parse().unwrap())
      .controller("did:example:123".parse().unwrap())
      .key_type(MethodType::Ed25519VerificationKey2018)
      .key_data(MethodData::PublicKeyBase58("".into()))
      .build()
      .unwrap()
  }

  #[test]
  fn test_diff() {
    let method = test_method();
    let new = method.clone();
    let diff = method.diff(&new).unwrap();
    assert!(diff.id.is_none());
    assert!(diff.controller.is_none());
    assert!(diff.key_data.is_none());
    assert!(diff.key_type.is_none());
    assert!(diff.properties.is_none());
  }

  #[test]
  fn test_properties() {
    let method = test_method();
    let mut new = method.clone();

    // add property
    let mut properties = Object::new();
    properties.insert("key1".to_string(), Value::String("value1".to_string()));
    *new.properties_mut() = properties;

    let diff = method.diff(&new).unwrap();
    assert!(diff.id.is_none());
    assert!(diff.controller.is_none());
    assert!(diff.key_data.is_none());
    assert!(diff.key_type.is_none());

    let merge = method.merge(diff).unwrap();
    assert_eq!(merge, new);

    // add another property
    let mut properties = Object::new();
    properties.insert("key2".to_string(), Value::String("value1".to_string()));
    *new.properties_mut() = properties;

    let diff = method.diff(&new).unwrap();
    assert!(diff.id.is_none());
    assert!(diff.controller.is_none());
    assert!(diff.key_data.is_none());
    assert!(diff.key_type.is_none());

    let merge = method.merge(diff).unwrap();
    assert_eq!(merge, new);

    // change property
    let mut properties = Object::new();
    properties.insert("key2".to_string(), Value::String("value2".to_string()));
    *new.properties_mut() = properties;

    let diff = method.diff(&new).unwrap();
    assert!(diff.id.is_none());
    assert!(diff.controller.is_none());
    assert!(diff.key_data.is_none());
    assert!(diff.key_type.is_none());

    let merge = method.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_id() {
    let method = test_method();
    let mut new = method.clone();
    *new.id_mut() = "did:diff:123".parse().unwrap();

    let diff = method.diff(&new).unwrap();
    assert!(diff.controller.is_none());
    assert!(diff.key_data.is_none());
    assert!(diff.key_type.is_none());
    assert!(diff.properties.is_none());
    assert_eq!(diff.id, Some(DiffString(Some("did:diff:123".to_string()))));

    let merge = method.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_controller() {
    let method = test_method();
    let mut new = method.clone();
    *new.controller_mut() = "did:diff:123".parse().unwrap();

    let diff = method.diff(&new).unwrap();
    assert!(diff.id.is_none());
    assert!(diff.key_data.is_none());
    assert!(diff.key_type.is_none());
    assert!(diff.properties.is_none());
    assert_eq!(diff.controller, Some(DiffString(Some("did:diff:123".to_string()))));

    let merge = method.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_key_type() {
    let method = test_method();
    let mut new = method.clone();
    *new.key_type_mut() = MethodType::MerkleKeyCollection2021;

    let diff = method.diff(&new).unwrap();
    assert!(diff.id.is_none());
    assert!(diff.controller.is_none());
    assert!(diff.key_data.is_none());
    assert!(diff.properties.is_none());
    assert_eq!(diff.key_type, Some(MethodType::MerkleKeyCollection2021));

    let merge = method.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_key_data() {
    let method = test_method();
    let mut new = method.clone();
    *new.key_data_mut() = MethodData::PublicKeyBase58("diff".into());

    let diff = method.diff(&new).unwrap();
    assert!(diff.id.is_none());
    assert!(diff.controller.is_none());
    assert!(diff.key_type.is_none());
    assert!(diff.properties.is_none());
    assert_eq!(
      diff.key_data,
      Some(DiffMethodData::PublicKeyBase58(Some(DiffString(Some(
        "diff".to_string()
      )))))
    );

    let merge = method.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_from_diff() {
    let method = test_method();
    let mut new = method.clone();

    let diff = method.diff(&new).unwrap();
    let diff_method = VerificationMethod::from_diff(diff);
    assert!(diff_method.is_err());

    // add property
    let mut properties = Object::new();
    properties.insert("key1".to_string(), Value::String("value1".to_string()));
    *new.properties_mut() = properties;

    let diff = method.diff(&new).unwrap();
    let diff_method = VerificationMethod::from_diff(diff);
    assert!(diff_method.is_err());

    // add id
    *new.id_mut() = "did:diff:123".parse().unwrap();
    let diff = method.diff(&new).unwrap();
    let diff_method = VerificationMethod::from_diff(diff);
    assert!(diff_method.is_err());

    // add controller
    *new.controller_mut() = "did:diff:123".parse().unwrap();
    let diff = method.diff(&new).unwrap();
    let diff_method = VerificationMethod::from_diff(diff);
    assert!(diff_method.is_err());

    // add key_type
    *new.key_type_mut() = MethodType::MerkleKeyCollection2021;
    let diff = method.diff(&new).unwrap();
    let diff_method = VerificationMethod::from_diff(diff);
    assert!(diff_method.is_err());

    // add key_data
    *new.key_data_mut() = MethodData::PublicKeyBase58("diff".into());
    let diff = method.diff(&new).unwrap();
    let diff_method = VerificationMethod::from_diff(diff.clone());
    assert!(diff_method.is_ok());
    let diff_method = diff_method.unwrap();
    assert_eq!(diff_method, new);

    let merge = method.merge(diff.clone()).unwrap();
    assert_eq!(merge, new);

    assert_eq!(new.into_diff().unwrap(), diff);
  }
}
