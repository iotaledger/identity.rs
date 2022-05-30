// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use identity_core::common::Object;
use identity_core::common::Url;
use identity_core::common::Value;
use identity_credential::credential::Status;
use serde::Deserialize;
use serde::Serialize;

use super::error::Result;
use super::CredentialStatusError;
use crate::did::IotaDIDUrl;
use crate::revocation::EmbeddedRevocationList;

/// Information used to determine the current status of a [`Credential`][crate::credential::Credential].
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EmbeddedRevocationStatus {
  /// A DID URL that can be resolved to a service of the issuer which contains the revocation list.
  pub id: IotaDIDUrl,
  /// The string indentifying the revocation method.
  #[serde(rename = "type")]
  pub types: String,
  /// The index of the revocation status of the verifiable credential.
  #[serde(rename = "revocationListIndex")]
  pub revocation_list_index: u32,
}

impl EmbeddedRevocationStatus {
  /// Creates a new `EmbeddedRevocationStatus`.
  pub fn new(id: IotaDIDUrl, revocation_list_index: u32) -> Self {
    EmbeddedRevocationStatus {
      id,
      types: EmbeddedRevocationList::name().to_owned(),
      revocation_list_index,
    }
  }
}

impl TryFrom<Status> for EmbeddedRevocationStatus {
  type Error = CredentialStatusError;

  fn try_from(status: Status) -> Result<Self> {
    let expected_type: &str = EmbeddedRevocationList::name();
    let index_property: &str = EmbeddedRevocationList::credential_list_index_property();

    let url_string: String = status.id.into_string();
    let id: IotaDIDUrl =
      IotaDIDUrl::parse(&url_string).map_err(|_| CredentialStatusError::InvalidStatusId(url_string.to_owned()))?;

    let types: String = status
      .types
      .ok_or_else(|| CredentialStatusError::InvalidStatusType(format!("expected {}", expected_type)))?;
    if types != expected_type {
      return Err(CredentialStatusError::InvalidStatusType(format!(
        "expected {}",
        expected_type
      )));
    }

    let revocation_list_index: &Value = status
      .properties
      .get(index_property)
      .ok_or_else(|| CredentialStatusError::InvalidStatusIndex(format!("missing {} property", index_property)))?;
    let revocation_list_index: u32 = {
      match revocation_list_index {
        Value::Array(_) | Value::Bool(_) | Value::Null | Value::Object(_) | Value::Number(_) => {
          return Err(CredentialStatusError::InvalidStatusIndex(
            "expected integer expressed as a string".to_owned(),
          ))
        }
        Value::String(index) => u32::from_str(index).map_err(|_| {
          CredentialStatusError::InvalidStatusIndex(
            "expected integer greater or equal to zero and less than 2^32".to_owned(),
          )
        })?,
      }
    };

    Ok(Self {
      id,
      types,
      revocation_list_index,
    })
  }
}

impl TryFrom<EmbeddedRevocationStatus> for Status {
  type Error = CredentialStatusError;

  fn try_from(status: EmbeddedRevocationStatus) -> Result<Self> {
    let object: Object = Object::from([(
      EmbeddedRevocationList::credential_list_index_property().to_owned(),
      Value::String(status.revocation_list_index.to_string()),
    )]);
    let url = Url::parse(status.id.to_string())
      .map_err(|_| CredentialStatusError::InvalidStatusId(format!("{} cannot be parsed as a Url", status.id)))?;
    Ok(Status::with_properties(url, Some(status.types), object))
  }
}

#[cfg(test)]
mod tests {
  use identity_core::common::Object;
  use identity_core::common::Url;
  use identity_core::common::Value;
  use identity_credential::credential::Status;

  use super::EmbeddedRevocationStatus;
  use crate::did::IotaDIDUrl;
  use crate::revocation::EmbeddedRevocationList;

  const TAG: &str = "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV";
  const SERVICE: &str = "revocation";

  #[test]
  fn test_embedded_status_invariants() {
    let url: Url = Url::parse(format!("did:iota:{}#{}", TAG, SERVICE)).unwrap();
    let iota_did_url: IotaDIDUrl = IotaDIDUrl::parse(url.clone().into_string()).unwrap();
    let revocation_list_index: u32 = 0;
    let embedded_revocation_status = EmbeddedRevocationStatus::new(iota_did_url.clone(), revocation_list_index);

    let object: Object = Object::from([(
      EmbeddedRevocationList::credential_list_index_property().to_owned(),
      Value::String(revocation_list_index.to_string()),
    )]);
    let status: Status = Status::with_properties(
      url.clone(),
      Some(EmbeddedRevocationList::name().to_owned()),
      object.clone(),
    );
    assert_eq!(embedded_revocation_status, status.try_into().unwrap());

    let status_missing_property: Status = Status::with_properties(
      url.clone(),
      Some(EmbeddedRevocationList::name().to_owned()),
      Object::new(),
    );
    assert!(EmbeddedRevocationStatus::try_from(status_missing_property).is_err());

    let status_wrong_type: Status = Status::with_properties(url, Some("DifferentType".to_owned()), object);
    assert!(EmbeddedRevocationStatus::try_from(status_wrong_type).is_err());
  }
}
