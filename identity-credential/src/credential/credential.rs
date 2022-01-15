// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Display;
use core::fmt::Formatter;

use serde::Serialize;

use identity_core::common::Context;
use identity_core::common::Object;
use identity_core::common::OneOrMany;
use identity_core::common::Timestamp;
use identity_core::common::Url;
use identity_core::convert::FmtJson;
use identity_core::crypto::SetSignature;
use identity_core::crypto::Signature;
use identity_core::crypto::TrySignature;
use identity_core::crypto::TrySignatureMut;
use identity_did::verification::MethodUriType;
use identity_did::verification::TryMethod;

use crate::credential::CredentialBuilder;
use crate::credential::Evidence;
use crate::credential::Issuer;
use crate::credential::Policy;
use crate::credential::Refresh;
use crate::credential::Schema;
use crate::credential::Status;
use crate::credential::Subject;
use crate::error::Error;
use crate::error::Result;

lazy_static! {
  static ref BASE_CONTEXT: Context = Context::Url(Url::parse("https://www.w3.org/2018/credentials/v1").unwrap());
}

/// Represents a set of claims describing an entity.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Credential<T = Object> {
  /// The JSON-LD context(s) applicable to the `Credential`.
  #[serde(rename = "@context")]
  pub context: OneOrMany<Context>,
  /// A unique `URI` referencing the subject of the `Credential`.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<Url>,
  /// One or more URIs defining the type of the `Credential`.
  #[serde(rename = "type")]
  pub types: OneOrMany<String>,
  /// One or more `Object`s representing the `Credential` subject(s).
  #[serde(rename = "credentialSubject")]
  pub credential_subject: OneOrMany<Subject>,
  /// A reference to the issuer of the `Credential`.
  pub issuer: Issuer,
  /// A timestamp of when the `Credential` becomes valid.
  #[serde(rename = "issuanceDate")]
  pub issuance_date: Timestamp,
  /// A timestamp of when the `Credential` should no longer be considered valid.
  #[serde(rename = "expirationDate", skip_serializing_if = "Option::is_none")]
  pub expiration_date: Option<Timestamp>,
  /// Information used to determine the current status of the `Credential`.
  #[serde(default, rename = "credentialStatus", skip_serializing_if = "OneOrMany::is_empty")]
  pub credential_status: OneOrMany<Status>,
  /// Information used to assist in the enforcement of a specific `Credential` structure.
  #[serde(default, rename = "credentialSchema", skip_serializing_if = "OneOrMany::is_empty")]
  pub credential_schema: OneOrMany<Schema>,
  /// Service(s) used to refresh an expired `Credential`.
  #[serde(default, rename = "refreshService", skip_serializing_if = "OneOrMany::is_empty")]
  pub refresh_service: OneOrMany<Refresh>,
  /// Terms-of-use specified by the `Credential` issuer.
  #[serde(default, rename = "termsOfUse", skip_serializing_if = "OneOrMany::is_empty")]
  pub terms_of_use: OneOrMany<Policy>,
  /// Human-readable evidence used to support the claims within the `Credential`.
  #[serde(default, skip_serializing_if = "OneOrMany::is_empty")]
  pub evidence: OneOrMany<Evidence>,
  /// Indicates that the `Credential` must only be contained within a
  /// [`Presentation`][crate::presentation::Presentation] with a proof issued from the `Credential` subject.
  #[serde(rename = "nonTransferable", skip_serializing_if = "Option::is_none")]
  pub non_transferable: Option<bool>,
  /// Miscellaneous properties.
  #[serde(flatten)]
  pub properties: T,
  /// Proof(s) used to verify a `Credential`
  #[serde(skip_serializing_if = "Option::is_none")]
  pub proof: Option<Signature>,
}

