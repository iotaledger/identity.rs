// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use identity_core::common::Object;
use identity_core::common::Url;
use identity_core::common::Value;
use identity_did::did::DIDUrl;
use identity_did::did::DID;
use identity_did::service::BITMAP_REVOCATION_METHOD;

use super::Status;
use crate::error::Error;
use crate::error::Result;

const REVOCATION_LIST_INDEX: &str = "revocationListIndex";

/// Information used to determine the current status of a [`Credential`][crate::credential::Credential].
#[derive(Clone, Debug, PartialEq)]
pub struct BitmapRevocationStatus<D: DID + Sized> {
  /// A DID URL that can be resolved to a service of the issuer which contains the revocation list.
  pub id: DIDUrl<D>,
  /// The type of the revocation method.
  pub type_: String,
  /// The index of the revocation status of the verifiable credential.
  pub revocation_list_index: u32,
}

impl<D: DID + Sized> BitmapRevocationStatus<D> {
  /// Creates a new `EmbeddedRevocationStatus`.
  pub fn new(id: DIDUrl<D>, revocation_list_index: u32) -> Self {
    BitmapRevocationStatus {
      id,
      type_: BITMAP_REVOCATION_METHOD.to_owned(),
      revocation_list_index,
    }
  }
}

impl<D: DID + Sized> TryFrom<Status> for BitmapRevocationStatus<D> {
  type Error = Error;

  fn try_from(status: Status) -> Result<Self> {
    let id: DIDUrl<D> = DIDUrl::parse(&status.id.into_string())
      .map_err(|_| Error::InvalidStatus("invalid id - expected a valid did url"))?;

    if status.type_ != BITMAP_REVOCATION_METHOD {
      return Err(Error::InvalidStatus("invalid type - unexpected revocation method"));
    }

    let revocation_list_index: &Value = status
      .properties
      .get(REVOCATION_LIST_INDEX)
      .ok_or(Error::InvalidStatus("missing credential index property"))?;
    let revocation_list_index: u32 = {
      match revocation_list_index {
        Value::Array(_) | Value::Bool(_) | Value::Null | Value::Object(_) | Value::Number(_) => {
          return Err(Error::InvalidStatus(
            "credential index property must be an integer expressed as a string",
          ))
        }
        Value::String(index) => u32::from_str(index)
          .map_err(|_| Error::InvalidStatus("expected value greater or equal to zero and less than 2^32"))?,
      }
    };

    Ok(Self {
      id,
      type_: status.type_,
      revocation_list_index,
    })
  }
}

impl<D: DID + Sized> TryFrom<BitmapRevocationStatus<D>> for Status {
  type Error = Error;

  fn try_from(other: BitmapRevocationStatus<D>) -> Result<Self> {
    let object: Object = Object::from([(
      REVOCATION_LIST_INDEX.to_owned(),
      Value::String(other.revocation_list_index.to_string()),
    )]);
    let url =
      Url::parse(other.id.to_string()).map_err(|_| Error::InvalidStatus("invalid id - expected a valid url"))?;
    Ok(Status::new_with_properties(url, other.type_, object))
  }
}

#[cfg(test)]
mod tests {
  use identity_core::common::Object;
  use identity_core::common::Url;
  use identity_core::common::Value;
  use identity_did::did::CoreDID;
  use identity_did::did::DIDUrl;
  use identity_did::service::BITMAP_REVOCATION_METHOD;

  use super::BitmapRevocationStatus;
  use super::Status;
  use super::REVOCATION_LIST_INDEX;

  const TAG: &str = "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV";
  const SERVICE: &str = "revocation";

  #[test]
  fn test_embedded_status_invariants() {
    let url: Url = Url::parse(format!("did:iota:{}#{}", TAG, SERVICE)).unwrap();
    let iota_did_url: DIDUrl<CoreDID> = DIDUrl::parse(url.clone().into_string()).unwrap();
    let revocation_list_index: u32 = 0;
    let embedded_revocation_status = BitmapRevocationStatus::new(iota_did_url, revocation_list_index);

    let object: Object = Object::from([(
      REVOCATION_LIST_INDEX.to_owned(),
      Value::String(revocation_list_index.to_string()),
    )]);
    let status: Status = Status::new_with_properties(url.clone(), BITMAP_REVOCATION_METHOD.to_owned(), object.clone());
    assert_eq!(embedded_revocation_status, status.try_into().unwrap());

    let status_missing_property: Status =
      Status::new_with_properties(url.clone(), BITMAP_REVOCATION_METHOD.to_owned(), Object::new());
    assert!(BitmapRevocationStatus::<CoreDID>::try_from(status_missing_property).is_err());

    let status_wrong_type: Status = Status::new_with_properties(url, "DifferentType".to_owned(), object);
    assert!(BitmapRevocationStatus::<CoreDID>::try_from(status_wrong_type).is_err());
  }
}
