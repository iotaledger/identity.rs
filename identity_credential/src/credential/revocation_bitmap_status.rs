// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use identity_core::common::Object;
use identity_core::common::Url;
use identity_core::common::Value;
use identity_did::did::DIDUrl;
use identity_did::did::DID;

use crate::credential::Status;
use crate::error::Error;
use crate::error::Result;

/// Information used to determine the current status of a [`Credential`][crate::credential::Credential]
/// using the `RevocationBitmap2022` specification.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RevocationBitmapStatus(Status);

impl RevocationBitmapStatus {
  const INDEX_PROPERTY_NAME: &'static str = "revocationBitmapIndex";
  /// Type name of the revocation bitmap.
  pub const TYPE: &'static str = "RevocationBitmap2022";

  /// Creates a new `RevocationBitmapStatus`.
  pub fn new<D: DID>(id: DIDUrl<D>, index: u32) -> Self {
    let mut object = Object::new();
    object.insert(Self::INDEX_PROPERTY_NAME.to_owned(), Value::String(index.to_string()));
    RevocationBitmapStatus(Status::new_with_properties(
      Url::from(id),
      Self::TYPE.to_owned(),
      object,
    ))
  }

  /// Returns the [`DIDUrl`] of the `RevocationBitmapStatus`, which should resolve
  /// to a `RevocationBitmap2022` service in a DID Document.
  pub fn id<D: DID>(&self) -> Result<DIDUrl<D>> {
    DIDUrl::parse(self.0.id.as_str())
      .map_err(|err| Error::InvalidStatus(format!("invalid DID Url '{}': {:?}", self.0.id, err)))
  }

  /// Returns the index of the credential in the issuer's revocation bitmap if it can be decoded.
  pub fn index(&self) -> Result<u32> {
    if let Some(Value::String(index)) = self.0.properties.get(Self::INDEX_PROPERTY_NAME) {
      u32::from_str(index).map_err(|err| {
        Error::InvalidStatus(format!(
          "expected {} to be an unsigned 32-bit integer: {}",
          Self::INDEX_PROPERTY_NAME,
          err
        ))
      })
    } else {
      Err(Error::InvalidStatus(format!(
        "expected {} to be an unsigned 32-bit integer expressed as a string",
        Self::INDEX_PROPERTY_NAME
      )))
    }
  }
}

impl TryFrom<Status> for RevocationBitmapStatus {
  type Error = Error;

  fn try_from(status: Status) -> Result<Self> {
    if status.type_ != Self::TYPE {
      Err(Error::InvalidStatus(format!(
        "expected type '{}', got '{}'",
        Self::TYPE,
        status.type_
      )))
    } else if !status.properties.contains_key(Self::INDEX_PROPERTY_NAME) {
      Err(Error::InvalidStatus(format!(
        "missing required property '{}'",
        Self::INDEX_PROPERTY_NAME
      )))
    } else {
      Ok(Self(status))
    }
  }
}

impl From<RevocationBitmapStatus> for Status {
  fn from(status: RevocationBitmapStatus) -> Self {
    status.0
  }
}

#[cfg(test)]
mod tests {
  use identity_core::common::Object;
  use identity_core::common::Url;
  use identity_core::common::Value;
  use identity_did::did::CoreDID;
  use identity_did::did::DIDUrl;

  use super::RevocationBitmapStatus;
  use super::Status;

  const TAG: &str = "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV";
  const SERVICE: &str = "revocation";

  #[test]
  fn test_embedded_status_invariants() {
    let url: Url = Url::parse(format!("did:iota:{}#{}", TAG, SERVICE)).unwrap();
    let did_url: DIDUrl<CoreDID> = DIDUrl::parse(url.clone().into_string()).unwrap();
    let revocation_list_index: u32 = 0;
    let embedded_revocation_status: RevocationBitmapStatus =
      RevocationBitmapStatus::new(did_url, revocation_list_index);

    let object: Object = Object::from([(
      RevocationBitmapStatus::INDEX_PROPERTY_NAME.to_owned(),
      Value::String(revocation_list_index.to_string()),
    )]);
    let status: Status =
      Status::new_with_properties(url.clone(), RevocationBitmapStatus::TYPE.to_owned(), object.clone());
    assert_eq!(embedded_revocation_status, status.try_into().unwrap());

    let status_missing_property: Status =
      Status::new_with_properties(url.clone(), RevocationBitmapStatus::TYPE.to_owned(), Object::new());
    assert!(RevocationBitmapStatus::try_from(status_missing_property).is_err());

    let status_wrong_type: Status = Status::new_with_properties(url, "DifferentType".to_owned(), object);
    assert!(RevocationBitmapStatus::try_from(status_wrong_type).is_err());
  }
}
