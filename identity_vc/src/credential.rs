use anyhow::Result;
use chrono::DateTime;

use crate::common::{Context, Issuer, Object, OneOrMany, URI};

/// A `Credential` represents a set of claims describing an entity.
///
/// `Credential`s can be combined with `Proof`s to create `VerifiableCredential`s.
#[derive(Debug, Deserialize, Serialize)]
pub struct Credential {
  /// A set of URIs or `Object`s describing the applicable JSON-LD contexts.
  ///
  /// NOTE: The first URI MUST be `https://www.w3.org/2018/credentials/v1`
  #[serde(rename = "@context")]
  pub context: OneOrMany<Context>,
  /// A unique `URI` referencing the subject of the credential.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<URI>,
  /// One or more URIs defining the type of credential.
  ///
  /// NOTE: The VC spec defines this as a set of URIs BUT they are commonly
  /// passed as non-`URI` strings and expected to be processed with JSON-LD.
  /// We're using a `String` here since we don't currently use JSON-LD and
  /// don't have any immediate plans to do so.
  #[serde(rename = "type")]
  pub types: OneOrMany<String>,
  /// One or more `Object`s representing the credential subject(s).
  #[serde(rename = "credentialSubject")]
  pub credential_subject: OneOrMany<Object>,
  /// A reference to the issuer of the credential.
  pub issuer: Issuer,
  /// The date and time the credential becomes valid.
  #[serde(rename = "issuanceDate")]
  pub issuance_date: String,
  /// The date and time the credential is no longer considered valid.
  #[serde(rename = "expirationDate", skip_serializing_if = "Option::is_none")]
  pub expiration_date: Option<String>,
  /// TODO
  #[serde(rename = "credentialStatus", skip_serializing_if = "Option::is_none")]
  pub credential_status: Option<OneOrMany<Object>>,
  /// TODO
  #[serde(rename = "credentialSchema", skip_serializing_if = "Option::is_none")]
  pub credential_schema: Option<OneOrMany<Object>>,
  /// TODO
  #[serde(rename = "refreshService", skip_serializing_if = "Option::is_none")]
  pub refresh_service: Option<OneOrMany<Object>>,
  /// The terms of use issued by the credential issuer
  #[serde(rename = "termsOfUse", skip_serializing_if = "Option::is_none")]
  pub terms_of_use: Option<OneOrMany<Object>>,
  /// TODO
  #[serde(skip_serializing_if = "Option::is_none")]
  pub evidence: Option<OneOrMany<Object>>,
  /// Miscellaneous properties.
  #[serde(flatten)]
  pub properties: Object,
}

impl Credential {
  pub const BASE_CONTEXT: &'static str = "https://www.w3.org/2018/credentials/v1";

  pub const BASE_TYPE: &'static str = "VerifiableCredential";
}
