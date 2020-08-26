use anyhow::Result;

use crate::{
  common::{Context, Object, OneOrMany, URI},
  verifiable::VerifiableCredential,
};

/// A `Presentation` represents a bundle of one or more `VerifiableCredential`s.
///
/// `Presentation`s can be combined with `Proof`s to create `VerifiablePresentation`s.
#[derive(Debug, Deserialize, Serialize)]
pub struct Presentation {
  /// A set of URIs or `Object`s describing the applicable JSON-LD contexts.
  ///
  /// NOTE: The first URI MUST be `https://www.w3.org/2018/credentials/v1`
  #[serde(rename = "@context")]
  pub context: OneOrMany<Context>,
  /// A unique `URI` referencing the subject of the presentation.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<URI>,
  /// One or more URIs defining the type of presentation.
  ///
  /// NOTE: The VC spec defines this as a set of URIs BUT they are commonly
  /// passed as non-`URI` strings and expected to be processed with JSON-LD.
  /// We're using a `String` here since we don't currently use JSON-LD and
  /// don't have any immediate plans to do so.
  #[serde(rename = "type")]
  pub types: OneOrMany<String>,
  /// TODO
  #[serde(rename = "verifiableCredential")]
  pub verifiable_credential: OneOrMany<VerifiableCredential>,
  /// The entity that generated the presentation.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub holder: Option<URI>,
  /// TODO
  #[serde(rename = "refreshService", skip_serializing_if = "Option::is_none")]
  pub refresh_service: Option<OneOrMany<Object>>,
  /// The terms of use issued by the presentation holder
  #[serde(rename = "termsOfUse", skip_serializing_if = "Option::is_none")]
  pub terms_of_use: Option<OneOrMany<Object>>,
  /// Miscellaneous properties.
  #[serde(flatten)]
  pub properties: Object,
}

impl Presentation {
  pub const BASE_TYPE: &'static str = "VerifiablePresentation";
}
