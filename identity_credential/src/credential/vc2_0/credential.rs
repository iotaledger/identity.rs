use identity_core::common::Context;
use identity_core::common::Object;
use identity_core::common::OneOrMany;
use identity_core::common::Timestamp;
use identity_core::common::Url;
use once_cell::sync::Lazy;
use serde::Deserialize;
use serde::Deserializer;
use serde::de::Error as _;
use serde::Serialize;

use crate::credential::Evidence;
use crate::credential::Issuer;
use crate::credential::JwtCredentialClaims;
use crate::credential::Policy;
use crate::credential::Proof;
use crate::credential::RefreshService;
use crate::credential::Schema;
use crate::credential::Status;
use crate::credential::Subject;
use crate::Error;

static BASE_CONTEXT: Lazy<Context> =
  Lazy::new(|| Context::Url(Url::parse("https://www.w3.org/ns/credentials/v2").unwrap()));

fn deserialize_vc2_0_context<'de, D>(deserializer: D) -> Result<OneOrMany<Context>, D::Error>
where
  D: Deserializer<'de>,
{
  let ctx = OneOrMany::<Context>::deserialize(deserializer)?;
  if ctx.contains(&BASE_CONTEXT) {
    Ok(ctx)
  } else {
    Err(D::Error::custom("Missing base context"))
  }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Credential<T = Object> {
  /// The JSON-LD context(s) applicable to the `Credential`.
  #[serde(rename = "@context", deserialize_with = "deserialize_vc2_0_context")]
  pub context: OneOrMany<Context>,
  /// A unique `URI` that may be used to identify the `Credential`.
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
  #[serde(rename = "validFrom")]
  pub valid_from: Timestamp,
  /// A timestamp of when the `Credential` should no longer be considered valid.
  #[serde(rename = "validUntil", skip_serializing_if = "Option::is_none")]
  pub valid_until: Option<Timestamp>,
  /// Information used to determine the current status of the `Credential`.
  #[serde(default, rename = "credentialStatus", skip_serializing_if = "Option::is_none")]
  pub credential_status: Option<Status>,
  /// Information used to assist in the enforcement of a specific `Credential` structure.
  #[serde(default, rename = "credentialSchema", skip_serializing_if = "OneOrMany::is_empty")]
  pub credential_schema: OneOrMany<Schema>,
  /// Service(s) used to refresh an expired `Credential`.
  #[serde(default, rename = "refreshService", skip_serializing_if = "OneOrMany::is_empty")]
  pub refresh_service: OneOrMany<RefreshService>,
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
  /// Optional cryptographic proof, unrelated to JWT.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub proof: Option<Proof>,
}

impl<'a> TryFrom<&'a JwtCredentialClaims> for Credential {
  type Error = Error;
  fn try_from(value: &'a JwtCredentialClaims) -> std::result::Result<Self, Self::Error> {
    let JwtCredentialClaims {
      exp,
      iss,
      jti,
      sub,
      vc,
      custom,
      ..
    } = value;
    let mut vc = vc.clone();
    vc.insert("issuer".to_owned(), serde_json::Value::String(iss.url().to_string()));
    if let Some(jti) = jti {
      vc.insert("id".to_owned(), serde_json::Value::String(jti.to_string()));
    }
    vc.insert(
      "validFrom".to_owned(),
      serde_json::Value::String(value.issuance_date().to_string()),
    );
    if let Some(exp) = exp.as_deref() {
      vc.insert("validUntil".to_owned(), serde_json::Value::String(exp.to_string()));
    }
    if let Some(sub) = sub {
      vc.entry("credentialSubject".to_owned())
        .or_insert(serde_json::json!({}))
        .as_object_mut()
        .unwrap()
        .insert("id".to_owned(), serde_json::Value::String(sub.to_string()));
    }
    if let Some(custom) = custom {
      for (key, value) in custom {
        vc.insert(key.clone(), value.clone());
      }
    }
    let vc = serde_json::to_value(vc).expect("out of memory");
    serde_json::from_value(vc).map_err(|e| Error::JwtClaimsSetDeserializationError(Box::new(e)))
  }
}