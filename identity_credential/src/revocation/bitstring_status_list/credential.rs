use std::fmt::Display;
use std::ops::Deref;
use std::ops::DerefMut;
use std::str::FromStr;

use identity_core::common::Context;
use identity_core::common::OneOrMany;
use identity_core::common::Timestamp;
use identity_core::common::Url;
use serde::Deserialize;
use serde::Serialize;
use serde_with::serde_as;
use serde_with::skip_serializing_none;
use serde_with::DisplayFromStr;
use thiserror::Error;

use crate::credential::Credential;
use crate::credential::CredentialBuilder;
use crate::credential::Issuer;
use crate::credential::Subject;

use super::BitstringStatusList;
use super::StatusMessage;

const CREDENTIAL_TYPE: &str = "BitstringStatusListCredential";
const CREDENTIAL_SUBJECT_TYPE: &str = "BitstringStatusList";

#[derive(Clone, Debug, Default, Eq, PartialEq)]
/// [`BitstringStatusList`]'s status purpose
pub enum StatusPurpose {
  #[default]
  /// This [`BitstringStatusList`] is comprised of binary entries, where the `set` states represents a suspended credential
  Suspension,
  /// This [`BitstringStatusList`] is comprised of binary entries, where the `set` states represents a revoked credential
  Revocation,
  /// This [`BitstringStatusList`] uses custom statuses
  Status,
  /// Arbitrary purpose
  Other(String),
}

impl FromStr for StatusPurpose {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(match s {
      "suspension" => Self::Suspension,
      "revocation" => Self::Revocation,
      "status" => Self::Status,
      _ => Self::Other(s.to_owned()),
    })
  }
}

impl Display for StatusPurpose {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let s = match self {
      Self::Revocation => "revocation",
      Self::Suspension => "suspension",
      Self::Status => "status",
      Self::Other(v) => v.as_str(),
    };
    write!(f, "{s}")
  }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
struct BitstringStatusListCredentialSubject {
  id: Option<Url>,
  #[serde(rename = "type")]
  r#type: String,
  #[serde_as(as = "DisplayFromStr")]
  status_purpose: StatusPurpose,
  encoded_list: String,
  ttl: Option<u64>,
  size: Option<usize>,
  status_messages: Option<Vec<StatusMessage>>,
  reference: Option<OneOrMany<Url>>,
}

impl From<BitstringStatusList> for BitstringStatusListCredentialSubject {
  fn from(value: BitstringStatusList) -> Self {
    let encoded_list = value.as_encoded_str().expect("Out of memory");
    let has_statuses = value.statuses.len() > 0;
    let status_purpose = if has_statuses {
      StatusPurpose::Status
    } else {
      StatusPurpose::default()
    };
    let size = has_statuses.then_some(value.entry_len());
    let status_messages = has_statuses.then_some(value.statuses.into_vec());

    BitstringStatusListCredentialSubject {
      id: None,
      r#type: CREDENTIAL_SUBJECT_TYPE.to_owned(),
      status_purpose,
      encoded_list,
      ttl: None,
      size,
      status_messages,
      reference: None,
    }
  }
}

impl From<BitstringStatusListCredentialSubject> for Subject {
  fn from(mut value: BitstringStatusListCredentialSubject) -> Self {
    use serde_json::Value;
    use std::collections::BTreeMap;

    let id = value.id.take();
    let properties = {
      let Value::Object(obj) = serde_json::to_value(value).expect("Failed to serialize credential subject") else {
        unreachable!("This is indeed a JSON object");
      };
      obj.into_iter().collect::<BTreeMap<_, _>>()
    };

    Subject { id, properties }
  }
}

#[derive(Debug, Error)]
#[error("{0} is not a valid BitstringStatusListCredential")]
/// The [`std::error::Error`] created when an invalid credential is presented as a [`BitstringStatusListCredential`]
pub struct InvalidCredentialError(Credential);

#[derive(Debug)]
/// Implementation of [BitstringStatusListCredential](https://www.w3.org/TR/vc-bitstring-status-list/#bitstring-generation-algorithm)
pub struct BitstringStatusListCredential {
  credential: Credential,
  parsed_credential_subject: BitstringStatusListCredentialSubject,
}

impl Deref for BitstringStatusListCredential {
  type Target = Credential;
  fn deref(&self) -> &Self::Target {
    &self.credential
  }
}

impl DerefMut for BitstringStatusListCredential {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.credential
  }
}

impl TryFrom<Credential> for BitstringStatusListCredential {
  type Error = InvalidCredentialError;
  fn try_from(value: Credential) -> Result<Self, Self::Error> {
    // Todo: make this thing less horrible
    let has_right_credential_type = value.types.contains(&CREDENTIAL_TYPE.to_owned());
    let subject = {
      let OneOrMany::One(subject) = &value.credential_subject else {
        return Err(InvalidCredentialError(value));
      };
      let subject_id = &subject.id;
      let subject = serde_json::Value::Object(serde_json::Map::from_iter(subject.properties.clone()));
      let mut bitstring_subject = serde_json::from_value::<BitstringStatusListCredentialSubject>(subject)
        .map_err(|e| {dbg!(e); InvalidCredentialError(value.clone())})?;
      bitstring_subject.id = subject_id.clone();
      bitstring_subject
    };
    dbg!(&subject);
    let has_matching_size_and_purpose = subject.size.is_none()
      && matches!(
        subject.status_purpose,
        StatusPurpose::Revocation | StatusPurpose::Suspension
      )
      && subject.status_messages.is_none()
      || subject.size.is_some_and(|s| s > 1)
        && matches!(subject.status_purpose, StatusPurpose::Status | StatusPurpose::Other(_))
        && subject
          .status_messages
          .as_deref()
          .is_some_and(|msgs| super::utils::bit_required(msgs.len()) == *subject.size.as_ref().unwrap());
    let has_right_credential_subject_type = subject.r#type.as_str() == CREDENTIAL_SUBJECT_TYPE;

    if has_right_credential_type && has_matching_size_and_purpose && has_right_credential_subject_type {
      Ok(BitstringStatusListCredential {
        credential: value,
        parsed_credential_subject: subject,
      })
    } else {
      Err(InvalidCredentialError(value))
    }
  }
}

