// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Object;
use identity_core::common::Url;
use identity_core::diff::Diff;
use identity_core::diff::DiffString;
use identity_core::diff::DiffVec;
use identity_core::diff::Error;
use identity_core::diff::Result;
use serde::Deserialize;
use serde::Serialize;

use crate::did::DID;
use crate::document::CoreDocument;
use crate::service::Service;
use crate::utils::DIDKey;
use crate::utils::OrderedSet;
use crate::verification::MethodRef;
use crate::verification::VerificationMethod;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(bound(deserialize = ""))]
pub struct DiffDocument<T = Object, U = Object, V = Object>
where
  T: Diff + Serialize + for<'__de> Deserialize<'__de>,
  U: Diff + Serialize + for<'__de> Deserialize<'__de> + Default,
  V: Diff + Serialize + for<'__de> Deserialize<'__de> + Default,
{
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<DiffString>,
  #[serde(skip_serializing_if = "Option::is_none")]
  controller: Option<Option<DiffString>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  also_known_as: Option<DiffVec<Url>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  verification_method: Option<DiffVec<DIDKey<VerificationMethod<U>>>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  authentication: Option<DiffVec<DIDKey<MethodRef<U>>>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  assertion_method: Option<DiffVec<DIDKey<MethodRef<U>>>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  key_agreement: Option<DiffVec<DIDKey<MethodRef<U>>>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  capability_delegation: Option<DiffVec<DIDKey<MethodRef<U>>>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  capability_invocation: Option<DiffVec<DIDKey<MethodRef<U>>>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  service: Option<DiffVec<DIDKey<Service<V>>>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  properties: Option<<T as Diff>::Type>,
}

