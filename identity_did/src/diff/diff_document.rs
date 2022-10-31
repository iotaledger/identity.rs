// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;
use serde::Serialize;

use identity_core::common::KeyComparable;
use identity_core::common::Object;
use identity_core::common::OneOrSet;
use identity_core::common::OrderedSet;
use identity_core::common::Url;
use identity_core::diff::Diff;
use identity_core::diff::DiffVec;
use identity_core::diff::Error;
use identity_core::diff::Result;

use crate::did::CoreDID;
use crate::did::DID;
use crate::document::CoreDocument;
use crate::document::CoreDocumentInner;
use crate::service::Service;
use crate::verification::MethodRef;
use crate::verification::VerificationMethod;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(bound(deserialize = ""))]
pub struct DiffDocument<D = CoreDID, T = Object, U = Object, V = Object>
where
  D: Diff + DID + KeyComparable + Serialize + for<'__de> Deserialize<'__de>,
  T: Diff + Serialize + for<'__de> Deserialize<'__de>,
  U: Diff + Serialize + for<'__de> Deserialize<'__de> + Default,
  V: Diff + Serialize + for<'__de> Deserialize<'__de> + Default,
{
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<<D as Diff>::Type>,
  #[serde(skip_serializing_if = "Option::is_none")]
  controller: Option<Option<DiffVec<D>>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  also_known_as: Option<DiffVec<Url>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  verification_method: Option<DiffVec<VerificationMethod<D, U>>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  authentication: Option<DiffVec<MethodRef<D, U>>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  assertion_method: Option<DiffVec<MethodRef<D, U>>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  key_agreement: Option<DiffVec<MethodRef<D, U>>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  capability_delegation: Option<DiffVec<MethodRef<D, U>>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  capability_invocation: Option<DiffVec<MethodRef<D, U>>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  service: Option<DiffVec<Service<D, V>>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  properties: Option<<T as Diff>::Type>,
}