impl BitstringStatusListCredential {
  /// Downcasts this [`BitstringStatusListCredential`] to a simple [`Credential`]
  pub fn into_inner(self) -> Credential {
    self.credential
  }
  /// Extracts the [`BitstringStatusList`] encoded in this credential
  pub fn as_bitstring_status_list(&self) -> super::Result<'_, BitstringStatusList> {
    let encoded_list = self.parsed_credential_subject.encoded_list.as_str();
    let statuses = self.parsed_credential_subject.status_messages.clone();
    BitstringStatusList::try_from_encoded_str(encoded_list, statuses.unwrap_or_default())
  }
}

#[derive(Debug)]
/// Builder for a [`BitstringStatusListCredential`]
pub struct BitstringStatusListCredentialBuilder {
  inner_builder: CredentialBuilder,
  subject: BitstringStatusListCredentialSubject,
}

impl BitstringStatusListCredentialBuilder {
  /// Seed this builder with [`BitstringStatusList`] `status_list`
  pub fn new(status_list: BitstringStatusList) -> Self {
    use std::collections::BTreeMap;

    let subject = status_list.into();
    let inner_builder = CredentialBuilder::new(BTreeMap::new())
      .type_(CREDENTIAL_TYPE)
      .issuance_date(Timestamp::now_utc());

    Self { inner_builder, subject }
  }
  /// Adds a value to the `Credential` context set.
  pub fn context(mut self, value: impl Into<Context>) -> Self {
    self.inner_builder.context.push(value.into());
    self
  }
  /// Sets the value of the `Credential` `id`.
  #[must_use]
  pub fn id(mut self, value: Url) -> Self {
    self.inner_builder.id = Some(value);
    self
  }
  /// Sets the value of the `Credential` `issuer`.
  pub fn issuer(mut self, value: impl Into<Issuer>) -> Self {
    self.inner_builder.issuer = Some(value.into());
    self
  }
  /// Sets the value of the `Credential` `issuanceDate`.
  pub fn issuance_date(mut self, value: Timestamp) -> Self {
    self.inner_builder.issuance_date = Some(value);
    self
  }
  /// Sets the value of the `Credential` `expirationDate`.
  pub fn expiration_date(mut self, value: Timestamp) -> Self {
    self.inner_builder.expiration_date = Some(value);
    self
  }

  /// Sets the value of the `Credential`'s `credentialSubject.id`
  pub fn subject_id(mut self, id: impl Into<Url>) -> Self {
    self.subject.id = Some(id.into());
    self
  }

  /// Sets the value of the `Credential`'s `credentialSubject.ttl`
  pub fn subject_ttl(mut self, ttl: u64) -> Self {
    self.subject.ttl = Some(ttl);
    self
  }

  /// Sets the value of the `Credential`'s `credentialSubject.statusPurpose`
  pub fn subject_status_purpose(mut self, purpose: StatusPurpose) -> Option<Self> {
    if self.subject.status_messages.is_some()
      && matches!(&purpose, StatusPurpose::Revocation | StatusPurpose::Suspension)
      || self.subject.status_messages.is_none() && matches!(&purpose, StatusPurpose::Other(_) | StatusPurpose::Status)
    {
      return None;
    }

    self.subject.status_purpose = purpose;
    Some(self)
  }

  /// Attempts to build a new [`BitstringStatusListCredential`] based on the provided configuration.
  pub fn build(mut self) -> Result<BitstringStatusListCredential, crate::Error> {
    let subject = self.subject.clone().into();
    self.inner_builder.subject.push(subject);
    self
      .inner_builder
      .build()
      .map(|credential| BitstringStatusListCredential {
        credential,
        parsed_credential_subject: self.subject,
      })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const VALID_BITSTRING_STRATUS_LIST_CREDENTIAL: &'static str = r#"
{
  "@context": [
    "https://www.w3.org/ns/credentials/v1",
    "https://www.w3.org/ns/credentials/examples/v1"
  ],
  "id": "https://example.com/credentials/status/3",
  "type": ["VerifiableCredential", "BitstringStatusListCredential"],
  "issuer": "did:example:12345",
  "issuanceDate": "2021-04-05T14:27:40Z",
  "credentialSubject": {
    "id": "https://example.com/status/3#list",
    "type": "BitstringStatusList",
    "statusPurpose": "revocation",
    "encodedList": "H4sIAAAAAAAAA-3BMQEAAADCoPVPbQwfoAAAAAAAAAAAAAAAAAAAAIC3AYbSVKsAQAAA"
  }
}"#;

  #[test]
  fn deserialize_valid_bitstring_status_list_credential_works() {
    let credential: Credential = serde_json::from_str(VALID_BITSTRING_STRATUS_LIST_CREDENTIAL).unwrap();
    let bitstring_credential = BitstringStatusListCredential::try_from(credential);
    assert!(bitstring_credential.is_ok());
  }
}