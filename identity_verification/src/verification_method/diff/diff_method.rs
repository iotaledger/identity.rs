// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;
use serde::Serialize;

use identity_core::common::Object;
use identity_core::diff::Diff;
use identity_core::diff::Error;
use identity_core::diff::Result;

use super::method_data::DiffMethodData;
use crate::verification_method::MethodBuilder;
use crate::verification_method::MethodData;
use crate::verification_method::MethodType;
use crate::verification_method::VerificationMethod;
use identity_did::CoreDID;
use identity_did::DIDUrl;
use identity_did::DID;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DiffMethod<D = CoreDID, T = Object>
where
  D: Diff + DID,
  T: Diff,
{
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<<DIDUrl<D> as Diff>::Type>,
  #[serde(skip_serializing_if = "Option::is_none")]
  controller: Option<<D as Diff>::Type>,
  #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
  type_: Option<MethodType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  data: Option<DiffMethodData>,
  #[serde(skip_serializing_if = "Option::is_none")]
  properties: Option<<T as Diff>::Type>,
}

impl<D> Diff for VerificationMethod<D>
where
  D: Diff + DID + Serialize + for<'de> Deserialize<'de>,
{
  type Type = DiffMethod<D>;

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
      type_: if self.type_() == other.type_() {
        None
      } else {
        Some(self.type_().diff(&other.type_())?)
      },
      data: if self.data() == other.data() {
        None
      } else {
        Some(self.data().diff(other.data())?)
      },
      properties: if self.properties() == other.properties() {
        None
      } else {
        Some(self.properties().diff(other.properties())?)
      },
    })
  }

  fn merge(&self, diff: Self::Type) -> Result<Self> {
    let id: DIDUrl<D> = diff
      .id
      .map(|value| self.id().merge(value))
      .transpose()?
      .unwrap_or_else(|| self.id().clone());

    let controller: D = diff
      .controller
      .map(|value| self.controller().merge(value))
      .transpose()?
      .unwrap_or_else(|| self.controller().clone());

    let data: MethodData = diff
      .data
      .map(|value| self.data().merge(value))
      .transpose()?
      .unwrap_or_else(|| self.data().clone());

    let type_: MethodType = diff
      .type_
      .map(|value| self.type_().merge(value))
      .transpose()?
      .unwrap_or_else(|| self.type_());

    let properties: Object = diff
      .properties
      .map(|value| self.properties().merge(value))
      .transpose()?
      .unwrap_or_else(|| self.properties().clone());

    // Use builder to enforce invariants.
    MethodBuilder::new(properties)
      .id(id)
      .controller(controller)
      .type_(type_)
      .data(data)
      .build()
      .map_err(Error::merge)
  }

  fn from_diff(diff: Self::Type) -> Result<Self> {
    let id: DIDUrl<D> = diff
      .id
      .map(Diff::from_diff)
      .transpose()?
      .ok_or_else(|| Error::convert("Missing field `method.id`"))?;

    let controller: D = diff
      .controller
      .map(Diff::from_diff)
      .transpose()?
      .ok_or_else(|| Error::convert("Missing field `method.controller`"))?;

    let type_: MethodType = diff
      .type_
      .map(Diff::from_diff)
      .transpose()?
      .ok_or_else(|| Error::convert("Missing field `method.type`"))?;

    let data: MethodData = diff
      .data
      .map(Diff::from_diff)
      .transpose()?
      .ok_or_else(|| Error::convert("Missing field `method.data`"))?;

    let properties: Object = diff.properties.map(Diff::from_diff).transpose()?.unwrap_or_default();

    // Use builder to enforce invariants.
    MethodBuilder::new(properties)
      .id(id)
      .controller(controller)
      .type_(type_)
      .data(data)
      .build()
      .map_err(Error::convert)
  }

  fn into_diff(self) -> Result<Self::Type> {
    Ok(DiffMethod {
      id: Some(self.id.into_diff()?),
      controller: Some(self.controller.into_diff()?),
      type_: Some(self.type_.into_diff()?),
      data: Some(self.data.into_diff()?),
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
  use identity_core::common::Object;
  use identity_core::common::Value;
  use identity_core::convert::FromJson;
  use identity_core::convert::ToJson;
  use identity_core::diff::DiffString;

  use super::*;

  fn test_method() -> VerificationMethod {
    VerificationMethod::builder(Default::default())
      .id("did:example:123#key".parse().unwrap())
      .controller("did:example:123".parse().unwrap())
      .type_(MethodType::Ed25519VerificationKey2018)
      .data(MethodData::PublicKeyMultibase("".into()))
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
    assert!(diff.data.is_none());
    assert!(diff.type_.is_none());
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
    assert!(diff.data.is_none());
    assert!(diff.type_.is_none());

    let merge = method.merge(diff).unwrap();
    assert_eq!(merge, new);

    // add another property
    let mut properties = Object::new();
    properties.insert("key2".to_string(), Value::String("value1".to_string()));
    *new.properties_mut() = properties;

    let diff = method.diff(&new).unwrap();
    assert!(diff.id.is_none());
    assert!(diff.controller.is_none());
    assert!(diff.data.is_none());
    assert!(diff.type_.is_none());

    let merge = method.merge(diff).unwrap();
    assert_eq!(merge, new);

    // change property
    let mut properties = Object::new();
    properties.insert("key2".to_string(), Value::String("value2".to_string()));
    *new.properties_mut() = properties;

    let diff = method.diff(&new).unwrap();
    assert!(diff.id.is_none());
    assert!(diff.controller.is_none());
    assert!(diff.data.is_none());
    assert!(diff.type_.is_none());

    let merge = method.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_id() {
    let method = test_method();
    let mut new = method.clone();
    new.set_id("did:diff:123#key".parse().unwrap()).unwrap();

    let diff = method.diff(&new).unwrap();
    assert!(diff.controller.is_none());
    assert!(diff.data.is_none());
    assert!(diff.type_.is_none());
    assert!(diff.properties.is_none());
    assert_eq!(diff.id, Some(DiffString(Some("did:diff:123#key".to_string()))));

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
    assert!(diff.data.is_none());
    assert!(diff.type_.is_none());
    assert!(diff.properties.is_none());
    assert_eq!(diff.controller, Some(DiffString(Some("did:diff:123".to_string()))));

    let merge = method.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_type() {
    let method = test_method();
    let mut new = method.clone();
    *new.type_mut() = MethodType::X25519KeyAgreementKey2019;

    let diff = method.diff(&new).unwrap();
    assert!(diff.id.is_none());
    assert!(diff.controller.is_none());
    assert!(diff.data.is_none());
    assert!(diff.properties.is_none());
    assert_eq!(diff.type_, Some(MethodType::X25519KeyAgreementKey2019));

    let merge = method.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_data_base58() {
    let method = test_method();
    let mut new = method.clone();
    *new.data_mut() = MethodData::PublicKeyBase58("diff".into());

    let diff = method.diff(&new).unwrap();
    assert!(diff.id.is_none());
    assert!(diff.controller.is_none());
    assert!(diff.type_.is_none());
    assert!(diff.properties.is_none());
    assert_eq!(
      diff.data,
      Some(DiffMethodData::PublicKeyBase58(Some(DiffString(Some(
        "diff".to_string()
      )))))
    );

    let merge = method.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_data_multibase() {
    let method = test_method();
    let mut new = method.clone();
    *new.data_mut() = MethodData::PublicKeyMultibase("diff".into());

    let diff = method.diff(&new).unwrap();
    assert!(diff.id.is_none());
    assert!(diff.controller.is_none());
    assert!(diff.type_.is_none());
    assert!(diff.properties.is_none());
    assert_eq!(
      diff.data,
      Some(DiffMethodData::PublicKeyMultibase(Some(DiffString(Some(
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
    new.set_id("did:diff:123#key".parse().unwrap()).unwrap();
    let diff = method.diff(&new).unwrap();
    let diff_method = VerificationMethod::from_diff(diff);
    assert!(diff_method.is_err());

    // add controller
    *new.controller_mut() = "did:diff:123".parse().unwrap();
    let diff = method.diff(&new).unwrap();
    let diff_method = VerificationMethod::from_diff(diff);
    assert!(diff_method.is_err());

    // add type_
    *new.type_mut() = MethodType::X25519KeyAgreementKey2019;
    let diff = method.diff(&new).unwrap();
    let diff_method = VerificationMethod::from_diff(diff);
    assert!(diff_method.is_err());

    // add data
    *new.data_mut() = MethodData::PublicKeyMultibase("diff".into());
    let diff = method.diff(&new).unwrap();
    let diff_method = VerificationMethod::from_diff(diff.clone());
    assert!(diff_method.is_ok());
    let diff_method = diff_method.unwrap();
    assert_eq!(diff_method, new);

    let merge = method.merge(diff.clone()).unwrap();
    assert_eq!(merge, new);

    assert_eq!(new.into_diff().unwrap(), diff);
  }

  #[test]
  fn test_from_into_diff() {
    let method: VerificationMethod = test_method();

    let diff: DiffMethod = method.clone().into_diff().unwrap();
    let new: VerificationMethod = VerificationMethod::from_diff(diff.clone()).unwrap();
    assert_eq!(method, new);

    let ser: String = diff.to_json().unwrap();
    let de: DiffMethod = DiffMethod::from_json(&ser).unwrap();
    assert_eq!(de, diff);
    let from: VerificationMethod = VerificationMethod::from_diff(de).unwrap();
    assert_eq!(method, from);
  }
}
