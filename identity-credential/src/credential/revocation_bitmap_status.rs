// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use identity_core::common::Object;
use identity_core::common::Url;
use identity_core::common::Value;
use identity_did::did::DIDUrl;
use identity_did::did::DID;

use super::Status;
use crate::error::Error;
use crate::error::Result;

/// Information used to determine the current status of a [`Credential`][crate::credential::Credential].
#[derive(Clone, Debug, PartialEq)]
pub struct RevocationBitmapStatus(Status);

impl RevocationBitmapStatus {
  const INDEX_PROPERTY_NAME: &'static str = "revocationBitmapIndex";
  /// The type name of the revocation bitmap.
  pub const TYPE: &'static str = "RevocationBitmap2022";

  /// Creates a new `RevocationBitmapStatus`.
  pub fn new<D: DID>(id: DIDUrl<D>, revocation_bitmap_index: u32) -> Result<Self> {
    let mut object = Object::new();
    object.insert(
      Self::INDEX_PROPERTY_NAME.to_owned(),
      Value::String(revocation_bitmap_index.to_string()),
    );
    Ok(RevocationBitmapStatus(Status::new_with_properties(
      Url::parse(id.to_string()).map_err(|_| Error::InvalidStatus("invalid url"))?,
      Self::TYPE.to_owned(),
      object,
    )))
  }

  /// Returns the [`DIDUrl`] of the bitmap status.
  pub fn id<D: DID>(&self) -> Result<DIDUrl<D>> {
    DIDUrl::parse(self.0.id.as_str()).map_err(|_| Error::InvalidStatus("invalid did url"))
  }

  /// Returns the index of the credential in the issuer's revocation bitmap if it can be decoded.
  pub fn index(&self) -> Result<u32> {
    if let Some(Value::String(index)) = self.0.properties.get(Self::INDEX_PROPERTY_NAME) {
      u32::from_str(index)
        .map_err(|_| Error::InvalidStatus("expected the revocation bitmap index to be an unsigned 32-bit integer"))
    } else {
      Err(Error::InvalidStatus(
        "credential index property must be an integer expressed as a string",
      ))
    }
  }
}

impl TryFrom<Status> for RevocationBitmapStatus {
  type Error = Error;

  fn try_from(status: Status) -> Result<Self> {
    if status.type_ != Self::TYPE {
      Err(Error::InvalidStatus("expected `RevocationBitmap2022`"))
    } else if !status.properties.contains_key(Self::INDEX_PROPERTY_NAME) {
      Err(Error::InvalidStatus(
        "status is missing required property `revocationBitmapIndex`",
      ))
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
      RevocationBitmapStatus::new(did_url, revocation_list_index).unwrap();

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