impl<T> Credential<T> {
  /// Returns the base JSON-LD context.
  pub fn base_context() -> &'static Context {
    &*BASE_CONTEXT
  }

  /// Returns the base type.
  pub const fn base_type() -> &'static str {
    "VerifiableCredential"
  }

  /// Creates a new `CredentialBuilder` to configure a `Credential`.
  ///
  /// This is the same as [CredentialBuilder::new].
  pub fn builder(properties: T) -> CredentialBuilder<T> {
    CredentialBuilder::new(properties)
  }

  /// Returns a new `Credential` based on the `CredentialBuilder` configuration.
  pub fn from_builder(builder: CredentialBuilder<T>) -> Result<Self> {
    let this: Self = Self {
      context: builder.context.into(),
      id: builder.id,
      types: builder.types.into(),
      credential_subject: builder.subject.into(),
      issuer: builder.issuer.ok_or(Error::MissingIssuer)?,
      issuance_date: builder.issuance_date.unwrap_or_default(),
      expiration_date: builder.expiration_date,
      credential_status: builder.status.into(),
      credential_schema: builder.schema.into(),
      refresh_service: builder.refresh.into(),
      terms_of_use: builder.policy.into(),
      evidence: builder.evidence.into(),
      non_transferable: builder.non_transferable,
      properties: builder.properties,
      proof: None,
    };

    this.check_structure()?;

    Ok(this)
  }

  /// Validates the semantic structure of the `Credential`.
  pub fn check_structure(&self) -> Result<()> {
    // Ensure the base context is present and in the correct location
    match self.context.get(0) {
      Some(context) if context == Self::base_context() => {}
      Some(_) | None => return Err(Error::MissingBaseContext),
    }

    // The set of types MUST contain the base type
    if !self.types.iter().any(|type_| type_ == Self::base_type()) {
      return Err(Error::MissingBaseType);
    }

    // Credentials MUST have at least one subject
    if self.credential_subject.is_empty() {
      return Err(Error::MissingSubject);
    }

    // Each subject is defined as one or more properties - no empty objects
    for subject in self.credential_subject.iter() {
      if subject.id.is_none() && subject.properties.is_empty() {
        return Err(Error::InvalidSubject);
      }
    }

    Ok(())
  }

  /// Returns a reference to the proof.
  pub fn proof(&self) -> Option<&Signature> {
    self.proof.as_ref()
  }

  /// Returns a mutable reference to the proof.
  pub fn proof_mut(&mut self) -> Option<&mut Signature> {
    self.proof.as_mut()
  }

  /// Checks whether this Credential expires after the given `Timestamp`.
  /// If the Credential does not have an expiration date (expiresAfter/expires_after) then true will be returned.
  pub fn expires_after(&self, timestamp: Timestamp) -> bool {
    if let Some(expiration_date) = self.expiration_date {
      expiration_date > timestamp
    } else {
      true
    }
  }

  /// Checks whether the issuance date of this Credential is before the given `Timestamp`
  pub fn issued_before(&self, timestamp: Timestamp) -> bool {
    self.issuance_date < timestamp // todo: would <= be better than < ?
  }
}

impl<T> Display for Credential<T>
where
  T: Serialize,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    self.fmt_json(f)
  }
}

impl<T> TrySignature for Credential<T> {
  fn signature(&self) -> Option<&Signature> {
    self.proof.as_ref()
  }
}

impl<T> TrySignatureMut for Credential<T> {
  fn signature_mut(&mut self) -> Option<&mut Signature> {
    self.proof.as_mut()
  }
}

impl<T> SetSignature for Credential<T> {
  fn set_signature(&mut self, value: Signature) {
    self.proof.replace(value);
  }
}

impl<T> TryMethod for Credential<T> {
  const TYPE: MethodUriType = MethodUriType::Absolute;
}

#[cfg(test)]
mod tests {
  use identity_core::convert::FromJson;

  use super::*;
  use proptest::proptest;

  const JSON1: &str = include_str!("../../tests/fixtures/credential-1.json");
  const JSON2: &str = include_str!("../../tests/fixtures/credential-2.json");
  const JSON3: &str = include_str!("../../tests/fixtures/credential-3.json");
  const JSON4: &str = include_str!("../../tests/fixtures/credential-4.json");
  const JSON5: &str = include_str!("../../tests/fixtures/credential-5.json");
  const JSON6: &str = include_str!("../../tests/fixtures/credential-6.json");
  const JSON7: &str = include_str!("../../tests/fixtures/credential-7.json");
  const JSON8: &str = include_str!("../../tests/fixtures/credential-8.json");
  const JSON9: &str = include_str!("../../tests/fixtures/credential-9.json");
  const JSON10: &str = include_str!("../../tests/fixtures/credential-10.json");
  const JSON11: &str = include_str!("../../tests/fixtures/credential-11.json");
  const JSON12: &str = include_str!("../../tests/fixtures/credential-12.json");

  const LAST_RFC3339_COMPATIBLE_UNIX_TIMESTAMP: i64 = 253402300799; // 9999-12-31T23:59:59Z
  const FIRST_RFC3999_COMPATIBLE_UNIX_TIMESTAMP: i64 = -62167219200; // 0000-01-01T00:00:00Z