impl<D, T, U, V> Diff for CoreDocument<D, T, U, V>
where
  D: DID + KeyComparable + Diff + Serialize + for<'de> Deserialize<'de>,
  T: Diff + Serialize + for<'de> Deserialize<'de> + Default,
  U: Diff + Serialize + for<'de> Deserialize<'de> + Default,
  V: Diff + Serialize + for<'de> Deserialize<'de> + Default,
{
  type Type = DiffDocument<D, T, U, V>;

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
          (Some(a), Some(b)) => Some(Some(a.diff(b)?)),
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
        Some(self.service().diff(other.service())?)
      },
      properties: if self.properties() == other.properties() {
        None
      } else {
        Some(self.properties().diff(other.properties())?)
      },
    })
  }

  fn merge(&self, diff: Self::Type) -> Result<Self> {
    let id: D = diff
      .id
      .map(|value| self.id().merge(value))
      .transpose()?
      .unwrap_or_else(|| self.id().clone());

    let controller: Option<OneOrSet<D>> = diff
      .controller
      .map(|value| match value {
        Some(diff_value) => self
          .controller()
          .map(|controller| controller.merge(diff_value))
          .transpose(),
        None => Ok(None),
      })
      .transpose()?
      .unwrap_or_else(|| self.controller().cloned());

    let also_known_as: OrderedSet<Url> = diff
      .also_known_as
      .map(|value| self.also_known_as().merge(value))
      .transpose()?
      .unwrap_or_else(|| self.also_known_as().clone());

    let verification_method: OrderedSet<VerificationMethod<D, U>> = diff
      .verification_method
      .map(|value| self.verification_method().merge(value))
      .transpose()?
      .unwrap_or_else(|| self.verification_method().clone());

    let authentication: OrderedSet<MethodRef<D, U>> = diff
      .authentication
      .map(|value| self.authentication().merge(value))
      .transpose()?
      .unwrap_or_else(|| self.authentication().clone());

    let assertion_method: OrderedSet<MethodRef<D, U>> = diff
      .assertion_method
      .map(|value| self.assertion_method().merge(value))
      .transpose()?
      .unwrap_or_else(|| self.assertion_method().clone());

    let key_agreement: OrderedSet<MethodRef<D, U>> = diff
      .key_agreement
      .map(|value| self.key_agreement().merge(value))
      .transpose()?
      .unwrap_or_else(|| self.key_agreement().clone());

    let capability_delegation: OrderedSet<MethodRef<D, U>> = diff
      .capability_delegation
      .map(|value| self.capability_delegation().merge(value))
      .transpose()?
      .unwrap_or_else(|| self.capability_delegation().clone());

    let capability_invocation: OrderedSet<MethodRef<D, U>> = diff
      .capability_invocation
      .map(|value| self.capability_invocation().merge(value))
      .transpose()?
      .unwrap_or_else(|| self.capability_invocation().clone());

    let service: OrderedSet<Service<D, V>> = diff
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
      inner: CoreDocumentInner {
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
      },
    })
  }

  fn from_diff(diff: Self::Type) -> Result<Self> {
    let id: D = diff
      .id
      .map(D::from_diff)
      .transpose()?
      .ok_or_else(|| Error::convert("Missing field `document.id`"))?;

    let controller: Option<OneOrSet<D>> = diff
      .controller
      .map(|diff| match diff {
        Some(diff) => Some(OneOrSet::from_diff(diff)).transpose(),
        None => Ok(None),
      })
      .transpose()?
      .ok_or_else(|| Error::convert("Missing field `document.controller`"))?;

    let also_known_as: OrderedSet<Url> = diff
      .also_known_as
      .map(Diff::from_diff)
      .transpose()?
      .ok_or_else(|| Error::convert("Missing field `document.also_known_as`"))?;

    let verification_method: OrderedSet<VerificationMethod<D, U>> = diff
      .verification_method
      .map(Diff::from_diff)
      .transpose()?
      .ok_or_else(|| Error::convert("Missing field `document.verification_method`"))?;

    let authentication: OrderedSet<MethodRef<D, U>> = diff
      .authentication
      .map(Diff::from_diff)
      .transpose()?
      .ok_or_else(|| Error::convert("Missing field `document.authentication`"))?;

    let assertion_method: OrderedSet<MethodRef<D, U>> = diff
      .assertion_method
      .map(Diff::from_diff)
      .transpose()?
      .ok_or_else(|| Error::convert("Missing field `document.assertion_method`"))?;

    let key_agreement: OrderedSet<MethodRef<D, U>> = diff
      .key_agreement
      .map(Diff::from_diff)
      .transpose()?
      .ok_or_else(|| Error::convert("Missing field `document.key_agreement`"))?;

    let capability_delegation: OrderedSet<MethodRef<D, U>> = diff
      .capability_delegation
      .map(Diff::from_diff)
      .transpose()?
      .ok_or_else(|| Error::convert("Missing field `document.capability_delegation`"))?;

    let capability_invocation: OrderedSet<MethodRef<D, U>> = diff
      .capability_invocation
      .map(Diff::from_diff)
      .transpose()?
      .ok_or_else(|| Error::convert("Missing field `document.capability_invocation`"))?;

    let service: OrderedSet<Service<D, V>> = diff
      .service
      .map(Diff::from_diff)
      .transpose()?
      .ok_or_else(|| Error::convert("Missing field `document.service`"))?;

    let properties: T = diff.properties.map(T::from_diff).transpose()?.unwrap_or_default();

    Ok(CoreDocument {
      inner: CoreDocumentInner {
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
      },
    })
  }

  fn into_diff(self) -> Result<Self::Type> {
    let inner = self.inner;

    Ok(DiffDocument {
      id: Some(inner.id.into_diff()?),
      controller: Some(inner.controller.map(|value| value.into_diff()).transpose()?),
      also_known_as: Some(inner.also_known_as.into_diff()?),
      verification_method: Some(inner.verification_method.into_diff()?),
      authentication: Some(inner.authentication.into_diff()?),
      assertion_method: Some(inner.assertion_method.into_diff()?),
      key_agreement: Some(inner.key_agreement.into_diff()?),
      capability_delegation: Some(inner.capability_delegation.into_diff()?),
      capability_invocation: Some(inner.capability_invocation.into_diff()?),
      service: Some(inner.service.into_diff()?),
      properties: if inner.properties == Default::default() {
        None
      } else {
        Some(inner.properties.into_diff()?)
      },
    })
  }
}

#[cfg(test)]
mod test {
  use std::collections::BTreeMap;

  use identity_core::common::Value;
  use identity_core::convert::FromJson;
  use identity_core::convert::ToJson;
  use identity_core::diff::DiffString;

  use crate::did::CoreDIDUrl;
  use crate::did::DID;
  use crate::service::ServiceBuilder;
  use crate::service::ServiceEndpoint;
  use crate::verification::MethodBuilder;
  use crate::verification::MethodData;
  use crate::verification::MethodType;

  use super::*;

  fn controller() -> CoreDID {
    "did:example:1234".parse().unwrap()
  }

  fn method(controller: &CoreDID, fragment: &str) -> VerificationMethod {
    MethodBuilder::default()
      .id(controller.to_url().join(fragment).unwrap())
      .controller(controller.clone())
      .type_(MethodType::Ed25519VerificationKey2018)
      .data(MethodData::new_multibase(fragment.as_bytes()))
      .build()
      .unwrap()
  }