impl<T, U, V> Diff for CoreDocument<T, U, V>
where
  T: Diff + Serialize + for<'de> Deserialize<'de>,
  U: Diff + Serialize + for<'de> Deserialize<'de> + Default,
  V: Diff + Serialize + for<'de> Deserialize<'de> + Default,
{
  type Type = DiffDocument<T, U, V>;

  fn diff(&self, other: &Self) -> Result<Self::Type> {
    Ok(DiffDocument {
      id: if self.id() == other.id() {
        None
      } else {
        Some(self.id().diff(other.id())?)
      },
      controller: if self.controller() == other.controller() {
        None
      } else {
        match (self.controller(), other.controller()) {
          (Some(a), Some(b)) => Some(Some(a.diff(&b)?)),
          (None, Some(b)) => Some(Some(b.clone().into_diff()?)),
          _ => Some(None),
        }
      },
      also_known_as: if self.also_known_as() == other.also_known_as() {
        None
      } else {
        Some(self.also_known_as().to_vec().diff(&other.also_known_as().to_vec())?)
      },
      verification_method: if self.verification_method() == other.verification_method() {
        None
      } else {
        Some(self.verification_method().diff(other.verification_method())?)
      },
      authentication: if self.authentication() == other.authentication() {
        None
      } else {
        Some(self.authentication().diff(other.authentication())?)
      },
      assertion_method: if self.assertion_method() == other.assertion_method() {
        None
      } else {
        Some(self.assertion_method().diff(other.assertion_method())?)
      },
      key_agreement: if self.key_agreement() == other.key_agreement() {
        None
      } else {
        Some(self.key_agreement().diff(other.key_agreement())?)
      },
      capability_delegation: if self.capability_delegation() == other.capability_delegation() {
        None
      } else {
        Some(self.capability_delegation().diff(other.capability_delegation())?)
      },
      capability_invocation: if self.capability_invocation() == other.capability_invocation() {
        None
      } else {
        Some(self.capability_invocation().diff(other.capability_invocation())?)
      },
      service: if self.service() == other.service() {
        None
      } else {
        Some(self.service().diff(&other.service())?)
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

    let controller: Option<DID> = diff
      .controller
      .flatten()
      .and_then(|value| self.controller().map(|controller| controller.merge(value)))
      .transpose()?
      .or_else(|| self.controller().cloned());

    let also_known_as: Vec<Url> = diff
      .also_known_as
      .map(|value| self.also_known_as().to_vec().merge(value))
      .transpose()?
      .unwrap_or_else(|| self.also_known_as().to_vec());

    let verification_method: OrderedSet<DIDKey<VerificationMethod<U>>> = diff
      .verification_method
      .map(|value| self.verification_method().merge(value))
      .transpose()?
      .unwrap_or_else(|| self.verification_method().clone());

    let authentication: OrderedSet<DIDKey<MethodRef<U>>> = diff
      .authentication
      .map(|value| self.authentication().merge(value))
      .transpose()?
      .unwrap_or_else(|| self.authentication().clone());

    let assertion_method: OrderedSet<DIDKey<MethodRef<U>>> = diff
      .assertion_method
      .map(|value| self.assertion_method().merge(value))
      .transpose()?
      .unwrap_or_else(|| self.assertion_method().clone());

    let key_agreement: OrderedSet<DIDKey<MethodRef<U>>> = diff
      .key_agreement
      .map(|value| self.key_agreement().merge(value))
      .transpose()?
      .unwrap_or_else(|| self.key_agreement().clone());

    let capability_delegation: OrderedSet<DIDKey<MethodRef<U>>> = diff
      .capability_delegation
      .map(|value| self.capability_delegation().merge(value))
      .transpose()?
      .unwrap_or_else(|| self.capability_delegation().clone());

    let capability_invocation: OrderedSet<DIDKey<MethodRef<U>>> = diff
      .capability_invocation
      .map(|value| self.capability_invocation().merge(value))
      .transpose()?
      .unwrap_or_else(|| self.capability_invocation().clone());

    let service: OrderedSet<DIDKey<Service<V>>> = diff
      .service
      .map(|value| self.service().merge(value))
      .transpose()?
      .unwrap_or_else(|| self.service().clone());

    let properties: T = diff
      .properties
      .map(|value| self.properties().merge(value))
      .transpose()?
      .unwrap_or_else(|| self.properties().clone());

    Ok(CoreDocument {
      id,
      controller,
      also_known_as,
      verification_method,
      authentication,
      assertion_method,
      key_agreement,
      capability_delegation,
      capability_invocation,
      service,
      properties,
    })
  }

  fn from_diff(diff: Self::Type) -> Result<Self> {
    let id: DID = diff
      .id
      .map(DID::from_diff)
      .transpose()?
      .ok_or_else(|| Error::convert("Missing field `document.id`"))?;

    let controller: Option<DID> = diff
      .controller
      .map(|diff| match diff {
        Some(diff) => Some(DID::from_diff(diff)).transpose(),
        None => Ok(None),
      })
      .transpose()?
      .ok_or_else(|| Error::convert("Missing field `document.controller`"))?;

    let also_known_as: Vec<Url> = diff
      .also_known_as
      .map(Diff::from_diff)
      .transpose()?
      .ok_or_else(|| Error::convert("Missing field `document.also_known_as`"))?;

    let verification_method: OrderedSet<DIDKey<VerificationMethod<U>>> = diff
      .verification_method
      .map(Diff::from_diff)
      .transpose()?
      .ok_or_else(|| Error::convert("Missing field `document.verification_method`"))?;

    let authentication: OrderedSet<DIDKey<MethodRef<U>>> = diff
      .authentication
      .map(Diff::from_diff)
      .transpose()?
      .ok_or_else(|| Error::convert("Missing field `document.authentication`"))?;

    let assertion_method: OrderedSet<DIDKey<MethodRef<U>>> = diff
      .assertion_method
      .map(Diff::from_diff)
      .transpose()?
      .ok_or_else(|| Error::convert("Missing field `document.assertion_method`"))?;

    let key_agreement: OrderedSet<DIDKey<MethodRef<U>>> = diff
      .key_agreement
      .map(Diff::from_diff)
      .transpose()?
      .ok_or_else(|| Error::convert("Missing field `document.key_agreement`"))?;

    let capability_delegation: OrderedSet<DIDKey<MethodRef<U>>> = diff
      .capability_delegation
      .map(Diff::from_diff)
      .transpose()?
      .ok_or_else(|| Error::convert("Missing field `document.capability_delegation`"))?;

    let capability_invocation: OrderedSet<DIDKey<MethodRef<U>>> = diff
      .capability_invocation
      .map(Diff::from_diff)
      .transpose()?
      .ok_or_else(|| Error::convert("Missing field `document.capability_invocation`"))?;

    let service: OrderedSet<DIDKey<Service<V>>> = diff
      .service
      .map(Diff::from_diff)
      .transpose()?
      .ok_or_else(|| Error::convert("Missing field `document.service`"))?;

    let properties: T = diff
      .properties
      .map(T::from_diff)
      .transpose()?
      .ok_or_else(|| Error::convert("Missing field `document.properties`"))?;

    Ok(CoreDocument {
      id,
      controller,
      also_known_as,
      verification_method,
      authentication,
      assertion_method,
      key_agreement,
      capability_delegation,
      capability_invocation,
      service,
      properties,
    })
  }

  fn into_diff(self) -> Result<Self::Type> {
    Ok(DiffDocument {
      id: Some(self.id().clone().into_diff()?),
      controller: Some(self.controller().cloned().map(|value| value.into_diff()).transpose()?),
      also_known_as: Some(self.also_known_as().to_vec().into_diff()?),
      verification_method: Some(self.verification_method().to_vec().into_diff()?),
      authentication: Some(self.authentication().to_vec().into_diff()?),
      assertion_method: Some(self.assertion_method().to_vec().into_diff()?),
      key_agreement: Some(self.key_agreement().to_vec().into_diff()?),
      capability_delegation: Some(self.capability_delegation().to_vec().into_diff()?),
      capability_invocation: Some(self.capability_invocation().to_vec().into_diff()?),
      service: Some(self.service().to_vec().into_diff()?),
      properties: Some(self.properties().clone().into_diff()?),
    })
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::service::ServiceBuilder;
  use crate::verification::MethodBuilder;
  use crate::verification::MethodData;
  use crate::verification::MethodType;
  use identity_core::common::Value;
  use std::collections::BTreeMap;

  fn controller() -> DID {
    "did:example:1234".parse().unwrap()
  }

  fn method(controller: &DID, fragment: &str) -> VerificationMethod {
    MethodBuilder::default()
      .id(controller.join(fragment).unwrap())
      .controller(controller.clone())
      .key_type(MethodType::Ed25519VerificationKey2018)
      .key_data(MethodData::new_b58(fragment.as_bytes()))
      .build()
      .unwrap()
  }
  fn service(controller: &DID) -> Service {
    ServiceBuilder::default()
      .id(controller.clone())
      .service_endpoint(Url::parse("did:service:1234").unwrap())
      .type_("test_service")
      .build()
      .unwrap()
  }
  fn document() -> CoreDocument {
    let controller = controller();
    let mut properties: BTreeMap<String, Value> = BTreeMap::default();
    properties.insert("key1".to_string(), "value1".into());

    CoreDocument::builder(properties)
      .id(controller.clone())
      .controller(controller.clone())
      .verification_method(method(&controller, "#key-1"))
      .verification_method(method(&controller, "#key-2"))
      .verification_method(method(&controller, "#key-3"))
      .authentication(method(&controller, "#auth-key"))
      .authentication(controller.join("#key-3").unwrap())
      .key_agreement(controller.join("#key-4").unwrap())
      .assertion_method(method(&controller, "#key-5"))
      .capability_delegation(method(&controller, "#key-6"))
      .capability_invocation(method(&controller, "#key-7"))
      .service(service(&controller))
      .build()
      .unwrap()
  }

  #[test]
  fn test_id() {
    let doc = document();
    let mut new = doc.clone();
    let new_did = "did:diff:1234";
    *new.id_mut() = new_did.parse().unwrap();
    assert_ne!(doc, new);

    let diff = doc.diff(&new).unwrap();
    assert_eq!(diff.id, Some(DiffString(Some(new_did.to_string()))));
    let merge = doc.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_controller() {
    let doc = document();
    let mut new = doc.clone();
    let new_controller: DID = "did:diff:1234".parse().unwrap();
    *new.controller_mut().unwrap() = new_controller.clone();
    assert_ne!(doc, new);

    let diff = doc.diff(&new).unwrap();
    assert_eq!(
      diff.clone().controller.unwrap(),
      Some(DiffString(Some(new_controller.to_string())))
    );
    let merge = doc.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_also_known_as() {
    let doc = document();
    let mut new = doc.clone();
    new.also_known_as_mut().push("diff:diff:1234".parse().unwrap());
    assert_ne!(doc, new);
    let diff = doc.diff(&new).unwrap();
    let merge = doc.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_add_verification_method() {
    let doc = document();
    let mut new = doc.clone();

    // add new method
    assert!(new
      .verification_method_mut()
      .append(method(&doc.clone().controller.unwrap(), "#key-diff").into()));
    assert_ne!(doc, new);
    let diff = doc.diff(&new).unwrap();
    let merge = doc.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_replace_verification_method() {
    let doc = document();
    let mut new = doc.clone();

    // update method
    let first = new.verification_method().first().unwrap().clone();
    new
      .verification_method_mut()
      .replace(&first, method(&"did:diff:1234".parse().unwrap(), "#key-diff").into());
    assert_ne!(doc, new);
    let diff = doc.diff(&new).unwrap();
    let merge = doc.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_remove_verification_method() {
    let doc = document();
    let mut new = doc.clone();

    // remove method
    let first = new.verification_method().first().unwrap().clone();
    new.verification_method_mut().remove(&first);
    assert_ne!(doc, new);
    let diff = doc.diff(&new).unwrap();
    let merge = doc.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_add_authentication() {
    let doc = document();
    let mut new = doc.clone();

    // add new method
    let method_ref: MethodRef = method(&doc.clone().controller.unwrap(), "#key-diff").into();
    assert!(new.authentication_mut().append(method_ref.into()));
    assert_ne!(doc, new);
    let diff = doc.diff(&new).unwrap();
    let merge = doc.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_replace_authentication() {
    let doc = document();
    let mut new = doc.clone();

    // update method
    let method_ref: MethodRef = method(&doc.clone().controller.unwrap(), "#key-diff").into();
    let first = new.authentication().first().unwrap().clone();
    new.authentication_mut().replace(&first, method_ref.into());
    assert_ne!(doc, new);
    let diff = doc.diff(&new).unwrap();
    let merge = doc.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_remove_authentication() {
    let doc = document();
    let mut new = doc.clone();

    // remove method
    let first = new.authentication().first().unwrap().clone();
    new.authentication_mut().remove(&first);
    assert_ne!(doc, new);
    let diff = doc.diff(&new).unwrap();
    let merge = doc.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_add_assertion_method() {
    let doc = document();
    let mut new = doc.clone();

    // add new method
    let method_ref: MethodRef = method(&doc.clone().controller.unwrap(), "#key-diff").into();
    assert!(new.assertion_method_mut().append(method_ref.into()));
    assert_ne!(doc, new);
    let diff = doc.diff(&new).unwrap();
    let merge = doc.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_replace_assertion_method() {
    let doc = document();
    let mut new = doc.clone();

    // update method
    let method_ref: MethodRef = method(&doc.clone().controller.unwrap(), "#key-diff").into();
    let first = new.assertion_method().first().unwrap().clone();
    new.assertion_method_mut().replace(&first, method_ref.into());
    assert_ne!(doc, new);
    let diff = doc.diff(&new).unwrap();
    let merge = doc.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_remove_assertion_method() {
    let doc = document();
    let mut new = doc.clone();

    // remove method
    let first = new.assertion_method().first().unwrap().clone();
    new.assertion_method_mut().remove(&first);
    assert_ne!(doc, new);
    let diff = doc.diff(&new).unwrap();
    let merge = doc.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_add_key_agreement() {
    let doc = document();
    let mut new = doc.clone();

    // add new method
    let method_ref: MethodRef = method(&doc.clone().controller.unwrap(), "#key-diff").into();
    assert!(new.key_agreement_mut().append(method_ref.into()));
    assert_ne!(doc, new);
    let diff = doc.diff(&new).unwrap();
    let merge = doc.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_replace_key_agreement() {
    let doc = document();
    let mut new = doc.clone();

    // update method
    let method_ref: MethodRef = method(&doc.clone().controller.unwrap(), "#key-diff").into();
    let first = new.key_agreement().first().unwrap().clone();
    new.key_agreement_mut().replace(&first, method_ref.into());
    assert_ne!(doc, new);
    let diff = doc.diff(&new).unwrap();
    let merge = doc.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_remove_key_agreement() {
    let doc = document();
    let mut new = doc.clone();

    // remove method
    let first = new.key_agreement().first().unwrap().clone();
    new.key_agreement_mut().remove(&first);
    assert_ne!(doc, new);
    let diff = doc.diff(&new).unwrap();
    let merge = doc.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_add_capability_delegation() {
    let doc = document();
    let mut new = doc.clone();

    // add new method
    let method_ref: MethodRef = method(&doc.clone().controller.unwrap(), "#key-diff").into();
    assert!(new.capability_delegation_mut().append(method_ref.into()));
    assert_ne!(doc, new);
    let diff = doc.diff(&new).unwrap();
    let merge = doc.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_replace_capability_delegation() {
    let doc = document();
    let mut new = doc.clone();

    // update method
    let method_ref: MethodRef = method(&doc.clone().controller.unwrap(), "#key-diff").into();
    let first = new.capability_delegation().first().unwrap().clone();
    new.capability_delegation_mut().replace(&first, method_ref.into());
    assert_ne!(doc, new);
    let diff = doc.diff(&new).unwrap();
    let merge = doc.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_remove_capability_delegation() {
    let doc = document();
    let mut new = doc.clone();

    // remove method
    let first = new.capability_delegation().first().unwrap().clone();
    new.capability_delegation_mut().remove(&first);
    assert_ne!(doc, new);
    let diff = doc.diff(&new).unwrap();
    let merge = doc.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_add_capability_invocation() {
    let doc = document();
    let mut new = doc.clone();

    // add new method
    let method_ref: MethodRef = method(&doc.clone().controller.unwrap(), "#key-diff").into();
    assert!(new.capability_invocation_mut().append(method_ref.into()));
    assert_ne!(doc, new);
    let diff = doc.diff(&new).unwrap();
    let merge = doc.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_replace_capability_invocation() {
    let doc = document();
    let mut new = doc.clone();

    // update method
    let method_ref: MethodRef = method(&doc.clone().controller.unwrap(), "#key-diff").into();
    let first = new.capability_invocation().first().unwrap().clone();
    new.capability_invocation_mut().replace(&first, method_ref.into());
    assert_ne!(doc, new);
    let diff = doc.diff(&new).unwrap();
    let merge = doc.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_remove_capability_invocation() {
    let doc = document();
    let mut new = doc.clone();

    // remove method
    let first = new.capability_invocation().first().unwrap().clone();
    new.capability_invocation_mut().remove(&first);
    assert_ne!(doc, new);
    let diff = doc.diff(&new).unwrap();
    let merge = doc.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_add_service() {
    let doc = document();
    let mut new = doc.clone();

    // add new service
    let service = service(&doc.clone().controller.unwrap().join("#key-diff").unwrap());
    assert!(new.service_mut().append(service.into()));
    assert_ne!(doc, new);
    let diff = doc.diff(&new).unwrap();
    let merge = doc.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_replace_service() {
    let doc = document();
    let mut new = doc.clone();

    // add new service
    let service = service(&doc.clone().controller.unwrap().join("#key-diff").unwrap());
    let first = new.service().first().unwrap().clone();
    new.service_mut().replace(&first, service.into());
    assert_ne!(doc, new);
    let diff = doc.diff(&new).unwrap();
    let merge = doc.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_remove_service() {
    let doc = document();
    let mut new = doc.clone();

    // remove method
    let first = new.service().first().unwrap().clone();
    new.service_mut().remove(&first);
    assert_ne!(doc, new);
    let diff = doc.diff(&new).unwrap();
    let merge = doc.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_replace_properties() {
    let doc = document();
    let mut new = doc.clone();

    // update properties
    *new.properties_mut() = BTreeMap::default();

    assert_ne!(doc, new);
    let diff = doc.diff(&new).unwrap();
    let merge = doc.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_add_properties() {
    let doc = document();
    let mut new = doc.clone();

    // update properties
    assert!(new
      .properties_mut()
      .insert("key2".to_string(), "value2".into())
      .is_none());

    assert_ne!(doc, new);
    let diff = doc.diff(&new).unwrap();
    let merge = doc.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_from_into_roundtrip() {
    let doc = document();

    let diff = doc.clone().into_diff().unwrap();
    let new = CoreDocument::from_diff(diff).unwrap();
    assert_eq!(doc, new);
  }
}