  fn deserialize_credential(credential_str: &str) -> Credential {
    Credential::from_json(credential_str).unwrap()
  }

  #[test]
  fn test_from_json() {
    let credentials = [
      JSON1, JSON2, JSON3, JSON4, JSON5, JSON6, JSON7, JSON8, JSON9, JSON10, JSON11, JSON12,
    ];
    for credential_str in credentials {
      let _ = deserialize_credential(credential_str);
    }
  }

  #[test]
  fn simple_expires_after_with_expiration_date() {
    let credential = deserialize_credential(JSON6);
    let expected_expiration_date = Timestamp::parse("2020-01-01T19:23:24Z").unwrap();
    // check that this credential has the expected expiration date
    assert_eq!(
      credential.expiration_date.unwrap(),
      expected_expiration_date,
      "the expiration date of the parsed credential does not match our expectation"
    );
    // now that we are sure that our parsed credential has the expected expiration date set we can start testing the
    // expires_after method with a later date
    let later_date = Timestamp::parse("2020-02-01T15:10:21Z").unwrap();
    assert!(!credential.expires_after(later_date));
    // and now with an earlier date
    let earlier_date = Timestamp::parse("2019-12-27T11:35:30Z").unwrap();
    assert!(credential.expires_after(earlier_date));
  }

  // test with a few timestamps that should be RFC3339 compatible
  proptest! {
    #[test]
    fn property_based_expires_after_with_expiration_date(seconds in 0..1000_000_000i64) {
      let credential = deserialize_credential(JSON6);
      let expected_expiration_date = Timestamp::parse("2020-01-01T19:23:24Z").unwrap();
      // check that this credential has the expected expiration date
      assert_eq!(credential.expiration_date.unwrap(), expected_expiration_date, "the expiration date of the parsed credential does not match our expectation");
      let after_expiration_date = Timestamp::from_unix(expected_expiration_date.to_unix() + seconds).unwrap();
      let before_expiration_date = Timestamp::from_unix(expected_expiration_date.to_unix() - seconds).unwrap();
      assert!(!credential.expires_after(after_expiration_date));
      assert!(credential.expires_after(before_expiration_date));
    }
  }

  proptest! {
    #[test]
    fn property_based_expires_after_no_expiration_date(seconds in FIRST_RFC3999_COMPATIBLE_UNIX_TIMESTAMP..LAST_RFC3339_COMPATIBLE_UNIX_TIMESTAMP) {
      let credential = deserialize_credential(JSON1);
      // check that this credential does not have a timestamp as per our expectation
      assert!(
        credential.expiration_date.is_none(),
        "The credential had an expiration date contrary to our expectation"
      );
      // expires after whatever the timestamp may be because the expires_after field is None.
      assert!(credential.expires_after(Timestamp::from_unix(seconds).unwrap()));
    }
  }

  #[test]
  fn simple_issued_before() {
    let credential = deserialize_credential(JSON1);
    let expected_issuance_date = Timestamp::parse("2010-01-01T19:23:24Z").unwrap();
    // check that this credential has the expected issuance date
    assert_eq!(
      credential.issuance_date, expected_issuance_date,
      "the issuance date of the parsed credential does not match our expectation"
    );
    // now that we are sure that our parsed credential has the expected issuance date set we can start testing issued
    // before with an earlier timestamp
    assert!(!credential.issued_before(Timestamp::parse("2010-01-01T19:22:09Z").unwrap()));
    // and now with a later timestamp
    assert!(credential.issued_before(Timestamp::parse("2010-01-01T20:00:00Z").unwrap()));
  }

  proptest! {
    #[test]
    fn property_based_issued_before(seconds in 0 ..1000_000_000i64) {
      let credential = deserialize_credential(JSON1);
      let expected_issuance_date = Timestamp::parse("2010-01-01T19:23:24Z").unwrap();
      // check that this credential has the expected issuance date
      assert_eq!(credential.issuance_date, expected_issuance_date, "the issuance date of the parsed credential does not match our expectation");
      let earlier_than_issuance_date = Timestamp::from_unix(expected_issuance_date.to_unix() - seconds).unwrap();
      let later_than_issuance_date = Timestamp::from_unix(expected_issuance_date.to_unix() + seconds).unwrap();
      assert!(!credential.issued_before(earlier_than_issuance_date));
      assert!(credential.issued_before(later_than_issuance_date));
    }
  }
}