  fn service(did_url: CoreDIDUrl) -> Service {
    ServiceBuilder::default()
      .id(did_url)
      .service_endpoint(ServiceEndpoint::One(Url::parse("did:service:1234").unwrap()))
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
      .authentication(controller.to_url().join("#key-3").unwrap())
      .key_agreement(controller.to_url().join("#key-4").unwrap())
      .assertion_method(method(&controller, "#key-5"))
      .capability_delegation(method(&controller, "#key-6"))
      .capability_invocation(method(&controller, "#key-7"))
      .service(service(controller.to_url().join("#service").unwrap()))
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
  fn test_controller_one() {
    let doc: CoreDocument = document();
    let mut new: CoreDocument = doc.clone();
    let new_controller: CoreDID = "did:diff:1234".parse().unwrap();
    *new.controller_mut() = Some(OneOrSet::new_one(new_controller));
    assert_ne!(doc, new);

    let diff: DiffDocument = doc.diff(&new).unwrap();
    let merge: CoreDocument = doc.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_controller_set() {
    let doc: CoreDocument = document();
    let mut new: CoreDocument = doc.clone();
    let new_controllers: Vec<CoreDID> = vec![
      "did:diff:1234".parse().unwrap(),
      "did:diff:5678".parse().unwrap(),
      "did:diff:9012".parse().unwrap(),
    ];
    *new.controller_mut() = Some(new_controllers.try_into().unwrap());
    assert_ne!(doc, new);

    let diff: DiffDocument = doc.diff(&new).unwrap();
    let merge: CoreDocument = doc.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_controller_unset() {
    let doc: CoreDocument = document();
    let mut new: CoreDocument = doc.clone();
    *new.controller_mut() = None;
    assert_ne!(doc, new);

    let diff: DiffDocument = doc.diff(&new).unwrap();
    let merge: CoreDocument = doc.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_also_known_as() {
    let doc = document();
    let mut new = doc.clone();
    new.also_known_as_mut().append("diff:diff:1234".parse().unwrap());
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
    assert!(new.verification_method_mut().append(method(&doc.inner.id, "#key-diff")));
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
      .replace(&first, method(&"did:diff:1234".parse().unwrap(), "#key-diff"));
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
    let method_ref: MethodRef = method(&doc.inner.id, "#key-diff").into();
    assert!(new.authentication_mut().append(method_ref));
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
    let method_ref: MethodRef = method(&doc.inner.id, "#key-diff").into();
    let first = new.authentication().first().unwrap().clone();
    new.authentication_mut().replace(&first, method_ref);
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
    let method_ref: MethodRef = method(&doc.inner.id, "#key-diff").into();
    assert!(new.assertion_method_mut().append(method_ref));
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
    let method_ref: MethodRef = method(&doc.inner.id, "#key-diff").into();
    let first = new.assertion_method().first().unwrap().clone();
    new.assertion_method_mut().replace(&first, method_ref);
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
    let method_ref: MethodRef = method(&doc.inner.id, "#key-diff").into();
    assert!(new.key_agreement_mut().append(method_ref));
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
    let method_ref: MethodRef = method(&doc.inner.id, "#key-diff").into();
    let first = new.key_agreement().first().unwrap().clone();
    new.key_agreement_mut().replace(&first, method_ref);
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
    let method_ref: MethodRef = method(&doc.inner.id, "#key-diff").into();
    assert!(new.capability_delegation_mut().append(method_ref));
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
    let method_ref: MethodRef = method(&doc.inner.id, "#key-diff").into();
    let first = new.capability_delegation().first().unwrap().clone();
    new.capability_delegation_mut().replace(&first, method_ref);
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
    let method_ref: MethodRef = method(&doc.inner.id, "#key-diff").into();
    assert!(new.capability_invocation_mut().append(method_ref));
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
    let method_ref: MethodRef = method(&doc.inner.id, "#key-diff").into();
    let first = new.capability_invocation().first().unwrap().clone();
    new.capability_invocation_mut().replace(&first, method_ref);
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

    // Add new service
    let service = service(doc.inner.id.to_url().join("#key-diff").unwrap());
    assert!(new.service_mut().append(service));
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
    let service = service(doc.inner.id.to_url().join("#key-diff").unwrap());
    let first = new.service().first().unwrap().clone();
    new.service_mut().replace(&first, service);
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
  fn test_from_into_diff() {
    let doc: CoreDocument = document();

    let diff: DiffDocument = doc.clone().into_diff().unwrap();
    let new: CoreDocument = CoreDocument::from_diff(diff.clone()).unwrap();
    assert_eq!(doc, new);

    let ser: String = diff.to_json().unwrap();
    let de: DiffDocument = DiffDocument::from_json(&ser).unwrap();
    assert_eq!(de, diff);
    let from: CoreDocument = CoreDocument::from_diff(de).unwrap();
    assert_eq!(doc, from);
  }

  #[test]
  fn test_rotate_key_material_method() {
    let doc = document();
    let mut new = doc.clone();

    let first: CoreDIDUrl = new.capability_invocation().first().unwrap().as_ref().clone();
    new.capability_invocation_mut().remove(&first);

    let method_ref: MethodRef = MethodBuilder::default()
      .id(first)
      .controller(new.inner.id.clone())
      .type_(MethodType::Ed25519VerificationKey2018)
      .data(MethodData::new_multibase(b"key_material"))
      .build()
      .unwrap()
      .into();

    assert!(new.capability_invocation_mut().append(method_ref));

    assert_ne!(doc, new);

    // Ensure overwriting the key material of a method with the same fragment produces a diff.
    let diff = doc.diff(&new).unwrap();
    assert!(diff.capability_invocation.is_some());
  }
}
