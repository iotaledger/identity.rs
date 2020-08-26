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

  pub fn validate(&self) -> Result<()> {
    validate_context(&self.context)?;

    if let Some(ref id) = self.id {
      validate_uri(id)?;
    }

    validate_types(&self.types, Self::BASE_TYPE)?;
    validate_subject(&self.credential_subject)?;
    validate_uri(self.issuer.uri())?;
    validate_timestamp("issuance date", &self.issuance_date)?;

    if let Some(ref timestamp) = self.expiration_date {
      validate_timestamp("expiration date", timestamp)?;
    }

    Ok(())
  }
}

pub fn validate_context(context: &OneOrMany<Context>) -> Result<()> {
  // Credentials/Presentations MUST have at least one context item
  ensure!(!context.is_empty(), "Not enough context items");

  // The first item MUST be a URI with the value of the base context
  match context.get(0) {
    Some(Context::URI(uri)) if uri == Credential::BASE_CONTEXT => Ok(()),
    Some(_) => bail!("Invalid base context"),
    None => unreachable!(),
  }
}

pub fn validate_types(types: &OneOrMany<String>, base: &'static str) -> Result<()> {
  // Credentials/Presentations MUST have at least one type
  ensure!(!types.is_empty(), "Not enough types");

  // The set of types MUST contain the base type
  ensure!(types.contains(&base.into()), "Missing base type");

  Ok(())
}

pub fn validate_subject(subjects: &OneOrMany<Object>) -> Result<()> {
  // A credential MUST have at least one subject
  ensure!(!subjects.is_empty(), "Not enough subjects");

  // Each subject is defined as one or more properties - no empty objects
  for subject in subjects.iter() {
    ensure!(!subject.is_empty(), "Invalid credential subject (empty)");
  }

  Ok(())
}

pub fn validate_uri(uri: &URI) -> Result<()> {
  const KNOWN: [&str; 4] = ["did:", "urn:", "http:", "https:"];

  // TODO: Proper URI validation
  ensure!(
    KNOWN.iter().any(|scheme| uri.starts_with(scheme)),
    "Invalid URI `{}`",
    uri.as_str(),
  );

  Ok(())
}

// Validate the timestamp format according to RFC 3339
//
// Ref: https://tools.ietf.org/html/rfc3339
pub fn validate_timestamp(name: &'static str, timestamp: &str) -> Result<()> {
  ensure!(!timestamp.is_empty(), "Invalid {} (empty)", name);

  match DateTime::parse_from_rfc3339(timestamp) {
    Ok(_) => Ok(()),
    Err(error) => bail!("Invalid {} ({})", name, error),
  }
}
