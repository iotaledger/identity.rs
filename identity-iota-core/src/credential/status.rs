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
use crate::revocation::EmbeddedRevocationList;

/// Information used to determine the current status of a [`Credential`][crate::credential::Credential].
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EmbeddedRevocationStatus {
  /// A DID URL that can be resolved to a service of the issuer which contains the revocation list.
  pub id: Url,
  /// The string indentifying the revocation method.
  #[serde(rename = "type")]
  pub types: String,
  /// The index of the revocation status of the verifiable credential.
  #[serde(rename = "revocationListIndex")]
  pub revocation_list_index: u32,
}

impl EmbeddedRevocationStatus {
  /// Creates a new `EmbeddedRevocationStatus`.
  pub fn new(id: Url, revocation_list_index: u32) -> Self {
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
      id: status.id,
      types,
      revocation_list_index,
    })
  }
}

impl From<EmbeddedRevocationStatus> for Status {
  fn from(embedded_status: EmbeddedRevocationStatus) -> Self {
    let object: Object = Object::from([(
      EmbeddedRevocationList::credential_list_index_property().to_owned(),
      Value::String(embedded_status.revocation_list_index.to_string()),
    )]);
    Status::with_properties(embedded_status.id, Some(embedded_status.types), object)
  }
}
